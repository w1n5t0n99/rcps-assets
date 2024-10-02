use std::collections::{HashMap, HashSet};

use askama::Template;
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
    pub navbar: NavbarTemplate,
    profile_page: ProfileTemplate,
}

impl SettingsTemplate {
    pub fn new(user: UserDescriptor) -> Self {
        let navbar = NavbarTemplate{profile_picture: user.picture.to_string()};
        let profile_page = ProfileTemplate::new(user);

        SettingsTemplate {
            navbar,
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
    pub fn new(message: Option<impl Into<String>>) -> Self {
        let warning = message.map(
            |m| WarningTemplate::new("message", m.into()),
        );

        AuthTemplate {
            login_page: LoginTemplate { warning }
        }
    }
}

#[derive(Template)]
#[template(path = "pages/login.html", escape = "none")]
pub struct LoginTemplate {
    warning: Option<WarningTemplate>,
}

#[derive(Template)]
#[template(path = "partials/warning.html", escape = "none")]
pub struct WarningTemplate {
    alert_id: String,
    message: String,
}

impl WarningTemplate {
    pub fn new(alert_id: impl Into<String>, message: impl Into<String>) -> Self {
        WarningTemplate {
            alert_id: alert_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Template)]
#[template(path = "partials/error.html", escape = "none")]
pub struct ErrorTemplate {
    alert_id: String,
    message: String,
}

impl ErrorTemplate {
    pub fn new(alert_id: impl Into<String>, message: impl Into<String>) -> Self {
        ErrorTemplate {
            alert_id: alert_id.into(),
            message: message.into(),
        }
    }
}

#[derive(Template)]
#[template(path = "partials/navbar.html", escape = "none")]
pub struct NavbarTemplate {
    profile_picture: String,
}