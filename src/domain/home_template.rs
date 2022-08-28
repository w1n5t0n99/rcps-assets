use sailfish::TemplateOnce;
use crate::utils::MsgType;

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "home.stpl")]
pub struct HomeTemplate {
    pub messages: Vec<(MsgType, String)>,
}