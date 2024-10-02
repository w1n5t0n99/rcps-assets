pub mod items;

use axum::{routing::get, Router};

use crate::{application::state::AppState, domain::identityaccess::model::user_repository::UserRepository};

pub fn router<U>() -> Router<AppState<U>>
where U: UserRepository
{
    Router::new()
        .route("/settings", get(self::items::settings::<U>))
}