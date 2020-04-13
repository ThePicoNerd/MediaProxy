use actix_web::http::StatusCode;
use actix_web::{get, web, App, HttpResponse, HttpServer};
use optimizer::{ApiError, ImageProcessingQuery};

mod fetching;
mod imageops;
mod optimizer;
mod performance;

#[get("/")]
async fn index(query: web::Query<ImageProcessingQuery>) -> HttpResponse {
    match optimizer::handle_query(query.into_inner()).await {
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
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let address = std::env::var("ADDRESS").unwrap_or_else(|_| String::from("127.0.0.1:8080"));
    println!("Binding {}", address);
    HttpServer::new(|| App::new().service(index))
        .bind(address)?
        .run()
        .await
}
