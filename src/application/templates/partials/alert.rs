use askama::Template;
use axum_messages::{Message, Level};


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

