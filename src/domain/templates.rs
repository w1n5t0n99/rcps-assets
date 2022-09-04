use sailfish::TemplateOnce;
use actix_web_flash_messages::Level;
use super::{PartialAsset, Asset};


#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "home.stpl")]
pub struct HomeTemplate {
    pub messages: Vec<(Level, String)>,
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "assets.stpl")]
pub struct AssetsTemplate {
    pub assets: Vec<PartialAsset>,
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "add_asset.stpl")]
pub struct AddAssetTemplate {
    pub messages: Vec<(Level, String)>,
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "asset.stpl")]
pub struct AssetTemplate {
    pub messages: Vec<(Level, String)>,
    pub asset: Asset,
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "edit_asset.stpl")]
pub struct EditAssetTemplate {
    pub asset: Asset,
}

