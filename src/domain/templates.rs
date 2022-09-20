use sailfish::TemplateOnce;
use actix_web_flash_messages::Level;
use super::{PartialAsset, Asset};


#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "home.stpl")]
pub struct HomeTemplate {
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "assets.stpl")]
pub struct AssetsTemplate {
    pub next_uri: String,
    pub prev_uri: String,
    pub assets: Vec<PartialAsset>,
    pub err_messages: Vec<String>,
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "add_asset.stpl")]
pub struct AddAssetTemplate {
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "asset.stpl")]
pub struct AssetTemplate {
    pub messages: Vec<String>,
    pub asset: Asset,
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "edit_asset.stpl")]
pub struct EditAssetTemplate {
    pub asset: Asset,
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "uploads.stpl")]
pub struct UploadsTemplate {
    pub messages: Vec<String>,
}

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "new_upload.stpl")]
pub struct NewUploadTemplate {
}

