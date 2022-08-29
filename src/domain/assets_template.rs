use sailfish::TemplateOnce;
use crate::domain::PartialAsset;

#[derive(TemplateOnce, Debug, PartialEq)] 
#[template(path = "assets.stpl")]
pub struct AssetsTemplate {
    pub assets: Vec<PartialAsset>,
}