use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{extract::{Query, State}, http::{HeaderValue, StatusCode}, middleware, response::Redirect, routing::{get, post}, Form, Router};
use axum_login::{tower_sessions::Session, AuthSession};
use axum_messages::Messages;
use oauth2::CsrfToken;
use serde::Deserialize;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, http::utils, identityaccess::identity_application_service::IdentityApplicationService, state::AppState, templates::{layouts::auth::AuthTemplate, partials::alert::AlertTemplate}}, domain::identityaccess::model::{credentials::{Credentials, PasswordCredentials}, user_repository::UserRepository}};


pub fn router<U>() -> Router<AppState<U>>
where U: UserRepository
{
    let loguout_router = Router::<AppState<U>>::new()
        .route("/sessions/logout", get(self::logout::<U>))
        .route_layer(middleware::from_fn(utils::login_required::<U>));
    
    let login_router = Router::<AppState<U>>::new()
        .route("/sessions/login", get(self::login))
        .route("/sessions/login", post(self::post_login::<U>))
        .route_layer(middleware::from_fn(utils::public_only::<U>));

    Router::<AppState<U>>::new().merge(loguout_router).merge(login_router)
}

#[derive(Debug, Clone, Deserialize)]
struct AuthzResp {
    email: String,
    password: String,
}

#[instrument(skip_all)]
pub async fn post_login<U: UserRepository>(
    mut auth_session: AuthSession<IdentityApplicationService<U>>,
    Form(AuthzResp { email, password, }): Form<AuthzResp>,
) ->  Result<impl IntoResponse, ApplicationError> {
    // TODO: validate form
    let creds = Credentials::Password(PasswordCredentials{
        email,
        password,
        next: None,
    });

    let user = auth_session.authenticate(creds).await;
    
    let user = match user {
        Ok(Some(u)) => { u }
        Ok(None) => { 
            return Err(ApplicationError::bad_request(
                anyhow!("password login failure"),
                AlertTemplate::warning("message", "Invalid login information")
            ));
        }
        Err(e) => { return Err(ApplicationError::InternalServerError(anyhow!(e))); }
    };    

    auth_session.login(&user).await.map_err(|e| ApplicationError::InternalServerError(e.into()))?;

    Ok(([("HX-Redirect", "/settings")], "success"))
}

#[instrument(skip_all)]
pub async fn login(messages: Messages) -> AuthTemplate {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    AuthTemplate::new(message)
}

#[instrument(skip_all)]
pub async fn logout<U: UserRepository>(mut auth_session:  AuthSession<IdentityApplicationService<U>>) -> Result<impl IntoResponse, ApplicationError> {
    match auth_session.logout().await {
        Ok(_) => Ok(Redirect::to("/sessions/login").into_response()),
        Err(e) => Err(ApplicationError::internal_server_error(anyhow!(e))),
    }
}