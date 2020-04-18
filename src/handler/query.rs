use crate::fetching;
use crate::imageops;
use actix_web::http::header::ContentType;
use base64::encode_config;
use custom_error::custom_error;
use serde::{Serialize, Deserialize};
use url::Url;

use imageops::ImageProcessingOutput;

fn b64_config() -> base64::Config {
    base64::Config::new(base64::CharacterSet::UrlSafe, false)
}

custom_error! {pub QueryFingerprintConversionError
    JsonError{source: serde_json::Error} = "Something went wrong when (de)serializing JSON.",
    Base64Error{source: base64::DecodeError} = "Something went wrong when decoding Base64.",
    UnicodeError{source: std::str::Utf8Error} = "Could not convert byte array to string!"
  }

#[derive(Serialize, Deserialize)]
pub struct Query {
    pub source: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub format: ImageProcessingOutput,
}

impl Query {
    pub fn to_fingerprint(self: &Self) -> String {
        let json = serde_json::to_string(&self).unwrap();
        encode_config(json, b64_config())
    }

    pub fn from_fingerprint(fingerprint: String) -> Result<Query, QueryFingerprintConversionError> {
        let bytes = base64::decode_config(fingerprint, b64_config())?;
        let json = std::str::from_utf8(&bytes)?;
        let query: Query = serde_json::from_str(json)?;
        Ok(query)
    }
}

custom_error! {pub HandleQueryError
  FetchError{source: fetching::FetchError} = "Something went wrong when fetching the source image.",
  ImageError{source: image::error::ImageError} = "Something went wrong when processing the image.",
  InputError{source: url::ParseError} = "Invalid input!",
}

pub struct Response {
    pub bytes: Vec<u8>,
    pub content_type: ContentType,
}

pub fn handle_query(query: Query) -> Result<Response, HandleQueryError> {
    let url = Url::parse(query.source.as_str())?;
    let original = fetching::fetch_dynimage(url)?;

    let result = imageops::resize(&original.img, query.width, query.height);

    let media_type = imageops::media_type(&query.format);

    Ok(Response {
        bytes: imageops::to_bytes(&result.img, query.format)?,
        content_type: ContentType(media_type),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_to_fingerprint() {
        let query = Query {
            source: String::from("https://dummyimage.com/600x400/000/fff"),
            format: ImageProcessingOutput::Jpeg,
            width: None,
            height: None
        };

        assert_eq!(query.to_fingerprint(), String::from("eyJzb3VyY2UiOiJodHRwczovL2R1bW15aW1hZ2UuY29tLzYwMHg0MDAvMDAwL2ZmZiIsIndpZHRoIjpudWxsLCJoZWlnaHQiOm51bGwsImZvcm1hdCI6ImpwZWcifQ"));
    }

    #[test]
    fn fingerprint_to_query() {
        let fingerprint = String::from("eyJzb3VyY2UiOiJodHRwczovL2R1bW15aW1hZ2UuY29tLzYwMHg0MDAvMDAwL2ZmZiIsIndpZHRoIjpudWxsLCJoZWlnaHQiOm51bGwsImZvcm1hdCI6ImpwZWcifQ");
        let query = Query::from_fingerprint(fingerprint).unwrap();
        assert_eq!(query.source, "https://dummyimage.com/600x400/000/fff");
    }

    #[test]
    fn invalid_fingerprint() {
        let fingerprint = String::from("bruh"); // Perfectly fine base 64, not so fine JSON.
        let query = Query::from_fingerprint(fingerprint);
        assert_eq!(query.is_err(), true);
    }
}