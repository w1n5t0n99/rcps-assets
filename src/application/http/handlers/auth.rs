use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{extract::{Query, State}, http::{HeaderValue, StatusCode}, middleware, response::Redirect, routing::{get, post}, Form, Router};
use axum_login::{tower_sessions::Session, AuthSession};
use axum_messages::Messages;
use oauth2::CsrfToken;
use serde::Deserialize;
use tracing::instrument;

use crate::{application::{errors::ApplicationError, http::utils, identityaccess::{identity_application_service::IdentityApplicationService, schema::AuthSchema}, state::AppState, templates::{pages::login::LoginTemplate, partials::alert::AlertTemplate}}, domain::identityaccess::model::{credentials::{Credentials, PasswordCredentials}, user_repository::UserRepository}};


pub fn router() -> Router<AppState>
{
    let loguout_router = Router::<AppState>::new()
        .route("/sessions/logout", get(self::logout))
        .route_layer(middleware::from_fn(utils::login_required));
    
    let login_router = Router::<AppState>::new()
        .route("/sessions/login", get(self::login))
        .route("/sessions/login", post(self::post_login))
        .route_layer(middleware::from_fn(utils::public_only));

    Router::<AppState>::new().merge(loguout_router).merge(login_router)
}

#[instrument(skip_all)]
pub async fn post_login(
    mut auth_session: AuthSession<IdentityApplicationService>,
    Form(AuthSchema { email, password, }): Form<AuthSchema>,
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
pub async fn login(messages: Messages) -> LoginTemplate {
    let message = messages
        .into_iter()
        .collect::<Vec<_>>()
        .first()
        .map(|m| m.to_owned());

    LoginTemplate::new(message)
}

#[instrument(skip_all)]
pub async fn logout(mut auth_session:  AuthSession<IdentityApplicationService>) -> Result<impl IntoResponse, ApplicationError> {
    match auth_session.logout().await {
        Ok(_) => Ok(Redirect::to("/sessions/login").into_response()),
        Err(e) => Err(ApplicationError::internal_server_error(anyhow!(e))),
    }
}