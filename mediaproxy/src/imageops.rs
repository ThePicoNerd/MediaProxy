use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use libwebp_sys::WebPEncodeRGB;
use mediaproxy_lib::query::ImageProcessingOutput;
use num::clamp;
use std::os::raw::{c_float, c_int};
use std::str::FromStr;
use std::time::Instant;

use crate::performance::Performance;

pub const MAX_IMAGE_SIZE: u32 = 2 << 11; // 4096

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

pub fn get_media_type(output: &ImageProcessingOutput) -> mime::Mime {
    match output {
        ImageProcessingOutput::Jpeg => mime::IMAGE_JPEG,
        ImageProcessingOutput::Png => mime::IMAGE_PNG,
        ImageProcessingOutput::WebP => mime::Mime::from_str("image/webp").unwrap(),
        ImageProcessingOutput::Gif => mime::IMAGE_GIF,
    }
}

pub mod to_bytes {
    use super::*;

    fn default_image_format(
        img: &DynamicImage,
        format: ImageOutputFormat,
    ) -> Result<Vec<u8>, image::ImageError> {
        let mut result: Vec<u8> = Vec::new();
        img.write_to(&mut result, format)?;
        Ok(result)
    }

    fn webp(img: &DynamicImage, quality: u16) -> Result<Vec<u8>, image::ImageError> {
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

    pub fn image(
        img: &DynamicImage,
        format: ImageProcessingOutput,
    ) -> Result<Vec<u8>, image::error::ImageError> {
        match format {
            ImageProcessingOutput::Jpeg => default_image_format(img, ImageOutputFormat::Jpeg(80)),
            ImageProcessingOutput::Png => default_image_format(img, ImageOutputFormat::Png),
            ImageProcessingOutput::WebP => webp(img, 80),
            ImageProcessingOutput::Gif => default_image_format(img, ImageOutputFormat::Gif),
        }
    }
}
