mod settings;
mod roles;
mod users;
mod user_create;
mod user_edit;

use axum::{middleware, routing::{get, post}, Router};

use crate::{application::{http::utils, state::AppState}, domain::identityaccess::model::user_repository::UserRepository};

pub fn router<U>() -> Router<AppState<U>>
where U: UserRepository
{
    Router::new()
        .route("/settings", get(self::settings::get_settings::<U>))
        .route("/settings/roles", get(self::roles::get_roles::<U>))
        .route("/settings/users", get(self::users::get_users::<U>))
        .route("/settings/users/new", get(self::user_create::get_user_create::<U>))
        .route("/settings/users/new", post(self::user_create::post_user_create::<U>))
        .route("/settings/users/:user_id/edit", get(self::user_edit::get_user_edit::<U>))
        .route_layer(middleware::from_fn(utils::login_required::<U>))
}