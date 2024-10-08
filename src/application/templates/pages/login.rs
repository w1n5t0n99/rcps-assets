use askama::Template;
use axum_messages::Message;

use crate::application::templates::partials::alert::AlertTemplate;


#[derive(Template)]
#[template(path = "pages/login.html", escape = "none")]
pub struct LoginTemplate {
    alert: Option<AlertTemplate>,
}

impl LoginTemplate {
    pub fn new(message: Option<Message>) -> Self {
        let alert = message.map(|m| AlertTemplate::new("message", m));

        LoginTemplate { alert }
    }
}