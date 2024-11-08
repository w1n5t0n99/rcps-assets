pub mod asset_items;
pub mod asset_item_create;
pub mod asset_item_view;
pub mod asset_items_edit;

use axum::{middleware, routing::{delete, get, post}, Router};

use crate::application::{http::utils, state::AppState};


pub fn router() -> Router<AppState>
{
    Router::new()
        .route("/asset_items", get(self::asset_items::get_asset_items))
        .route("/asset_items/new", get(self::asset_item_create::get_asset_item_create))
        .route("/asset_items/new", post(self::asset_item_create::post_asset_item_create))
        .route("/asset_items/:id", get(self::asset_item_view::get_asset_item_view))
        .route("/asset_items/:id/edit", get(self::asset_items_edit::get_asset_item_edit))
        .route("/asset_items/:id/edit", post(self::asset_items_edit::post_asset_item_edit))
        .route_layer(middleware::from_fn(utils::login_required))
}