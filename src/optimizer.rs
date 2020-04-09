pub const MAX_IMAGE_SIZE: u32 = 2 << 11; // 4096
use image::DynamicImage;

use std::time::Instant;
use crate::performance::Performance;

pub struct ResizeResponse {
  pub img: DynamicImage,
  pub performance: Performance,
}

pub fn resize(img: &DynamicImage, width: Option<u32>, height: Option<u32>) -> ResizeResponse {
    let start = Instant::now();
    let resized = img.thumbnail(
      width.unwrap_or(MAX_IMAGE_SIZE),
      height.unwrap_or(MAX_IMAGE_SIZE),
    );
    ResizeResponse {
      img: resized,
      performance: Performance {
        elapsed_ns: start.elapsed().as_nanos(),
      }
    }
}

pub fn to_bytes(
  img: &DynamicImage,
  format: image::ImageOutputFormat,
) -> Result<Vec<u8>, image::error::ImageError> {
  let mut result: Vec<u8> = Vec::new();
  img.write_to(&mut result, format)?;
  Ok(result)
}
