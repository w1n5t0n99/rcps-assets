use askama::Template;
use axum_messages::Message;

use crate::{application::templates::partials::{alert::AlertTemplate, navbar::NavbarTemplate}, domain::{crud::model::asset_types::{AssetType, AssetTypeFilter}, identityaccess::model::users::SessionUser}};


#[derive(Template)]
#[template(path = "pages/asset_types.html", escape = "none")]
pub struct AssetTypesTemplate {
    navbar: NavbarTemplate,
    alert: Option<AlertTemplate>,
    asset_types: Vec<AssetType>,
    filter: AssetTypeFilter,
}

impl AssetTypesTemplate {
    pub fn new(session_user: SessionUser, message: Option<Message>, asset_types: Vec<AssetType>, filter: AssetTypeFilter) -> Self {
        let navbar = NavbarTemplate::new(session_user.user.picture.to_string());
        let alert = message.map(|m| AlertTemplate::new("global_alert_message", m));
        Self {navbar, alert, asset_types, filter}
    }
    
    pub fn sort_icon(&self, row: &str) -> &'static str {
        let order: Option<&str> = self.filter.order.as_deref();
        let sort: Option<&str> = self.filter.sort.as_deref();

        match (sort, order) {
            (Some(sort), Some(order)) => {
                if sort == row && order == "ASC" {
                    "↑"
                } else if sort == row && order == "DESC" {
                    "↓"
                }
                else {
                    ""
                }
            },
            _ => {""}
        }
    }
}
