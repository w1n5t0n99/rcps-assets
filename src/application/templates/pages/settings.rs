use askama::Template;
use axum_messages::Message;

use crate::{application::templates::partials::{alert::AlertTemplate, navbar::NavbarTemplate}, domain::identityaccess::model::users::UserDescriptor};


#[derive(Template)]
#[template(path = "pages/settings.html", escape = "none")]
pub struct SettingsTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    user: UserDescriptor,
}

impl SettingsTemplate {
    pub fn new(logged_in_user: UserDescriptor, message: Option<Message>) -> Self {
        let navbar = NavbarTemplate::new(logged_in_user.picture.to_string());
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));
        Self {navbar, user: logged_in_user, alert}
    }
}