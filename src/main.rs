use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpResponse, HttpServer};
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
    width: Option<u32>,
    height: Option<u32>,
    format: ImageProcessingOutput,
}

#[get("/")]
async fn index(query: web::Query<ImageProcessingQuery>) -> HttpResponse {
    let url = Url::parse(query.source.as_str()).expect("Invalid url!");
    let original = fetching::fetch_dynimage(url).await.expect("Could not fetch image!");

    let result = optimizer::resize(&original.img, query.width, query.height);

    let (format, content_type) = match &query.format {
        ImageProcessingOutput::Jpeg => (
            image::ImageOutputFormat::Jpeg(80),
            ContentType(mime::IMAGE_JPEG),
        ),
        ImageProcessingOutput::Png => (image::ImageOutputFormat::Png, ContentType(mime::IMAGE_PNG)),
        ImageProcessingOutput::Gif => (image::ImageOutputFormat::Gif, ContentType(mime::IMAGE_GIF)),
    };

    let bytes = optimizer::to_bytes(&result.img, format).expect("Could not export optimized image.");
    return HttpResponse::build(StatusCode::OK)
        .set(content_type)
        .body(bytes);
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
