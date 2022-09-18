use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use chrono::Utc;
use actix_web_flash_messages::FlashMessage;
use actix_multipart::{Multipart, Field};
use futures::{StreamExt, TryStreamExt};
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::{error_chain_fmt, see_other, e500};
use crate::domain::Asset;


#[tracing::instrument(
    name = "Upload Assets from file",
    skip(payload, pool),
)]
pub async fn upload_assets(
    mut payload: Multipart,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition();

        //let filename = content_type.get_filename().unwrap();
        let filepath = format!("./temp_files/{}.csv", uuid::Uuid::new_v4());
        let fp = filepath.clone();
        //FlashMessage::success(filepath).send();

        // File::create is blocking operation
        let mut f = spawn_blocking_with_tracing(move || std::fs::File::create(filepath))
            .await
            .map_err(e500)??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk?;

            // filesystem operations are blocking
            f = spawn_blocking_with_tracing(move|| f.write_all(&data).map(|_| f))
                .await
                .map_err(e500)??;            
        }

        // Parsing CSV file is blocking
        let assets = spawn_blocking_with_tracing(move || load_assets_from_csv(fp))
            .await
            .map_err(e500)?
            .map_err(e500)?;

        for a in assets.iter().take(5) {
            println!("############## {:?} #############", a.name);
        }

        // Insert assets into Assets table
        // Insert upload status into UploadHistory table
    }

    Ok(see_other("/asset_items/uploads"))
}

#[tracing::instrument(name = "Load assets from csv", skip(filepath))]
fn load_assets_from_csv(filepath: String) -> Result<Vec<Asset>, anyhow::Error> {  
    let mut rdr = csv::Reader::from_path(filepath)?;
    
    let assets: Vec<Asset> = rdr.deserialize().filter_map(|r| {
        match r {
            Ok(a) => {
                let asset: Asset = a;
                Some(asset)     
            }
            Err(_) => {
                None
            }
        }
    })
    .collect();


    Ok(assets)
}