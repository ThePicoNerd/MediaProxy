use image::{DynamicImage, ImageOutputFormat};
use num::clamp;
use serde::Deserialize;
use std::time::Instant;

use crate::performance::Performance;

pub const MAX_IMAGE_SIZE: u32 = 2 << 11; // 4096

#[derive(Deserialize)]
pub enum ImageProcessingOutput {
    #[serde(rename = "jpeg")]
    #[serde(alias = "jpg")]
    Jpeg,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "gif")]
    Gif,
}

pub struct ResizeResponse {
    pub img: DynamicImage,
    pub performance: Performance,
}

pub fn resize(img: &DynamicImage, width: Option<u32>, height: Option<u32>) -> ResizeResponse {
    let start = Instant::now();
    let resized = match (width, height) {
        (None, None) => img.clone(),
        (width, height) => {
            let nwidth = clamp(width.unwrap_or(MAX_IMAGE_SIZE), 1, MAX_IMAGE_SIZE);
            let nheight = clamp(height.unwrap_or(MAX_IMAGE_SIZE), 1, MAX_IMAGE_SIZE);
            img.thumbnail(nwidth, nheight)
        }
    };
    ResizeResponse {
        img: resized,
        performance: Performance {
            elapsed_ns: start.elapsed().as_nanos(),
        },
    }
}

pub fn to_bytes(
    img: &DynamicImage,
    format: ImageProcessingOutput,
) -> Result<Vec<u8>, image::error::ImageError> {
    match format {
        ImageProcessingOutput::Jpeg => {
            let mut result: Vec<u8> = Vec::new();
            img.write_to(&mut result, ImageOutputFormat::Jpeg(80))?;
            Ok(result)
        }
        ImageProcessingOutput::Png => {
            let mut result: Vec<u8> = Vec::new();
            img.write_to(&mut result, ImageOutputFormat::Png)?;
            Ok(result)
        }
        ImageProcessingOutput::Gif => {
            let mut result: Vec<u8> = Vec::new();
            img.write_to(&mut result, ImageOutputFormat::Gif)?;
            Ok(result)
        }
    }
}
