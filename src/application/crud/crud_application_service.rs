use crate::{domain::crud::{crud_repository::{CrudRepository, CrudRepositoryError}, model::asset_types::AssetType}, infastructure::services::postgres_crud_repository::PostgresCrudRepository};



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
}