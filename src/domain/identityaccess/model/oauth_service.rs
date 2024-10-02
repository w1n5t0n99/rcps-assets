use std::future::Future;

use oauth2::{url::Url, CsrfToken};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::credentials::OauthCredentials;

#[derive(Error, Debug)]
pub enum OAuthError {
    #[error("CSRF state does not match")]
    CsrfMismatch,
    #[error("HTTP request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("OAuth token error: {0}")]
    Token(anyhow::Error),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait OAuthService: Send + Sync + Clone + 'static {
    type UserProfile: std::fmt::Debug  + Clone + Send + Sync + Serialize + for<'de> Deserialize<'de>;

    fn authenticate(
        &self,
        creds: OauthCredentials,
    ) -> impl Future<Output = Result<Self::UserProfile, OAuthError>> + Send;

    fn client_id(&self) -> String;

    fn authorize_url(&self) -> (Url, CsrfToken);
}