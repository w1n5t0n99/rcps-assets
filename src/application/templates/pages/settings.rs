use askama::Template;
use axum_messages::Message;
use tower_sessions::Session;

use crate::{application::templates::partials::{alert::AlertTemplate, navbar::NavbarTemplate}, domain::identityaccess::model::users::{SessionUser, UserDescriptor}};


#[derive(Template)]
#[template(path = "pages/settings.html", escape = "none")]
pub struct SettingsTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    session_user: SessionUser,
}

impl SettingsTemplate {
    pub fn new(session_user: SessionUser, message: Option<Message>) -> Self {
        let navbar = NavbarTemplate::new(session_user.user.picture.to_string());
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));
        Self {navbar, session_user, alert}
    }
}