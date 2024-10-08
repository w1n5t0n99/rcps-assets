use askama::Template;


#[derive(Template)]
#[template(path = "partials/navbar.html", escape = "none")]
pub struct NavbarTemplate {
    profile_picture: String,
}

impl NavbarTemplate {
    pub fn new(profile_picture: String) -> Self {
        Self {
            profile_picture
        }
    }
}