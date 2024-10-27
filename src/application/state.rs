use axum::extract::FromRef;
use reqwest::Client;

use crate::domain::identityaccess::model::user_repository::UserRepository;

use super::{content::content_application_service::ContentApplicationService, crud::crud_application_service::CrudApplicationService, identityaccess::identity_application_service::IdentityApplicationService};


#[derive(Debug, Clone)]
pub struct AppState {
    pub identity_service: IdentityApplicationService,
    pub content_service: ContentApplicationService,
    pub crud_service: CrudApplicationService,
}

impl FromRef<AppState> for IdentityApplicationService
{
    fn from_ref(input: &AppState) -> Self {
        input.identity_service.clone()
    }
}

impl FromRef<AppState> for ContentApplicationService
{
    fn from_ref(input: &AppState) -> Self {
        input.content_service.clone()
    }
}

impl FromRef<AppState> for CrudApplicationService
{
    fn from_ref(input: &AppState) -> Self {
        input.crud_service.clone()
    }
}

impl AppState
{
    pub fn new(identity_service: IdentityApplicationService, crud_service: CrudApplicationService, content_service: ContentApplicationService) -> Self {
        Self {
            identity_service,
            content_service,
            crud_service,
        }
    }
}

