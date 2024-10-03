use std::collections::{HashMap, HashSet};

use askama::Template;
use axum_messages::{Message, Level};
use validator::ValidationError;

use crate::domain::identityaccess::model::users::UserDescriptor;


#[derive(Template)]
#[template(path = "pages/profile.html", escape = "none")]
pub struct ProfileTemplate {
    pub user: UserDescriptor,
}

impl ProfileTemplate {
    pub fn new(user: UserDescriptor) -> Self {
        Self {
            user,
        }
    }
}

#[derive(Template)]
#[template(path = "layouts/settings.html", escape = "none")]
pub struct SettingsTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    profile_page: ProfileTemplate,
}

impl SettingsTemplate {
    pub fn new(user: UserDescriptor, message: Option<Message>) -> Self {
        let navbar = NavbarTemplate{profile_picture: user.picture.to_string()};
        let profile_page = ProfileTemplate::new(user);
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));

        SettingsTemplate {
            navbar,
            alert,
            profile_page,
        }
    }
}

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

#[derive(Template)]
#[template(path = "partials/alert.html", escape = "none")]
pub struct AlertTemplate {
    alert_id: String,
    message: Message,
}

impl AlertTemplate {
    pub fn new(alert_id: impl Into<String>, message: Message) -> Self {
        AlertTemplate {
            alert_id: alert_id.into(),
            message: message,
        }
    }

    pub fn warning(alert_id: impl Into<String>, message: impl Into<String>) -> Self {
        let message =  Message { level: Level::Warning, message: message.into(), metadata: None };

        AlertTemplate {
            alert_id: alert_id.into(),
            message: message,
        }
    }
    
    pub fn error(alert_id: impl Into<String>, message: impl Into<String>) -> Self {
        let message = Message { level: Level::Error, message: message.into(), metadata: None };

        AlertTemplate {
            alert_id: alert_id.into(),
            message: message,
        }
    }
}

#[derive(Template)]
#[template(path = "partials/navbar.html", escape = "none")]
pub struct NavbarTemplate {
    profile_picture: String,
}