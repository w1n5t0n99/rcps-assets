use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, web};
use sailfish::TemplateOnce;
use sqlx::PgPool;
use crate::utils::e500;
use crate::domain::Asset;


#[derive(serde::Deserialize)]
pub struct QueryParams {
    search: Option<String>,
}

#[tracing::instrument( name = "View Assets", skip(pool, query),)]
pub async fn assets(pool: web::Data<PgPool>, query: web::Query<QueryParams>) -> Result<HttpResponse, actix_web::Error> {
    

    todo!()
}

#[tracing::instrument(name = "Retrieve assets from database", skip(pool))]
async fn retrieve_assets(pool: &PgPool) -> Result<Asset, sqlx::Error> {

    let result = sqlx::query!(
        r#"SELECT id, asset_id, name, serial_num FROM assets"#,
    )
    .fetch_all(pool)
    .await?;
    
    todo!()
}



