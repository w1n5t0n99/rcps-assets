pub mod asset_items;
pub mod asset_item_create;

use axum::{middleware, routing::{delete, get, post}, Router};

use crate::application::{http::utils, state::AppState};


pub fn router() -> Router<AppState>
{
    Router::new()
        .route("/asset_items", get(self::asset_items::get_asset_items))
        .route("/asset_items/new", get(self::asset_item_create::get_asset_item_create))
        .route("/asset_items/new", post(self::asset_item_create::post_asset_item_create))
        .route_layer(middleware::from_fn(utils::login_required))
}