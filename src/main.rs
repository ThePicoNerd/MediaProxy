use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use optimizer::{ApiError, ImageProcessingQuery};

mod fetching;
mod imageops;
mod optimizer;
mod performance;

fn mediaproxy(query: web::Json<ImageProcessingQuery>) -> HttpResponse {
    match optimizer::handle_query(query.into_inner()) {
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
                ApiError::InputError { .. } => (StatusCode::BAD_REQUEST, "The input is malformed."),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An unknown error occurred.",
                ),
            };
            println!("{}", body);
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

// use actix_web::{
//     middleware, web, App, HttpResponse, HttpServer,
// };
// use serde::{Deserialize, Serialize};

// #[derive(Debug, Serialize, Deserialize)]
// struct MyObj {
//     name: String,
//     number: i32,
// }

// /// This handler uses json extractor
// async fn index(item: web::Json<MyObj>) -> HttpResponse {
//     println!("model: {:?}", &item);
//     HttpResponse::Ok().json(item.0) // <- send response
// }

// #[actix_rt::main]
// async fn main() -> std::io::Result<()> {
//     std::env::set_var("RUST_LOG", "actix_web=info");

//     HttpServer::new(|| {
//         App::new()
//             // enable logger
//             .wrap(middleware::Logger::default())
//             .data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
//             .service(web::resource("/").route(web::post().to(index)))
//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }