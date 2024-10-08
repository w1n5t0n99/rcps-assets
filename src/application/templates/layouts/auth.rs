use askama::Template;
use axum_messages::Message;

use crate::application::templates::pages::login::LoginTemplate;


#[derive(Template)]
#[template(path = "layouts/auth.html", escape = "none")]
pub struct AuthTemplate {
    login_page: LoginTemplate,
}

impl AuthTemplate {
    pub fn new(message: Option<Message>) -> Self {
        AuthTemplate { login_page: LoginTemplate::new(message) }
    }
}