use sailfish::TemplateOnce;
use super::Asset;

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "asset.stpl")]
pub struct AssetTemplate {
    pub asset: Asset,
}