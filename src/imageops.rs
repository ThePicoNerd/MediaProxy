use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use libwebp_sys::WebPEncodeRGB;
use num::clamp;
use serde::{Deserialize, Serialize};
use std::os::raw::{c_float, c_int};
use std::time::Instant;

use crate::performance::Performance;

pub const MAX_IMAGE_SIZE: u32 = 2 << 11; // 4096

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

pub struct ResizeResponse {
    pub img: DynamicImage,
    pub performance: Performance,
}

pub fn resize(img: &DynamicImage, width: Option<u32>, height: Option<u32>) -> ResizeResponse {
    let start = Instant::now();
    let nwidth = clamp(width.unwrap_or_else(|| img.width()), 1, MAX_IMAGE_SIZE);
    let nheight = clamp(height.unwrap_or_else(|| img.height()), 1, MAX_IMAGE_SIZE);
    let resized = img.thumbnail(nwidth, nheight);
    ResizeResponse {
        img: resized,
        performance: Performance {
            elapsed_ns: start.elapsed().as_nanos(),
        },
    }
}

fn to_bytes_webp(img: &DynamicImage, quality: u16) -> Result<Vec<u8>, image::ImageError> {
    let (width, height) = img.dimensions();
    let stride = width * 3;
    let mut output: *mut u8 = std::ptr::null_mut();
    unsafe {
        let length = WebPEncodeRGB(
            img.to_bytes().as_slice().as_ptr(),
            width as c_int,
            height as c_int,
            stride as c_int,
            quality as c_float,
            &mut output,
        );
        let vec = Vec::from_raw_parts(output, length, length);
        Ok(vec)
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
        ImageProcessingOutput::WebP => {
            let bytes = to_bytes_webp(img, 80)?;
            Ok(bytes)
        }
        ImageProcessingOutput::Gif => {
            let mut result: Vec<u8> = Vec::new();
            img.write_to(&mut result, ImageOutputFormat::Gif)?;
            Ok(result)
        }
    }
}
