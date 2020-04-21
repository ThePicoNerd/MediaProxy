use crate::fetching;
use crate::imageops;
use actix_web::http::header::ContentType;
use custom_error::custom_error;
use mediaproxy_lib::query::Query;
use url::Url;

custom_error! {pub HandleQueryError
  FetchError{source: fetching::FetchError} = "Something went wrong when fetching the source image.",
  ImageError{source: image::error::ImageError} = "Something went wrong when processing the image.",
  InputError{source: url::ParseError} = "Invalid input!",
}

pub struct Response {
    pub bytes: Vec<u8>,
    pub content_type: ContentType,
}

pub fn handle_query(query: Query) -> Result<Response, HandleQueryError> {
    let url = Url::parse(query.source.as_str())?;
    let original = fetching::fetch_dynimage(url)?;

    let result = imageops::resize(&original.img, query.width, query.height);

    let media_type = imageops::get_media_type(&query.format);

    Ok(Response {
        bytes: imageops::to_bytes::image(&result.img, query.format)?,
        content_type: ContentType(media_type),
    })
}
