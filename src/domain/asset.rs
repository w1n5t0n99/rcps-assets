use validator::{ValidationError, Validate};
use unicode_segmentation::UnicodeSegmentation;


#[derive(Debug, Validate)]
pub struct Asset {
    pub asset_id: String,
    #[validate(custom = "validate_name")]
    pub name: String,
    pub serial_num: String,
    pub model: String,
    pub brand: String,
}

fn validate_name(s: &str) -> Result<(), ValidationError> {
    let is_empty_or_whitespace = s.trim().is_empty();
    
    let is_too_long = s.graphemes(true).count() > 256;

    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

    if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
        return Err(ValidationError::new("Invalid asset name."));
    }

    Ok(())
}