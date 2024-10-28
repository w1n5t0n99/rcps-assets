pub mod asset_types;
pub mod asset_type_create;
pub mod asset_types_view;
pub mod asset_type_edit;

use axum::{extract::DefaultBodyLimit, middleware, routing::{delete, get, post}, Router};

use crate::application::{http::utils, state::AppState};

pub fn router() -> Router<AppState>
{
    Router::new()
        .route("/asset_types/:id", get(self::asset_types_view::get_asset_type_view))
        .route("/asset_types/:id", delete(self::asset_type_edit::delete_asset_type))
        .route("/asset_types/:id/edit", get(self::asset_type_edit::get_asset_type_edit))
        .route("/asset_types/:id/edit", post(self::asset_type_edit::post_asset_type_edit))
        .route("/asset_types/new", get(self::asset_type_create::get_asset_type_create))
        .route("/asset_types/new", post(self::asset_type_create::post_asset_type_create).layer(DefaultBodyLimit::max(5242880)))
        .route("/asset_types", get(self::asset_types::get_asset_types))
        .route_layer(middleware::from_fn(utils::login_required))
}