use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use custom_error::custom_error;
use mime;
use serde::Deserialize;
use url::Url;

mod fetching;
mod optimizer;
mod performance;

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

struct ApiResponse {
    bytes: Vec<u8>,
    content_type: ContentType,
}

async fn handle_query(query: ImageProcessingQuery) -> Result<ApiResponse, ApiError> {
    let url = Url::parse(query.source.as_str())?;
    let original = fetching::fetch_dynimage(url).await?;

    let result = optimizer::resize(&original.img, query.width, query.height);

    let (format, content_type) = match &query.format {
        ImageProcessingOutput::Jpeg => (
            image::ImageOutputFormat::Jpeg(80),
            ContentType(mime::IMAGE_JPEG),
        ),
        ImageProcessingOutput::Png => (image::ImageOutputFormat::Png, ContentType(mime::IMAGE_PNG)),
        ImageProcessingOutput::Gif => (image::ImageOutputFormat::Gif, ContentType(mime::IMAGE_GIF)),
    };

    Ok(ApiResponse {
        bytes: optimizer::to_bytes(&result.img, format)?,
        content_type: content_type,
    })
}

#[get("/")]
async fn index(query: web::Query<ImageProcessingQuery>) -> HttpResponse {
    return match handle_query(query.into_inner()).await {
        Ok(result) => HttpResponse::build(StatusCode::OK)
            .set(result.content_type)
            .body(result.bytes),
        Err(error) => {
            let (status, body) = match error {
                ApiError::FetchError { source } => (
                    StatusCode::BAD_REQUEST,
                    match source {
                        fetching::FetchError::MaxSizeExceeded => "The source image is too large.",
                        _ => "Could not fetch source image!",
                    },
                ),
                ApiError::InputError { source: _ } => {
                    (StatusCode::BAD_REQUEST, "The input is malformed.")
                }
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unknown error occurred.",
                ),
            };
            HttpResponse::build(status).body(body)
        }
    };
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = std::env::var("ADDRESS").unwrap_or(String::from("127.0.0.1:8080"));
    println!("Binding {}", address);
    HttpServer::new(|| App::new().service(index))
        .bind(address)?
        .run()
        .await
}
