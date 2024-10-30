use axum::http::uri::Scheme;

use crate::{application::content::content_application_service::{ContentApplicationService, ContentError}, domain::crud::{crud_repository::{CrudRepository, CrudRepositoryError}, model::asset_types::{AssetType, NewAssetType, UpdateAssetType}}, infastructure::services::postgres_crud_repository::PostgresCrudRepository};

use super::schema::{NewAssetTypeSchema, UpdateAssetTypeSchema};



#[derive(Debug, thiserror::Error)]
pub enum CrudError {
    #[error(transparent)]
    Repo(#[from] CrudRepositoryError),
    #[error("transparent")]
    Content(#[from] ContentError),
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

   pub async fn  get_asset_type(&self, id: i32) -> Result<Option<AssetType>, CrudError> {
    let asset_types = self.crud_repo.get_asset_type_by_id(id).await?;

    Ok(asset_types)
}

   pub async fn add_asset_type(&self, schema: NewAssetTypeSchema, content: &ContentApplicationService) -> Result<AssetType, CrudError> {
        // should be validated in handler 
        let attachment_url = match schema.picture {
            Some(temp_file) => {
                let attachment = content.upload_image_file_as_attachment(temp_file)
                    .await?;

                attachment.url
            },
            None => {
                "/static/images/empty-image.svg".to_string()
            },
        };

        let new_asset_type = NewAssetType {
            brand: schema.brand,
            model: schema.model,
            description: schema.description,
            cost: schema.cost,
            picture: Some(attachment_url),
        };

        let asset_type = self.crud_repo.add_asset_type(new_asset_type)
            .await?;

        Ok(asset_type)
   } 

   pub async fn update_asset_type(&self, id: i32, schema: UpdateAssetTypeSchema) -> Result<Option<AssetType>, CrudError> {

        let update_asset_type = UpdateAssetType {
            brand: schema.brand,
            model: schema.model,
            description: schema.description,
            cost: schema.cost,
        };

        let asset_type = self.crud_repo.update_asset_type(id, update_asset_type)
            .await?;

        Ok(asset_type)
    }   

    pub async fn delete_asset_type(&self, id: i32) -> Result<Option<i32>, CrudError> {
        let asset_type = self.crud_repo.delete_asset_type(id)
            .await?;

        Ok(asset_type)
    }  

    pub async fn update_asset_type_picture(&self, id: i32, picture_url: String) -> Result<Option<AssetType>, CrudError> {
        let asset_type = self.crud_repo.update_asset_type_picture(id, picture_url)
            .await?;

        Ok(asset_type)
    }
}

