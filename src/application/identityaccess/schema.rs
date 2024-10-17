use oauth2::CsrfToken;
use serde::{Deserialize};
use garde::Validate;


#[derive(Clone, Debug, Deserialize)]
#[derive(Validate)]
pub struct NewUserSchema {
    #[garde(email)]
    pub email: String,
    #[garde(length(min=8))]
    pub password: String,
    #[garde(matches(password))]	
    pub confirm_password: String,
    #[garde(length(min=1))]
    pub given_name: String,
    #[garde(length(min=1))]
    pub family_name: String,
    #[garde(skip)]	
    pub role_id: i32,
    #[garde(skip)]	
    #[serde(default = "default_picture")]
    pub picture: String,
}

fn default_picture() -> String {
    "/static/images/User.svg".to_string()
}

#[derive(Clone, Debug, Deserialize)]
#[derive(Validate)]
pub struct UpdateUserSchema {
    #[garde(length(min=1))]
    pub given_name: String,
    #[garde(length(min=1))]
    pub family_name: String,
    #[garde(skip)]	
    pub role_id: i32,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum OauthSchema {
    Success { code: String, state: CsrfToken },
    Error { error: String, state: CsrfToken },
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthSchema {
    pub email: String,
    pub password: String,
}

