mod settings;
mod roles;
mod users;
mod user_create;

use axum::{middleware, routing::get, Router};

use crate::{application::{http::utils, state::AppState}, domain::identityaccess::model::user_repository::UserRepository};

pub fn router<U>() -> Router<AppState<U>>
where U: UserRepository
{
    Router::new()
        .route("/settings", get(self::settings::get_settings::<U>))
        .route("/settings/roles", get(self::roles::get_roles::<U>))
        .route("/settings/users", get(self::users::get_users::<U>))
        .route("/settings/users/new", get(self::user_create::get_user_create::<U>))
        .route_layer(middleware::from_fn(utils::login_required::<U>))
}