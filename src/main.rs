#![feature(or_patterns)]
#![feature(ip)]

use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use handler::handle_query;

mod fetching;
mod handler;
mod imageops;
mod performance;

fn mediaproxy(query: web::Json<handler::Query>) -> HttpResponse {
    let fingerprint = query.to_fingerprint();

    match handle_query(query.into_inner()) {
        Ok(result) => HttpResponse::build(StatusCode::OK)
            .set_header("x-mediaproxy-fingerprint", fingerprint)
            .set(result.content_type)
            .body(result.bytes),
        Err(error) => {
            let (status, body) = match error {
                handler::HandleQueryError::FetchError { source } => (
                    StatusCode::BAD_REQUEST,
                    match source {
                        fetching::FetchError::MaxSizeExceeded => "The source image is too large.",
                        _ => "Could not fetch source image!",
                    },
                ),
                handler::HandleQueryError::InputError { .. } => {
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
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/").route(web::post().to(mediaproxy)))
    })
    .bind(address)?
    .run()
    .await
}
