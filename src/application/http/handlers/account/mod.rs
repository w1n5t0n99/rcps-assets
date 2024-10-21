mod settings;
mod roles;
mod users;
mod user_create;
mod user_edit;
mod user_content;

use axum::{extract::DefaultBodyLimit, middleware, routing::{get, post}, Router};

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
        .route("/settings/users/:user_id/edit", post(self::user_edit::post_user_edit::<U>))
        .route("/settings/users/:user_id/delete", post(self::user_edit::post_user_delete::<U>))
        .route("/settings/users/:user_id/change_picture", post(self::user_content::post_change_user_picture::<U>).layer(DefaultBodyLimit::max(5242880))) //5MiB
        .route("/content/:hash/*filename", get(self::user_content::get_content::<U>))
        .route_layer(middleware::from_fn(utils::login_required::<U>))
}