use std::time::Instant;
use url::Url;

pub struct FetchResponse {
  pub bytes: Vec<u8>,
  pub content_type: Option<reqwest::header::HeaderValue>,
  pub fetch_ns: u128,
}

pub async fn fetch_bytes(url: Url) -> Result<FetchResponse, reqwest::Error> {
  let start = Instant::now();
  let url_str = url.into_string();
  let res = reqwest::get(&url_str).await?;
  let content_type = res.headers().get(reqwest::header::CONTENT_TYPE).cloned();
  let bytes = res.bytes().await?;
  let fetch_time = start.elapsed();

  Ok(FetchResponse {
    bytes: bytes.to_vec(),
    content_type: content_type,
    fetch_ns: fetch_time.as_nanos(),
  })
}
