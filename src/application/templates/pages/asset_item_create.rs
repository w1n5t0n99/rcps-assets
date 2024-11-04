use askama::Template;
use axum_messages::Message;

use crate::{application::templates::partials::{alert::AlertTemplate, navbar::NavbarTemplate}, domain::identityaccess::model::users::{SessionUser, UserDescriptor}};


#[derive(Template)]
#[template(path = "pages/asset_item_create.html", escape = "none")]
pub struct AssetItemCreateTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
}

impl AssetItemCreateTemplate {
    pub fn new(session_user: SessionUser, message: Option<Message>) -> Self {
        let navbar = NavbarTemplate::new(session_user.user.picture.to_string());
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));
        Self {navbar, alert}
    }
}