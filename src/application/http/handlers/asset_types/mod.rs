pub mod asset_types;
pub mod asset_type_create;

use axum::{extract::DefaultBodyLimit, middleware, routing::{get, post}, Router};

use crate::{application::{http::utils, state::AppState}, domain::identityaccess::model::user_repository::UserRepository};

pub fn router() -> Router<AppState>
{
    Router::new()
        .route("/asset_types", get(self::asset_types::get_asset_types))
        .route("/asset_types/new", get(self::asset_type_create::get_asset_type_create))
        .route("/asset_types/new", post(self::asset_type_create::post_asset_type_create))
        .route_layer(middleware::from_fn(utils::login_required))
}