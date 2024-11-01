mod settings;
mod roles;
mod users;
mod user_create;
mod user_edit;

use axum::{extract::DefaultBodyLimit, middleware, response::Redirect, routing::{delete, get, post}, Router};

use crate::application::{http::utils, state::AppState};

pub fn router() -> Router<AppState>
{
    Router::new()
        .route("/", get(|| async { Redirect::permanent("/settings") }))
        .route("/settings", get(self::settings::get_settings))
        .route("/settings/roles", get(self::roles::get_roles))
        .route("/settings/users", get(self::users::get_users))
        .route("/settings/users/new", get(self::user_create::get_user_create))
        .route("/settings/users/new", post(self::user_create::post_user_create).layer(DefaultBodyLimit::max(5242880)))
        .route("/settings/users/:user_id/edit", get(self::user_edit::get_user_edit))
        .route("/settings/users/:user_id/edit", post(self::user_edit::post_user_edit).layer(DefaultBodyLimit::max(5242880)))
        .route("/settings/users/:user_id", delete(self::user_edit::delete_user))
        .route("/settings/users/:user_id/change_picture", post(self::user_edit::post_change_user_picture).layer(DefaultBodyLimit::max(5242880))) //5MiB
        .route_layer(middleware::from_fn(utils::login_required))
}