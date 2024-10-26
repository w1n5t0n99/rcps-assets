use crate::{domain::crud::{crud_repository::{CrudRepository, CrudRepositoryError}, model::asset_types::{AssetType, NewAssetType}}, infastructure::services::postgres_crud_repository::PostgresCrudRepository};

use super::schema::NewAssetTypeSchema;



#[derive(Debug, thiserror::Error)]
pub enum CrudError {
    #[error(transparent)]
    Repo(#[from] CrudRepositoryError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct CrudApplicationService {
    crud_repo: PostgresCrudRepository,
}

impl CrudApplicationService {
    pub fn new(crud_repo: PostgresCrudRepository) -> Self {
        Self{crud_repo}
    }

   pub async fn  get_asset_types(&self) -> Result<Vec<AssetType>, CrudError> {
        let asset_types = self.crud_repo.get_asset_types().await?;

        Ok(asset_types)
   }

   pub async fn add_asset_type(&self, schema: NewAssetTypeSchema) -> Result<AssetType, CrudError> {
        // should be validated in handler
        //TODO: process uploaded image

        let new_asset_type = NewAssetType {
            brand: schema.brand,
            model: schema.model,
            description: schema.description,
            cost: schema.cost,
            picture: None,
        };

        let asset_type = self.crud_repo.add_asset_type(new_asset_type)
            .await?;

        Ok(asset_type)
   } 
}