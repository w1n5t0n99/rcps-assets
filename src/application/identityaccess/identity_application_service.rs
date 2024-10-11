use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use oauth2::{url::Url, CsrfToken};
use thiserror::Error;

use crate::{domain::identityaccess::model::{credentials::Credentials, oauth_service::{OAuthError, OAuthService}, password_service::{PasswordError, PasswordService}, roles::Role, user_repository::{UserRepository, UserRepositoryError}, users::{EmailAddress, NewUser, PasswordHash, Picture, UserDescriptor}}, infastructure::services::google_oauth_service::GoogleOauthService};

use super::schema::NewUserSchema;


pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error(transparent)]
    User(#[from] UserRepositoryError),
    #[error(transparent)]
    Password(#[from] PasswordError),
    #[error(transparent)]
    OAuth(#[from] OAuthError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct IdentityApplicationService<U>
where 
    U: UserRepository
{
    user_repo: U,
    google_oauth: GoogleOauthService,
    password: PasswordService,
}

impl<U> IdentityApplicationService<U>
where 
    U: UserRepository
{
    pub fn new(user_repo: U, google_oauth: GoogleOauthService) -> Self {
        Self {
            user_repo,
            google_oauth,
            password: PasswordService::new(),
        }
    }

    pub async fn get_users(&self) -> Result<Vec<UserDescriptor>, IdentityError> {
        let users = self.user_repo.get_user_descriptors()
            .await?;

        Ok(users)
    }

    pub async fn get_roles(&self) -> Result<Vec<Role>, IdentityError> {
        let roles = self.user_repo.get_roles()
            .await?;

        Ok(roles)
    }

    pub async fn add_user(&self, user: NewUserSchema) -> Result<UserDescriptor, IdentityError> {
        // should be validated in handler
        let password_hash = self.password.generate_password(user.password)
            .await?;

        let new_user = NewUser {
            password_hash: PasswordHash::new(password_hash),
            email: EmailAddress::new(user.email),
            email_verified: true,
            given_name: user.given_name,
            family_name: user.family_name,
            role_id: user.role_id,
            picture: Picture::new(user.picture),
        };

        let user_desc = self.user_repo.add_user(new_user)
            .await?;

        Ok(user_desc)
    }

    pub fn google_client_id(&self) -> String {
        self.google_oauth.client_id()
    }
    
    pub fn google_auth_url(&self) -> (Url, CsrfToken) {
        self.google_oauth.authorize_url()
    }

}

#[async_trait]
impl<U> AuthnBackend for IdentityApplicationService<U>
where 
    U: UserRepository
{
    type User = UserDescriptor;
    type Credentials = Credentials;
    type Error = IdentityError;

    async fn authenticate(&self, creds: Self::Credentials) -> Result<Option<Self::User>, Self::Error> {
        match creds {
            Credentials::Password(cred) => {
                let user = self.user_repo.get_user_for_auth(
                    EmailAddress::new(&cred.email),
                ).await?;

                if let Some(user) = user {
                    let is_valid = self.password.authenticate(cred, user.password_hash.to_string())
                        .await?;

                    if is_valid {
                            return Ok(Some(user.into()));
                    }
                }
            }
            Credentials::OAuth(cred) => {
                let profile = self.google_oauth.authenticate(cred).await?;
                
                let user = self.user_repo.get_user_descriptor_for_auth(
                    EmailAddress::new(profile.email),
                    Some(Picture::new(profile.picture)),
                ).await?;

                return Ok(user);
            }
        }

        Ok(None)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = self.user_repo.get_user_descriptor(user_id).await?;
        Ok(user)
    }
}

impl AuthUser for UserDescriptor {
    type Id = uuid::Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        //TODO: change auth source
        self.id.as_bytes()
    }
}