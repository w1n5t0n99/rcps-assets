use validator::{ValidationError, Validate};
use unicode_segmentation::UnicodeSegmentation;


#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct PartialAsset {
    pub sid: i32,
    pub asset_id: String,
    pub name: String,
    pub serial_num: String,
}

#[derive(Debug, PartialEq, Validate, serde::Deserialize, sqlx::FromRow)]
pub struct Asset {
    #[serde(default)]
    pub sid: i32,
    #[validate(custom = "custom_validate")]
    pub asset_id: String,
    #[validate(custom = "custom_validate")]
    pub name: String,
    pub serial_num: String,
    pub model: String,
    pub brand: String,
}

fn custom_validate(s: &str) -> Result<(), ValidationError> {
    let is_empty_or_whitespace = s.trim().is_empty();
    
    let is_too_long = s.graphemes(true).count() > 256;

    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

    if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
        return Err(ValidationError::new("Invalid asset name."));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use claim::{assert_err, assert_ok};

    fn get_test_asset(name: String) -> Asset {
        Asset {
            sid: 0,
            asset_id: "1156973".to_string(),
            name: name,
            serial_num: "".to_string(),
            model: "".to_string(),
            brand: "".to_string(),
        }
    }

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let asset = get_test_asset("a".repeat(256));
        assert_ok!(asset.validate());
    }

    #[test]
    fn a_name_longer_than_256_graphemes_is_rejected() {
        let asset = get_test_asset("a".repeat(257));
        assert_err!(asset.validate());
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let asset = get_test_asset(" ".to_string());
        assert_err!(asset.validate());
    }

    #[test]
    fn empty_string_is_rejected() {
        let asset = get_test_asset("".to_string());
        assert_err!(asset.validate());
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let asset = get_test_asset(name.to_string());
            assert_err!(asset.validate());
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let asset = get_test_asset("Ursula Le Guin".to_string());
        assert_ok!(asset.validate());
    }

}