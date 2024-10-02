use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{extract::{Query, State}, http::StatusCode, middleware, response::Redirect, routing::{get, post}, Router};
use axum_login::{tower_sessions::Session, AuthSession};
use axum_messages::Messages;
use oauth2::CsrfToken;
use serde::Deserialize;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, http::utils, identityaccess::identity_application_service::{IdentityApplicationService, CSRF_STATE_KEY}, state::AppState}, domain::identityaccess::model::{credentials::{Credentials, OauthCredentials}, user_repository::UserRepository}, infastructure::services::postgres_user_repository::PostgresUserRepository};


pub fn router<U>() -> Router<AppState<U>>
where U: UserRepository
{
    Router::new()
        .route("/sessions/oauth/google", post(self::google_oauth::<U>))
        .route("/sessions/oauth/google", get(self::google_oauth_callback::<U>))
        .route_layer(middleware::from_fn(utils::public_only::<U>))
}

#[derive(Debug, Clone, Deserialize)]
struct AuthzResp {
    code: String,
    state: CsrfToken,
}

#[instrument(skip_all)]
pub async fn google_oauth_callback<U: UserRepository>(
    mut auth_session: AuthSession<IdentityApplicationService<U>>,
    session: Session,
    messages: Messages,
    Query(AuthzResp {
        code,
        state: new_state,
    }): Query<AuthzResp>,
) -> Result<impl IntoResponse, ApplicationError> {
    let Ok(Some(old_state)) = session.get(CSRF_STATE_KEY).await else {
        messages.error("Failed to sign in with Google");

        return Err(
            ApplicationError::redirect(anyhow!("oauth login failure - csrf mismatch"), "/sessions/login")
        ); 
    };

    let creds = Credentials::OAuth(OauthCredentials {
        code,
        old_state,
        new_state,
    });

    let user = auth_session.authenticate(creds).await;

    let user = match user {
        Ok(Some(u)) => { u }
        Ok(None) => { 
            messages.error("Failed to sign in with Google");

            return Err(
                ApplicationError::redirect(anyhow!("oauth login failure"), "/sessions/login")
            ); 
        }
        Err(e) => { return Err(ApplicationError::InternalServerError(anyhow!(e))); }
    };    

    auth_session.login(&user).await.map_err(|e| ApplicationError::InternalServerError(e.into()))?;

    Ok(Redirect::to("/settings").into_response())  
    //Ok((StatusCode::OK, "Google Callback"))    
}

#[instrument(skip_all)]
pub async fn google_oauth<U: UserRepository>(
    auth_session: AuthSession<IdentityApplicationService<U>>,
    session: Session,
) -> impl IntoResponse {

    let (auth_url, csrf_state) = auth_session.backend.google_auth_url();

    session
        .insert(CSRF_STATE_KEY, csrf_state.secret())
        .await
        .expect("Serialization should not fail.");
    
    Redirect::to(auth_url.as_str()).into_response()  
}