use crate::fetching;
use crate::imageops;
use actix_web::http::header::ContentType;
use custom_error::custom_error;
use serde::Deserialize;
use url::Url;

use imageops::ImageProcessingOutput;

#[derive(Deserialize)]
pub struct ImageProcessingQuery {
    pub source: String,
    #[serde(alias = "w")]
    pub width: Option<u32>,
    #[serde(alias = "h")]
    pub height: Option<u32>,
    pub format: ImageProcessingOutput,
}

custom_error! {pub ApiError
  FetchError{source: fetching::FetchError} = "Something went wrong when fetching the source image.",
  ImageError{source: image::error::ImageError} = "Something went wrong when processing the image.",
  InputError{source: url::ParseError} = "Invalid input!",
}

pub struct ApiResponse {
    pub bytes: Vec<u8>,
    pub content_type: ContentType,
}

pub fn handle_query(query: ImageProcessingQuery) -> Result<ApiResponse, ApiError> {
    let url = Url::parse(query.source.as_str())?;
    let original = fetching::fetch_dynimage(url)?;

    let result = imageops::resize(&original.img, query.width, query.height);

    let media_type = imageops::media_type(&query.format);

    Ok(ApiResponse {
        bytes: imageops::to_bytes(&result.img, query.format)?,
        content_type: ContentType(media_type),
    })
}
