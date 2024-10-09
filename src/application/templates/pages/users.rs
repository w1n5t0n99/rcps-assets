use askama::Template;
use axum_messages::Message;

use crate::{application::templates::{pages::users, partials::{alert::AlertTemplate, navbar::NavbarTemplate}}, domain::identityaccess::model::users::UserDescriptor};


#[derive(Template)]
#[template(path = "pages/users.html", escape = "none")]
pub struct UsersTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    users: Vec<UserDescriptor>,

}

impl UsersTemplate {
    pub fn new(logged_in_user: UserDescriptor, users: Vec<UserDescriptor>, message: Option<Message>) -> Self {
        let navbar = NavbarTemplate::new(logged_in_user.picture.to_string());
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));
        Self {navbar, users, alert}
    }
}