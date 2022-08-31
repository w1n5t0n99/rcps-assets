use sailfish::TemplateOnce;
use actix_web_flash_messages::Level;


#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "add_asset.stpl")]
pub struct AddAssetTemplate {
    pub messages: Vec<(Level, String)>,
}