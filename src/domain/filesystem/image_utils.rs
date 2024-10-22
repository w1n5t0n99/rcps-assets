use anyhow::{anyhow, Context};
use image::EncodableLayout;

use crate::domain::filesystem::models::ExtensionType;

use super::models::Extension;


pub fn process_image(data: Vec<u8>, ext: Extension) -> anyhow::Result<(Vec<u8>, Extension)> {
    assert_eq!(ext.ext_type(), ExtensionType::Image);

    if ext == Extension::SVG || ext == Extension::WEBP {
        return Ok((data, ext))
    }

    let img = image::load_from_memory(&data)
        .context("loading image failed")?;

    // Create the WebP encoder for the above image
    let encoder = webp::Encoder::from_image(&img)
        .map_err(|_| anyhow!("webp encoder failed to initialize from image"))?;

    // Encode the image at a specified quality 0-100
    let webp = encoder.encode(75f32);

    // TODO: more effecient way than copying to vec
    Ok((webp.as_bytes().into(), Extension::WEBP))
}