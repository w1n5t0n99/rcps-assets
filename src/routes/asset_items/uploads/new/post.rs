use actix_web::{web, HttpResponse};
use anyhow::{Context, anyhow};
use sqlx::{PgPool, Postgres, Transaction};
use validator::Validate;
use actix_web_flash_messages::FlashMessage;
use actix_multipart::{Multipart, Field};
use futures::{StreamExt, TryStreamExt};
use std::io::Write;
use mime::Mime;

use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::see_other;
use crate::domain::Asset;
use crate::errors::Error;


pub struct UploadPayload {
    pub data: Vec<u8>,
    pub filename: String, 
    pub tmp_path: String,
    pub mime: Mime,
}


#[tracing::instrument(
    name = "Upload Assets from file",
    skip(payload, pool),
)]
pub async fn upload_assets(
    mut payload: Multipart,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, Error> {

    let mut upload_payload = None;
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

        // should only be one field in upload form
        upload_payload = save_field_to_temp_file(&mut field).await?;
    }
    
    let upload_payload = upload_payload.ok_or_else(|| {
        FlashMessage::error(format!("No file found")).send();
        Error::from_redirect(anyhow!("Failed to upload file"), "/asset_items")
    })?;

    if upload_payload.mime.to_string().eq_ignore_ascii_case("text/csv") == false {
        // remove temp file if invalid file
        let fp = upload_payload.tmp_path.clone();
        spawn_blocking_with_tracing(move || std::fs::remove_file(fp))
            .await??;

        FlashMessage::error(format!("Invalid uploaded file type")).send();
        return Err(Error::from_redirect(anyhow!("Invalid uploaded file type: {}", upload_payload.mime.to_string()), "/asset_items"));
    }

    let fp = upload_payload.tmp_path.clone();
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
    let fp = upload_payload.tmp_path.clone();
    spawn_blocking_with_tracing(move || std::fs::remove_file(fp))
        .await??;


    insert_upload_status(&pool, upload_payload.filename, total as i32, skipped as i32, inserted).await?;

    FlashMessage::success("Asset file uploaded successfully.").send();
    Ok(see_other("/asset_items/uploads"))
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

#[tracing::instrument(name = "Save multipart field to temp file", skip_all)]
async fn save_field_to_temp_file(field: &mut Field) -> Result<Option<UploadPayload>, Error> {
    let content_type = field.content_disposition();
    let filename = content_type
        .get_filename()
        .ok_or_else(|| anyhow!("upload: missing filename"))?
        .to_owned();

    let filepath = format!("./temp_files/{}.csv", uuid::Uuid::new_v4());
    // File::create is blocking operation
    let fp = filepath.clone();
    let mut f = spawn_blocking_with_tracing(move || std::fs::File::create(fp))
        .await??;

    let mut buf: Vec<u8> = Vec::with_capacity(1024);
    while let Some(chunk) = field.next().await {
        let data = chunk.context("Could not read multipart chunk")?;
        buf.extend(data.to_owned());

        // filesystem operations are blocking
        f = spawn_blocking_with_tracing(move|| f.write_all(&data).map(|_| f))
        .await??; 
    }

    if buf.is_empty() {
        spawn_blocking_with_tracing(move || std::fs::remove_file(filepath))
            .await??;
        return Ok(None);
    }

    Ok(
        Some(
            UploadPayload {
                data: buf,
                filename: filename,
                tmp_path: filepath,
                mime: field.content_type().to_owned(),
            }
        )
    )
}

#[tracing::instrument(name = "Saving upload details into database", skip_all)]
async fn insert_upload_status( pool: &PgPool, uploaded_file: String, total: i32, skipped: i32, added: i32) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO uploads (uploaded_file, upload_date, total, skipped, added)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        uploaded_file,
        chrono::Utc::now(),
        total,
        skipped,
        added,
    )
    .execute(pool)
    .await?;
    
    Ok(())
}