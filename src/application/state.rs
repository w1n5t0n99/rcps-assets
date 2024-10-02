use axum::extract::FromRef;
use reqwest::Client;

use crate::domain::identityaccess::model::user_repository::UserRepository;

use super::identityaccess::identity_application_service::IdentityApplicationService;


#[derive(Debug, Clone)]
pub struct AppState<U: UserRepository> {
    pub identity_service: IdentityApplicationService<U>,
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
    pub fn new(identity_service: IdentityApplicationService<U>) -> Self {
        Self {
            identity_service,
        }
    }
}

