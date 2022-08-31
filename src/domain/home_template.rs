use sailfish::TemplateOnce;
use actix_web_flash_messages::Level;


#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "home.stpl")]
pub struct HomeTemplate {
    pub messages: Vec<(Level, String)>,
}