use askama::Template;

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