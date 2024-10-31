use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use compact_str::CompactString;
use derive_more::derive::{Display, AsRef};


#[derive(Clone, Debug, Display, Serialize, Deserialize, AsRef, sqlx::Type)]
#[as_ref(str, [u8], String)]
pub struct EmailAddress(String);

impl EmailAddress {
    pub fn new(raw_value: impl Into<String>) -> Self {
        Self(raw_value.into())
    }
}

#[derive(Clone, Debug, Display, Serialize, Deserialize, AsRef, sqlx::Type)]
#[as_ref(str, [u8], String)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(raw_value: impl Into<String>) -> Self {
        Self(raw_value.into())
    }
}

#[derive(Clone, Debug, Display, Serialize, Deserialize, AsRef, sqlx::Type)]
#[as_ref(str, [u8], String)]
pub struct Picture(String);

impl Picture {
    pub fn new(raw_value: impl Into<String>) -> Self {
        Self(raw_value.into())
    }
}

impl Default for Picture {
    fn default() -> Self {
        Self(Default::default())
    }
}


#[derive(Clone, Copy, Debug, Display, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "provider", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    EmailPassword,
    Google,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewUser {
    pub password_hash: PasswordHash,  //hashed user password or unique identifier from oauth provider
    pub email: EmailAddress,
    pub email_verified: bool,
    pub given_name: String,
    pub family_name: String,
    pub role_id: i32,
    pub picture: Picture,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpdateUser {
    pub given_name: String,
    pub family_name: String,
    pub role_id: i32,
    pub picture: Option<Picture>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct User {
    pub id: uuid::Uuid,
    pub password_hash: PasswordHash, 
    pub email: EmailAddress,
    pub email_verified: bool,
    pub given_name: String,
    pub family_name: String,
    pub role: String,
    pub picture: Picture,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow, sqlx::Type)]
pub struct UserDescriptor {
    pub id: uuid::Uuid,
    pub email: EmailAddress,
    pub given_name: String,
    pub family_name: String,
    pub role: String,
    pub picture: Picture,
}

impl From<User> for UserDescriptor {
    fn from(value: User) -> Self {
        UserDescriptor {
            id: value.id,
            email: value.email,
            family_name: value.family_name,
            given_name: value.given_name,
            role: value.role,
            picture: value.picture,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionUser {
    pub user: UserDescriptor,
}

impl From<UserDescriptor> for SessionUser {
    fn from(value: UserDescriptor) -> Self {
        Self{user: value}
    }
}

impl From<User> for SessionUser {
    fn from(value: User) -> Self {
        Self{user: value.into()}
    }
}









