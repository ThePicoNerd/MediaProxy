use actix_web::client::Client;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use mediaproxy_lib::query::Query;
use clap::Arg;
use url::Url;

async fn cache(req: HttpRequest, query: web::Json<Query>, url: web::Data<Url>, client: web::Data<Client>) -> Result<HttpResponse, actix_web::Error> {
    let new_url = url.get_ref().clone();
    let json = serde_json::to_string(&query.into_inner()).unwrap();
    let forwarded_req = client.request_from(new_url.as_str(), req.head()).no_decompress();
    let res = forwarded_req.send_body(json).await.map_err(actix_web::Error::from)?;
    let mut client_resp = HttpResponse::build(res.status());

    for (header_name, header_value) in
        res.headers().iter().filter(|(h, _)| *h != "connection")
    {
        client_resp.header(header_name.clone(), header_value.clone());
    }

    Ok(client_resp.streaming(res))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = clap::App::new("MediaProxy Router")
        .arg(
            Arg::with_name("forward_addr").long("forward")
                .takes_value(true)
                .value_name("FWD ADDR")
                .required(true),
        )
        .arg(
            Arg::with_name("listen_addr").long("listen")
                .takes_value(true)
                .value_name("LISTEN ADDR")
                .required(false).default_value("127.0.0.1:8080"),
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
            .data(Client::new())
            .data(forward_url.clone())
            .service(web::resource("/").route(web::post().to(cache)))
    })
    .bind(listen_addr)?
    .run()
    .await
}
