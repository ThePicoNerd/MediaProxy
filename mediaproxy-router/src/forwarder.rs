use actix_web::error::ErrorInternalServerError;
use actix_web::HttpResponse;
use mediaproxy_common::query::Query;
use url::Url;

pub fn forward(query: Query, forward_url: Url) -> actix_web::Result<HttpResponse> {
    let fingerprint = query.to_fingerprint();

    let mut new_url = forward_url;
    new_url.set_path(&fingerprint);

    let res = match reqwest::blocking::get(new_url) {
        Ok(res) => res,
        Err(error) => return Err(ErrorInternalServerError(error)),
    };

    let mut client_resp = HttpResponse::build(res.status());

    let bytes = match res.bytes() {
        Ok(bytes) => bytes,
        Err(error) => return Err(ErrorInternalServerError(error)),
    };

    Ok(client_resp.body(bytes))
}
