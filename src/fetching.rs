use crate::performance::Performance;
use std::time::Instant;
use url::Url;

pub struct FetchBytesResponse {
  pub bytes: Vec<u8>,
  pub content_type: Option<reqwest::header::HeaderValue>,
  pub performance: Performance,
}

async fn fetch_bytes(url: Url) -> Result<FetchBytesResponse, reqwest::Error> {
  let start = Instant::now();
  let url_str = url.into_string();
  let res = reqwest::get(&url_str).await?;
  let content_type = res.headers().get(reqwest::header::CONTENT_TYPE).cloned();
  let bytes = res.bytes().await?;
  let fetch_time = start.elapsed();

  Ok(FetchBytesResponse {
    bytes: bytes.to_vec(),
    content_type: content_type,
    performance: Performance {
      elapsed_ns: fetch_time.as_nanos(),
    },
  })
}

pub struct FetchDynamicImageResponse {
  pub img: image::DynamicImage,
  pub content_type: Option<reqwest::header::HeaderValue>,
  pub performance: Performance,
}

pub async fn fetch_dynimage(url: Url) -> Result<FetchDynamicImageResponse, image::ImageError> {
  let start = Instant::now();
  let response = fetch_bytes(url)
    .await
    .expect("An error occurred when downloading the source image.");
  Ok(FetchDynamicImageResponse {
    img: image::load_from_memory(&response.bytes).expect("Could not process image!"),
    content_type: response.content_type,
    performance: Performance {
      elapsed_ns: start.elapsed().as_nanos(),
    },
  })
}
