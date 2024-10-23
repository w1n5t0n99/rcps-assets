use axum::extract::FromRef;
use reqwest::Client;

use crate::domain::identityaccess::model::user_repository::UserRepository;

use super::{content::content_application_service::ContentApplicationService, identityaccess::identity_application_service::IdentityApplicationService};


#[derive(Debug, Clone)]
pub struct AppState<U: UserRepository> {
    pub identity_service: IdentityApplicationService<U>,
    pub content_service: ContentApplicationService,
}

impl<U> FromRef<AppState<U>> for IdentityApplicationService<U>
where U: UserRepository
{
    fn from_ref(input: &AppState<U>) -> Self {
        input.identity_service.clone()
    }
}

impl<U> AppState<U>
    where U: UserRepository
{
    pub fn new(identity_service: IdentityApplicationService<U>, content_service: ContentApplicationService) -> Self {
        Self {
            identity_service,
            content_service,
        }
    }
}

