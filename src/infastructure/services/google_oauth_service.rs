use anyhow::{anyhow, Context};
use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl};
use serde::{Deserialize, Serialize};

use crate::{domain::identityaccess::model::{credentials::OauthCredentials, oauth_service::{OAuthError, OAuthService}}, settings::GoogleConfig};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoogleUserProfile {
    pub sub: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email: String,
    pub email_verified: bool,
    pub hd: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GoogleOauthService {
    client: BasicClient,
    http_client: reqwest::Client,

}

impl GoogleOauthService {
    pub fn new(config: &GoogleConfig) -> anyhow::Result<Self> {
        let client_id = ClientId::new(config.client_id.clone());
        let client_secret = ClientSecret::new(config.client_secret.clone());
        let auth_url = AuthUrl::new(config.auth_url.clone()).context("error parsing auth url")?;
        let token_url = TokenUrl::new(config.token_url.clone()).context("error parsing token url")?;
        let redirect_url = RedirectUrl::new(config.redirect_url.clone())?;

        let client = BasicClient::new(
            client_id,
            Some(client_secret),
            auth_url,
            Some(token_url)
        )
        .set_redirect_uri(redirect_url);

        let http_client = reqwest::Client::new();

        Ok(Self{client, http_client})
    }
}

impl OAuthService for GoogleOauthService {

    type UserProfile = GoogleUserProfile;

    
    async fn authenticate(&self, creds: OauthCredentials) -> Result<Self::UserProfile, OAuthError> {
        // Ensure the CSRF state has not been tampered with.
        if creds.old_state.secret() != creds.new_state.secret() {
            return Err(OAuthError::CsrfMismatch);
        };
        
        let token = self.client
            .exchange_code(AuthorizationCode::new(creds.code))
            .request_async(async_http_client)
            .await
            .context("google oauth token error")
            .map_err(|e| OAuthError::Token(anyhow!(e)))?;
        
        
        let profile = self
            .http_client
            .get("https://openidconnect.googleapis.com/v1/userinfo")
            .bearer_auth(token.access_token().secret().to_owned())
            .send()
            .await?;
        
        let profile = profile.json::<Self::UserProfile>().await?;
        Ok(profile)
    }
    
    fn client_id(&self) -> String {
        self.client.client_id().to_string()
    }
    
    fn authorize_url(&self) -> (oauth2::url::Url, oauth2::CsrfToken) {
        //https://accounts.google.com/o/oauth2/v2/auth?scope=openid%20profile%20email&client_id={{google_client_id}}&response_type=code&redirect_uri={{google_redirect_url}}

        let url = self
            .client
            .authorize_url(CsrfToken::new_random)
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()))
            .add_scope(Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()))
            .url();

        url
    }
}