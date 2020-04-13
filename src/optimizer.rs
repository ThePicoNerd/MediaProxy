use crate::fetching;
use crate::imageops;
use actix_web::http::header::ContentType;
use custom_error::custom_error;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
enum ImageProcessingOutput {
    #[serde(rename = "jpeg")]
    #[serde(alias = "jpg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "gif")]
    Gif,
}

#[derive(Deserialize)]
pub struct ImageProcessingQuery {
    source: String,
    #[serde(alias = "w")]
    width: Option<u32>,
    #[serde(alias = "h")]
    height: Option<u32>,
    format: ImageProcessingOutput,
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

pub async fn handle_query(query: ImageProcessingQuery) -> Result<ApiResponse, ApiError> {
    let url = Url::parse(query.source.as_str())?;
    let original = fetching::fetch_dynimage(url).await?;

    let result = imageops::resize(&original.img, query.width, query.height);

    let (format, content_type) = match &query.format {
        ImageProcessingOutput::Jpeg => (
            image::ImageOutputFormat::Jpeg(80),
            ContentType(mime::IMAGE_JPEG),
        ),
        ImageProcessingOutput::Png => (image::ImageOutputFormat::Png, ContentType(mime::IMAGE_PNG)),
        ImageProcessingOutput::Gif => (image::ImageOutputFormat::Gif, ContentType(mime::IMAGE_GIF)),
    };

    Ok(ApiResponse {
        bytes: imageops::to_bytes(&result.img, format)?,
        content_type,
    })
}
