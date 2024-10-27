use anyhow::anyhow;
use axum::extract::Query;
use axum_login::AuthSession;
use serde::Deserialize;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, identityaccess::identity_application_service::IdentityApplicationService, templates::partials::users::roles_list::RolesListTemplate}, domain::identityaccess::model::user_repository::UserRepository};


#[derive(Deserialize)]
pub struct Params {
    pub selected: Option<String>,
}

#[instrument(skip_all)]
pub async fn get_roles(
    auth_session: AuthSession<IdentityApplicationService>,
    Query(params): Query<Params>,
) -> Result<RolesListTemplate, ApplicationError> {

    let roles = auth_session.backend.get_roles()
        .await
        .map_err(|e| ApplicationError::InternalServerError(anyhow!(e)))?;

    Ok(RolesListTemplate::new(params.selected, roles))
}