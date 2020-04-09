pub const MAX_IMAGE_SIZE: u32 = 2 << 11; // 4096
use image::DynamicImage;

pub fn resize(img: &DynamicImage, width: Option<u32>, height: Option<u32>) -> DynamicImage {
  if width.is_none() && height.is_none() {
    img.clone()
  } else {
    img.thumbnail(
      width.unwrap_or(MAX_IMAGE_SIZE),
      height.unwrap_or(MAX_IMAGE_SIZE),
    )
  }
}

pub fn to_bytes(
  img: &DynamicImage,
  format: image::ImageOutputFormat,
) -> Result<Vec<u8>, image::error::ImageError> {
  let mut result: Vec<u8> = Vec::new();
  img.write_to(&mut result, format)?;
  return Ok(result);
}
