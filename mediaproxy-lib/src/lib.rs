pub mod query {
  use base64::encode_config;
  use serde::{Deserialize, Serialize};

  #[derive(Serialize, Deserialize)]
  pub enum ImageProcessingOutput {
    #[serde(rename = "jpeg")]
    #[serde(alias = "jpg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    WebP,
    #[serde(rename = "gif")]
    Gif,
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
      encode_config(
        json,
        base64::Config::new(base64::CharacterSet::UrlSafe, false),
      )
    }
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
        height: None,
      };

      assert_eq!(query.to_fingerprint(), String::from("eyJzb3VyY2UiOiJodHRwczovL2R1bW15aW1hZ2UuY29tLzYwMHg0MDAvMDAwL2ZmZiIsIndpZHRoIjpudWxsLCJoZWlnaHQiOm51bGwsImZvcm1hdCI6ImpwZWcifQ"));
    }
  }
}
