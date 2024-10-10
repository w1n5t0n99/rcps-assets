use serde::{Deserialize, Serialize};



#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUserSchema {
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    pub given_name: String,
    pub family_name: String,
    pub role_id: i32,
    pub picture: String,
}