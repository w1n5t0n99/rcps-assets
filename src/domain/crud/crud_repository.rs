use std::future::Future;

use thiserror::Error;

use super::model::asset_types::{AssetType, NewAssetType};


#[derive(Error, Debug)]
pub enum CrudRepositoryError {
    #[error("item already exists")]
    Duplicate,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait CrudRepository: Send + Sync + Clone + 'static {

    fn add_asset_type(
        &self,
        asset_item: NewAssetType,
    ) -> impl Future<Output = Result<AssetType, CrudRepositoryError>> + Send;

    fn get_asset_type_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<AssetType, CrudRepositoryError>> + Send;

    fn get_asset_type(
        &self,
        brand: String,
        model: String,
    ) -> impl Future<Output = Result<AssetType, CrudRepositoryError>> + Send;
}