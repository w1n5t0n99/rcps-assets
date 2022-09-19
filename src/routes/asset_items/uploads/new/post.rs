use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, ResponseError};
use anyhow::Context;
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use chrono::Utc;
use actix_web_flash_messages::FlashMessage;
use actix_multipart::{Multipart, Field};
use futures::{StreamExt, TryStreamExt, TryFutureExt};
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
        //let content_type = field.content_disposition();

        //let filename = content_type.get_filename().unwrap();
        let filepath = format!("./temp_files/{}.csv", uuid::Uuid::new_v4());

        // File::create is blocking operation
        let fp = filepath.clone();
        let mut f = spawn_blocking_with_tracing(move || std::fs::File::create(fp))
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

        let mut transaction = pool.begin()
            .await
            .context("Failed to acquire a Postgres connection from the pool")
            .map_err(e500)?;

        let fp = filepath.clone();
        let rows_inserted = copy_to_db(&mut transaction, fp)
            .await
            .map_err(e500)?;

        transaction
            .commit()
            .await
            .context("Failed to commit SQL transaction to store a new subscriber.")
            .map_err(e500)?;

        let fp = filepath.clone();
        spawn_blocking_with_tracing(move || std::fs::remove_file(fp))
            .await
            .map_err(e500)??;

        FlashMessage::success(format!("Assets uploaded: {}", rows_inserted)).send();
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

#[tracing::instrument(name = "Copy assets to database", skip(transaction, filepath))]
async fn copy_to_db(transaction: &mut Transaction<'_, Postgres>, filepath: String) -> Result<u64, anyhow::Error> {  
    let mut copy_in = transaction.copy_in_raw("COPY assets(asset_id, name, serial_num, brand, model, date_added) FROM STDIN (FORMAT CSV, HEADER TRUE)").await?;

    let file = tokio::fs::File::open(filepath).await?;
    copy_in.read_from(file).await?;

    let rows_inserted = copy_in.finish().await?;

    Ok(rows_inserted)
}