use askama::Template;
use axum_messages::Message;

use crate::{application::templates::partials::{alert::AlertTemplate, navbar::NavbarTemplate}, domain::{crud::model::{asset_items::AssetItem, asset_types::{AssetType, AssetTypeFilter}}, identityaccess::model::users::SessionUser}};


#[derive(Template)]
#[template(path = "pages/asset_items.html", escape = "none")]
pub struct AssetItemsTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    asset_items: Vec<AssetItem>,
}

impl AssetItemsTemplate {
    pub fn new(session_user: SessionUser, message: Option<Message>, asset_items: Vec<AssetItem>) -> Self {
        let navbar = NavbarTemplate::new(session_user.user.picture.to_string());
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));
        Self {navbar, alert, asset_items}
    }
}
