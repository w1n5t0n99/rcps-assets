use anyhow::anyhow;
use askama_axum::IntoResponse;
use axum::{
    extract::Request, http::{HeaderMap, StatusCode}, middleware::{self, Next}, response::{Redirect, Response}, routing::get, RequestExt, Router
};
use axum_login::AuthSession;
use axum_messages::Messages;

use crate::{application::{errors::ApplicationError, identityaccess::identity_application_service::IdentityApplicationService}, domain::identityaccess::model::{user_repository::UserRepository, users::SessionUser}};


pub async fn login_required(
    auth_session: AuthSession<IdentityApplicationService>,
    // you can also add more extractors here but the last
    // extractor must implement `FromRequest` which
    // `Request` does
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if let Some(user) = auth_session.user {
        let session_user: SessionUser = user.into();
        request.extensions_mut().insert(session_user);

        let response = next.run(request).await;
        return Ok(response);
    }
   
   if request.headers().contains_key("Hx-Request") {
        // HTMX does not procees 300 responses, so we must send a 200 response to redirect
        Ok(([("HX-Redirect", "/sessions/login")], "HTMX request not logged in").into_response())
   } else {
        Ok(Redirect::to("/sessions/login").into_response()) 
   }
}

pub async fn public_only(
    auth_session: AuthSession<IdentityApplicationService>,
    mut request: Request,
    next: Next,
) -> Result<Response, ApplicationError> {
    if auth_session.user.is_none() {
        let response = next.run(request).await;
            return Ok(response);
    }

    // Calling extractor manually so it doesn't consume any messages on happy path
    let messages = request.extract_parts::<Messages>()
        .await
        .map_err(|_| ApplicationError::internal_server_error(anyhow!("error extracting flash messages")))?;

    messages.error("You are already signed in.");
    Ok(Redirect::to("/settings").into_response()) 
}
