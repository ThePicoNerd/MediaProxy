use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use clap::Arg;
use mediaproxy_common::query::Query;
use url::Url;

mod forwarder;

fn router(query: web::Json<Query>, url: web::Data<Url>) -> HttpResponse {
    forwarder::forward(query.into_inner(), url.get_ref().clone())
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = clap::App::new("MediaProxy Router")
        .arg(
            Arg::with_name("forward_addr")
                .long("forward")
                .takes_value(true)
                .value_name("FWD ADDR")
                .required(true),
        )
        .arg(
            Arg::with_name("listen_addr")
                .long("listen")
                .takes_value(true)
                .value_name("LISTEN ADDR")
                .required(false)
                .default_value("127.0.0.1:8080"),
        )
        .get_matches();

    let listen_addr = matches.value_of("listen_addr").unwrap();
    let forward_addr = matches.value_of("forward_addr").unwrap();
    let forward_url = Url::parse(&format!("http://{}", forward_addr)).unwrap();

    println!("Binding {}", listen_addr);
    println!("Forwarding requests to {}", forward_addr);

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .data(forward_url.clone())
            .service(web::resource("/").route(web::post().to(router)))
    })
    .bind(listen_addr)?
    .run()
    .await
}
