use password_auth::{generate_hash, verify_password};
use thiserror::Error;
use tokio::task::{self, JoinError};

use super::{credentials::PasswordCredentials};


#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Password hash parsing errors")]
    Parse,
    #[error(transparent)]
    Task(#[from] JoinError),
}

#[derive(Debug, Clone)]
pub struct PasswordService {

}

impl PasswordService {
    pub fn new() -> Self {
        PasswordService { }
    }

    pub async fn generate_password(&self, password: String) -> Result<String, PasswordError> {
        let password_hash = task::spawn_blocking(move || {
            generate_hash(password)
        })
        .await?;

        Ok(password_hash)
    }

    pub async fn authenticate(&self, creds: PasswordCredentials, password_hash: String) -> Result<bool, PasswordError> {

        task::spawn_blocking(move || {
            match verify_password(creds.password, &password_hash) {
                Ok(_) => Ok(true),
                Err(e) => {
                    match e {
                        password_auth::VerifyError::Parse(_) => Err(PasswordError::Parse),
                        password_auth::VerifyError::PasswordInvalid => Ok(false),
                    }
                }
            }
        })
        .await?
    }
}