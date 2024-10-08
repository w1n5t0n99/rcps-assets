use askama::Template;

use crate::domain::identityaccess::model::roles::Role;


#[derive(Template)]
#[template(path = "partials/users/roles_list.html")]
pub struct RolesListTemplate {
    selected_role: String,
    roles: Vec<Role>,
}

impl RolesListTemplate {
    pub fn new(selected_role: impl Into<String>, roles: Vec<Role>) -> Self {
        Self {
            selected_role: selected_role.into(),
            roles
        }
    }
}