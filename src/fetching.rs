use crate::performance::Performance;
use custom_error::custom_error;
use std::time::Instant;
use url::Url;

/// The maximum allowed file size of the source image.
pub const MAX_INPUT_SIZE: u64 = 2 << 25; // About 32 MiB.

pub struct FetchBytesResponse {
    pub bytes: Vec<u8>,
    pub performance: Performance,
}

custom_error! {pub FetchError
  Unknown = "Unknown error!",
  MaxSizeExceeded = "The maximum response size was exceeded!",
  InvalidInput = "An invalid input was provided.",
  ProcessImageError{source: image::ImageError} = "Could not process the image!",
  UpstreamFetchError{source: reqwest::Error} = "An error occurred when fetching the image!"
}

pub type FetchResult<T> = Result<T, FetchError>;

fn fetch_bytes(url: Url) -> FetchResult<FetchBytesResponse> {
    let start = Instant::now();
    let url_str = url.into_string();
    let res = reqwest::blocking::get(&url_str)?;

    let body_size = match res.content_length() {
        Some(x) => x,
        None => {
            // If Reqwest can't determine the size of the input, nobody can! We must play it safe and ABORT!
            return Err(FetchError::InvalidInput);
        }
    };

    if body_size > MAX_INPUT_SIZE {
        // The response is larger than the maximum allowed size. ERROR!!!
        return Err(FetchError::MaxSizeExceeded);
    }

    let bytes = res.bytes()?.to_vec();

    Ok(FetchBytesResponse {
        bytes,
        performance: Performance {
            elapsed_ns: start.elapsed().as_nanos(),
        },
    })
}

pub struct FetchDynamicImageResponse {
    pub img: image::DynamicImage,
    pub performance: Performance,
}

pub fn fetch_dynimage(url: Url) -> FetchResult<FetchDynamicImageResponse> {
    let start = Instant::now();
    let response = fetch_bytes(url)?;
    let img = image::load_from_memory(&response.bytes)?;
    Ok(FetchDynamicImageResponse {
        img,
        performance: Performance {
            elapsed_ns: start.elapsed().as_nanos(),
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filesize_limit() {
        let very_large_file =
            "https://spacetelescope.org/static/archives/images/original/opo0328a.tif";
        let result = fetch_bytes(Url::parse(very_large_file).unwrap()); // This file is roughly 170 MiB, way above the 32 MiB limit.
        assert!(result.is_err(), "you shouldn't be able to download files above 32 MiB (or have we changed the file size limit?)")
    }

    #[test]
    fn invalid_file_input() {
        let invalid_file = "https://httpbin.org/get";
        let result = fetch_dynimage(Url::parse(invalid_file).unwrap());
        assert!(
            result.is_err(),
            "the image crate shouldn't accept JSON as a valid image format ..."
        )
    }
}
