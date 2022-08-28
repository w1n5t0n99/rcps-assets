use sailfish::TemplateOnce;
use crate::utils::MsgType;

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "add_asset.stpl")]
pub struct AddAssetTemplate {
    pub messages: Vec<(MsgType, String)>,
}