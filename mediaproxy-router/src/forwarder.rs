use actix_web::http::StatusCode;
use actix_web::HttpResponse;
use mediaproxy_common::query::Query;
use url::Url;

fn internal_server_error() -> HttpResponse {
    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body("Internal server error")
}

pub fn forward(query: Query, forward_url: Url) -> HttpResponse {
    let fingerprint = query.to_fingerprint();

    let mut new_url = forward_url;
    new_url.set_path(&fingerprint);

    let res = match reqwest::blocking::get(new_url) {
        Ok(res) => res,
        Err(_) => return internal_server_error(),
    };

    let mut client_resp = HttpResponse::build(res.status());

    let bytes = match res.bytes() {
        Ok(bytes) => bytes,
        Err(_) => return internal_server_error(),
    };

    client_resp.body(bytes)
}
