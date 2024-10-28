use askama::Template;
use axum_messages::Message;

use crate::{application::templates::partials::{alert::AlertTemplate, navbar::NavbarTemplate}, domain::{crud::model::asset_types::AssetType, identityaccess::model::users::{SessionUser, UserDescriptor}}};


#[derive(Template)]
#[template(path = "pages/asset_type_view.html", escape = "none")]
pub struct AssetTypeViewTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    asset_type: AssetType,
}

impl AssetTypeViewTemplate {
    pub fn new(session_user: SessionUser, message: Option<Message>, asset_type: AssetType) -> Self {
        let navbar = NavbarTemplate::new(session_user.user.picture.to_string());
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));
        Self {navbar, alert, asset_type}
    }
}