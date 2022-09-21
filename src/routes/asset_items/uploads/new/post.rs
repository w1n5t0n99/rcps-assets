use actix_web::{web, HttpResponse};
use anyhow::{Context, anyhow};
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use actix_web_flash_messages::FlashMessage;
use actix_multipart::{Multipart, Field};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::see_other;
use crate::domain::Asset;
use crate::errors::AssetsError;


#[tracing::instrument(
    name = "Upload Assets from file",
    skip(payload, pool),
)]
pub async fn upload_assets(
    mut payload: Multipart,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, AssetsError> {

    while let Ok(Some(mut field)) = payload.try_next().await {
        /*
        let content_type = field.content_disposition();
        println!("{}", field.name());
        println!("CONTENT TYPE: {}", content_type);
        println!("FILE NAME: {}", content_type.get_filename().unwrap());
        println!("UPLOAD NAME FIELD: {}", content_type.get_name().unwrap());
        println!("MIME TYPE: {}", field.content_type());
        println!("FIELD: {:?}", field);
        */
        
        check_form_content_type(&field)?;

        //FlashMessage::error(format!("Invalid uploaded file type: {}", field.content_type().to_string())).send();

        let filepath = format!("./temp_files/{}.csv", uuid::Uuid::new_v4());

        // File::create is blocking operation
        let fp = filepath.clone();
        let mut f = spawn_blocking_with_tracing(move || std::fs::File::create(fp))
            .await??;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.context("Could not read multipart chunk")?;

            // filesystem operations are blocking
            f = spawn_blocking_with_tracing(move|| f.write_all(&data).map(|_| f))
                .await??;       
        }

        let fp = filepath.clone();
        let (assets_res, assets_err): (Vec<_>, Vec<_>) = spawn_blocking_with_tracing(move|| load_assets_from_csv(fp))
            .await??
            .into_iter()
            .partition(Result::is_ok);

        let total = assets_res.len() + assets_err.len();
        let mut inserted = 0;
        let mut skipped = assets_err.len();

        for asset in assets_res {
            // Assets should be guaranteed to be OK
            let a = asset.unwrap();
            
            match insert_asset(&pool, &a).await {
                Ok(_) => {inserted = inserted + 1 },
                Err(_) => {skipped = skipped + 1 },
            }
        }


        // After parsing the data we can remove the temp file
        let fp = filepath.clone();
        spawn_blocking_with_tracing(move || std::fs::remove_file(fp))
            .await??;

       FlashMessage::success(format!("Assets total: {} uploaded: {} skipped: {}", total, inserted, skipped)).send();
    }

    Ok(see_other("/asset_items/uploads"))
}

fn check_form_content_type(field: &Field) -> Result<(), AssetsError> {
    match field.content_type().to_string().eq_ignore_ascii_case("text/csv")  {
        true => Ok(()),
        false => {
            Err(AssetsError::Conflict("Invalid uploaded file type".to_string()))
        }
    }
}


fn load_assets_from_csv(filepath: String) -> Result<Vec<Result<Asset, anyhow::Error>>, csv::Error> {  
    let mut rdr = csv::Reader::from_path(filepath)?;
    
    let assets: Vec<Result<Asset, anyhow::Error>> = rdr.deserialize().map(|row| {
        match row {
            Ok(r) => { 
                let a: Asset = r;
                match a.validate() {
                    Ok(_) => Ok(a),
                    Err(e) => Err(anyhow!(e)),
                }
            },
            Err(e) => Err(anyhow!(e)),
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

#[tracing::instrument(name = "Saving new asset details into database", skip(pool, asset))]
async fn insert_asset( pool: &PgPool, asset: &Asset) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO assets (asset_id, name, serial_num, model, brand, date_added)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        asset.asset_id,
        asset.name,
        asset.serial_num,
        asset.model,
        asset.brand,
        asset.date_added,
    )
    .execute(pool)
    .await?;
    
    Ok(())
}