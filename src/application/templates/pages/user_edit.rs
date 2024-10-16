use askama::Template;
use axum_messages::Message;

use crate::{application::templates::partials::{alert::AlertTemplate, navbar::NavbarTemplate}, domain::identityaccess::model::users::{SessionUser, UserDescriptor}};


#[derive(Template)]
#[template(path = "pages/user_edit.html", escape = "none")]
pub struct UserEditTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    session_user: SessionUser,
    user: UserDescriptor,
}

impl UserEditTemplate {
    pub fn new(session_user: SessionUser, user: UserDescriptor, message: Option<Message>) -> Self {
        let navbar = NavbarTemplate::new(session_user.user.picture.to_string());
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));
        Self {navbar, session_user, user, alert}
    }
}