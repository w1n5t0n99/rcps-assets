use mime::Mime;
use actix_multipart::{Field, Multipart};
use futures::{StreamExt, TryStreamExt};
use sqlx::PgPool;
use validator::Validate;
use std::path::Path;
use std::fs::File;
use std::io::Write;
use anyhow::Context;

use crate::telemetry::spawn_blocking_with_tracing;
use crate::utils::error_chain_fmt;
use crate::domain::Asset;


pub struct UploadPayload {
    pub data: Vec<u8>,
    pub filename: String,   // uploaded file name
    hash: blake3::Hash,
    pub tmp_path: String,   // path to tmp file where uploaded data copied to, deleted at end of processing
    pub mime: Mime,
}

#[derive(thiserror::Error)]
pub enum UploadError {
    #[error("Invalid upload data")]
    ValidationError,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for UploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Debug, serde::Serialize)]
pub struct UploadResponse {
    pub id: i32,
    pub hash: String,
    pub filename: String,
    pub total: i32,
    pub inserted: i32,
    pub skipped: i32,
}

#[tracing::instrument(
    name = "Save multipart field to temp file",
    skip_all,
)]
pub async fn save_field_to_temp_file(field: &mut Field) -> Result<Option<UploadPayload>, UploadError> {
    let content_type = field.content_disposition();
    let filename = content_type
        .get_filename()
        .ok_or(UploadError::ValidationError)?
        .to_owned();

    println!("+++++++++++++{:?}+++++++++++++", content_type.get_filename_ext());

    // create temporary file
    let (f, filepath) = spawn_blocking_with_tracing(move || {
        let uuid_fp = format!("./temp_files/{}.csv", uuid::Uuid::new_v4());
        let filepath = Path::new(&uuid_fp);
        // TODO - check if file already exists
        (File::create(filepath), uuid_fp)
    })
    .await
    .context("blocking thread error")?;

    let mut f = f.context("tmp file - create file error")?;
    let mut hasher = blake3::Hasher::new();
    let mut buf: Vec<u8> = Vec::with_capacity(1024); // TODO - can we estimate a real size from the multipart?

    // copy multipart field into file
    while let Some(chunk) = field.next().await {
        let bytes = chunk.context("multipart read error")?;

        hasher.update(&bytes);
        buf.extend(bytes.to_owned());
        // write data to file
        f = spawn_blocking_with_tracing(move || f.write_all(&bytes).map(|_| f))
            .await
            .context("blocking thread error")?
            .context("tmp file write error")?;
    }

    if buf.is_empty() {
        tracing::warn!("empty multipart file");
        spawn_blocking_with_tracing(move || std::fs::remove_file(filepath))
            .await
            .context("blocking thread error")?
            .context("tmp file delete error")?;
        return Ok(None);
    }

    Ok(Some(UploadPayload {
        data: buf,
        filename,
        tmp_path: filepath,
        hash: hasher.finalize(),
        mime: field.content_type().to_owned(),
    }))
}

fn parse_payload_as_csv<D>(payload: UploadPayload) -> Result<Vec<Result<D, csv::Error>>, csv::Error> where 
    D: serde::de::DeserializeOwned + validator::Validate
{
    let mut rdr = csv::Reader::from_path(&payload.tmp_path)?;

    let domain_vec: Vec<Result<D, csv::Error>> = rdr.deserialize().map(|row| {
        match row {
            Ok(r) => { let d: D = r; Ok(d) },
            Err(e) => Err(e),
        }
    })
    .collect();

    Ok(domain_vec)
}


#[tracing::instrument(
    name = "insert payload into database as assets",
    skip_all,
)]
pub async fn insert_payload_as_assets(payload: UploadPayload, pool: &PgPool) -> Result<Option<UploadResponse>, UploadError> {
    let (assets, errors): (Vec<_>, Vec<_>) = spawn_blocking_with_tracing(move|| parse_payload_as_csv::<Asset>(payload))
        .await
        .context("blocking thread error")?
        .context("error parsing payload as csv")?
        .into_iter()
        .partition(Result::is_ok);

    let total_count: i32 = assets.len() as i32 + errors.len() as i32;
    let mut in_count = 0;
    let mut err_count = errors.len() as i32;
    
    for asset in assets {
        let a = asset.unwrap();
        if a.validate().is_err() {
            err_count = err_count + 1;
            continue;
        }

        
        
    }

    todo!()
}

#[tracing::instrument(
    name = "insert payload into database as users",
    skip_all,
)]
pub async fn insert_payload_as_users(payload: UploadPayload) -> Result<Option<UploadResponse>, UploadError> {
    // TODO - maybe pass database operation as function pointer for flexability
    todo!()
}