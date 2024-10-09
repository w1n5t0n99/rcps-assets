use anyhow::anyhow;
use axum_login::AuthSession;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, identityaccess::identity_application_service::IdentityApplicationService, templates::partials::users::roles_list::RolesListTemplate}, domain::identityaccess::model::user_repository::UserRepository};


#[instrument(skip_all)]
pub async fn get_roles<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
) -> Result<RolesListTemplate, ApplicationError> {
    let user = auth_session.user.ok_or( ApplicationError::InternalServerError(anyhow!("user session not found")))?;

    let roles = auth_session.backend.get_roles()
        .await
        .map_err(|e| ApplicationError::InternalServerError(anyhow!(e)))?;

    Ok(RolesListTemplate::new(user.role, roles))
}