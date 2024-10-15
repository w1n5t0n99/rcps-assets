use askama::Template;
use axum_messages::Message;

use crate::{application::templates::partials::{alert::AlertTemplate, navbar::NavbarTemplate}, domain::identityaccess::model::users::UserDescriptor};


#[derive(Template)]
#[template(path = "pages/user_edit.html", escape = "none")]
pub struct UserEditTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    logged_in_user: UserDescriptor,
    user: UserDescriptor,
}

impl UserEditTemplate {
    pub fn new(logged_in_user: UserDescriptor, user: UserDescriptor, message: Option<Message>) -> Self {
        let navbar = NavbarTemplate::new(logged_in_user.picture.to_string());
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));
        Self {navbar, logged_in_user,user, alert}
    }
}