use askama::Template;
use axum_messages::Message;
use validator::ValidationError;

use crate::{application::templates::{pages::profile::ProfileTemplate, partials::{alert::AlertTemplate, navbar::NavbarTemplate}}, domain::identityaccess::model::users::UserDescriptor};


#[derive(Template)]
#[template(path = "layouts/settings.html", escape = "none")]
pub struct SettingsTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    profile_page: ProfileTemplate,
}

impl SettingsTemplate {
    pub fn new(user: UserDescriptor, message: Option<Message>) -> Self {
        let navbar = NavbarTemplate::new(user.picture.to_string());
        let profile_page = ProfileTemplate::new(user);
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));

        SettingsTemplate {
            navbar,
            alert,
            profile_page,
        }
    }
}