use askama::Template;

use crate::domain::crud::model::asset_types::AssetType;


#[derive(Template)]
#[template(path = "partials/crud/asset_types_search_results.html")]
pub struct AssetTypesSearchResults {
    asset_types: Vec<AssetType>,
}

impl AssetTypesSearchResults {
    pub fn new(asset_types: Vec<AssetType>) -> Self {
      Self {
        asset_types,
      }
    }
}