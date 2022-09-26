use actix_web::http::Uri;
use serde_aux::prelude::deserialize_number_from_string;


#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub enum Paginate {
    #[serde(rename = "next")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    Next(i32),
    #[serde(rename = "prev")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    Prev(i32),
}

