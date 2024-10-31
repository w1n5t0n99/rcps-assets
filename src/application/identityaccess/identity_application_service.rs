use anyhow::anyhow;
use axum::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use oauth2::{url::Url, CsrfToken};
use thiserror::Error;
use uuid::Uuid;

use crate::{application::content::content_application_service::{ContentApplicationService, ContentError}, domain::identityaccess::model::{credentials::Credentials, oauth_service::{OAuthError, OAuthService}, password_service::{PasswordError, PasswordService}, roles::Role, user_repository::{UserRepository, UserRepositoryError}, users::{EmailAddress, NewUser, PasswordHash, Picture, SessionUser, UpdateUser, UserDescriptor}}, infastructure::services::{google_oauth_service::GoogleOauthService, postgres_user_repository::PostgresUserRepository}};

use super::schema::{NewUserSchema, UpdateUserSchema};


pub const CSRF_STATE_KEY: &str = "oauth.csrf-state";

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error(transparent)]
    User(#[from] UserRepositoryError),
    #[error(transparent)]
    Password(#[from] PasswordError),
    #[error("transparent")]
    Content(#[from] ContentError),
    #[error(transparent)]
    OAuth(#[from] OAuthError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct IdentityApplicationService
{
    user_repo: PostgresUserRepository,
    google_oauth: GoogleOauthService,
    password: PasswordService,
}

impl IdentityApplicationService
{
    pub fn new(user_repo: PostgresUserRepository, google_oauth: GoogleOauthService) -> Self {
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

    pub async fn add_user(&self, schema: NewUserSchema, content: &ContentApplicationService) -> Result<UserDescriptor, IdentityError> {
        let attachment_url = match schema.picture {
            Some(temp_file) => {
                let attachment = content.upload_image_file_as_attachment(temp_file)
                    .await?;

                attachment.url
            },
            None => {
                "/static/images/empty-image.svg".to_string()
            },
        };

        let password_hash = self.password.generate_password(schema.password)
            .await?;

        let new_user = NewUser {
            password_hash: PasswordHash::new(password_hash),
            email: EmailAddress::new(schema.email),
            email_verified: true,
            given_name: schema.given_name,
            family_name: schema.family_name,
            role_id: schema.role_id,
            picture: Picture::new(attachment_url),
        };

        let user_desc = self.user_repo.add_user(new_user)
            .await?;

        Ok(user_desc)
    }

    pub async fn delete_user(&self, session_user: SessionUser, user_id: Uuid) -> Result<Option<Uuid>, IdentityError> {
        if session_user.user.id == user_id {
            return Err(IdentityError::Unknown(anyhow!("cannot delete session user")))?;
        }

        self.user_repo.delete_user(user_id)
            .await
            .map_err(|e| e.into())
    }

    pub async fn update_user(&self, session_user: SessionUser, user_id: Uuid, schema: UpdateUserSchema, content: &ContentApplicationService) -> Result<Option<UserDescriptor>, IdentityError> {

        let attachment_url = match schema.picture {
            Some(temp_file) => {
                let attachment = content.upload_image_file_as_attachment(temp_file)
                    .await?;

                Some(Picture::new(attachment.url))
            },
            None => { None },
        };

        let update_user = UpdateUser {
            given_name: schema.given_name,
            family_name: schema.family_name,
            role_id: schema.role_id,
            picture: attachment_url,
        };

        if session_user.user.id == user_id {
            self.user_repo.update_session_user(user_id, update_user)
                .await
                .map_err(|e| e.into())
        }
        else {
            self.user_repo.update_user(user_id, update_user)
                .await
                .map_err(|e| e.into())
        }
    }

    pub async fn update_user_picture(&self, user_id: Uuid, picture_url: String) -> Result<Option<UserDescriptor>, IdentityError> {
        let picture = Picture::new(picture_url);

        self.user_repo.update_user_picture(user_id, picture)
            .await
            .map_err(|e| e.into())
    }

    pub fn google_client_id(&self) -> String {
        self.google_oauth.client_id()
    }
    
    pub fn google_auth_url(&self) -> (Url, CsrfToken) {
        self.google_oauth.authorize_url()
    }

}

#[async_trait]
impl AuthnBackend for IdentityApplicationService
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