#![feature(or_patterns)]
#![feature(ip)]

use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use handler::handle_query;
use mediaproxy_lib::query::Query;
use clap::Arg;

mod fetching;
mod handler;
mod imageops;
mod performance;

fn mediaproxy(query: web::Json<Query>) -> HttpResponse {
    match handle_query(query.into_inner()) {
        Ok(result) => HttpResponse::build(StatusCode::OK)
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
    let matches = clap::App::new("MediaProxy")
        .arg(
            Arg::with_name("listen_addr")
                .takes_value(true)
                .value_name("LISTEN ADDR")
                .required(false).default_value("127.0.0.1:8080"),
        )
        .get_matches();
        let listen_addr = matches.value_of("listen_addr").unwrap();
    println!("Binding {}", listen_addr);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/").route(web::post().to(mediaproxy)))
    })
    .bind(listen_addr)?
    .run()
    .await
}
