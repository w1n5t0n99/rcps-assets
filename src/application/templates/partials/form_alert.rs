use askama::Template;
use garde::Report;


#[derive(Template)]
#[template(path = "partials/form_alert.html", escape = "none")]
pub struct FormAlertTemplate {
    alert_id: String,
    report: Report,
}

impl FormAlertTemplate {
    pub fn new(alert_id: impl Into<String>, report: Report) -> Self {
        Self {
            alert_id: alert_id.into(),
            report,
        }
    }

    pub fn global_new(report: Report) -> Self {
        Self {
            alert_id: "global_alert_message".to_string(),
            report,
        }
    }
}