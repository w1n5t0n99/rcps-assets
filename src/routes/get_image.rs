use actix_web::web;
use actix_files::NamedFile;


pub async fn get_image(path: web::Path<String>) -> actix_web::Result<NamedFile> {
    let file_path = format!("static/{}.png", path.into_inner());

    Ok(NamedFile::open(file_path)?)
}