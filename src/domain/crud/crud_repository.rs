use std::future::Future;

use thiserror::Error;

use super::model::{asset_items::{AssetItem, AssetItemID, NewAssetItem}, asset_types::{AssetType, AssetTypeFilter, NewAssetType, UpdateAssetType, UploadResult}};


#[derive(Error, Debug)]
pub enum CrudRepositoryError {
    #[error("item already exists")]
    Duplicate,
    #[error("related item in foreign table does not exist")]
    Reference,
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

pub trait CrudRepository: Send + Sync + Clone + 'static {

    fn add_asset_type(
        &self,
        new_asset_type: NewAssetType,
    ) -> impl Future<Output = Result<AssetType, CrudRepositoryError>> + Send;

    fn get_asset_type_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<AssetType>, CrudRepositoryError>> + Send;

    fn get_asset_type(
        &self,
        brand: String,
        model: String,
    ) -> impl Future<Output = Result<Option<AssetType>, CrudRepositoryError>> + Send;

    fn get_asset_types(
        &self,
    ) -> impl Future<Output = Result<Vec<AssetType>, CrudRepositoryError>> + Send;

    fn get_asset_types_search(
        &self,
        filter: AssetTypeFilter,
    ) -> impl Future<Output = Result<Vec<AssetType>, CrudRepositoryError>> + Send;

    fn update_asset_type(
        &self,
        id: i32,
        update_asset_type: UpdateAssetType,
    ) -> impl Future<Output = Result<Option<AssetType>, CrudRepositoryError>> + Send;

    fn update_asset_type_picture(
        &self,
        id: i32,
        picture: String,
    ) -> impl Future<Output = Result<Option<AssetType>, CrudRepositoryError>> + Send;

    fn delete_asset_type(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<i32>, CrudRepositoryError>> + Send;

    fn bulk_add_asset_type(
        &self,
        new_asset_types: &[NewAssetType],
    ) -> impl Future<Output = Result<UploadResult, CrudRepositoryError>> + Send;

    fn bulk_add_or_update_asset_type(
        &self,
        new_asset_types: &[NewAssetType],
    ) -> impl Future<Output = Result<UploadResult, CrudRepositoryError>> + Send;

    fn add_asset_item(
        &self,
        new_asset_item: NewAssetItem,
    ) -> impl Future<Output = Result<AssetItemID, CrudRepositoryError>> + Send;

    fn get_asset_items(
        &self,
    ) -> impl Future<Output = Result<Vec<AssetItem>, CrudRepositoryError>> + Send;

    fn get_asset_item_by_id(
        &self,
        id: i32,
    ) -> impl Future<Output = Result<Option<AssetItem>, CrudRepositoryError>> + Send;
}

