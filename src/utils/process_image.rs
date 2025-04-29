use axum::body::Bytes;
use image::{imageops, DynamicImage, GenericImageView};
use webp::Encoder;
use std::time::Instant;
use tokio::task;

/// Processes an uploaded image by cropping, resizing, and converting to WebP format
/// using optimized webp-encoder crate with multicore support.
/// 
/// # Arguments
/// * `data` - Raw image bytes from the upload
/// * `width` - Target output width in pixels
/// * `height` - Target output height in pixels
/// * `debug` - Optional debug flag to enable timing logs
/// 
/// # Returns
/// Result containing WebP-encoded bytes or error message
/// 
/// # Example
/// ```
/// let processed = process_image(data, 300, 300, true).await?;
/// ```
pub async fn process_image(
    data: Bytes,
    width: u32,
    height: u32,
    debug: bool,
) -> Result<Bytes, String> {
    let timer = Instant::now();

    let result: Result<Bytes, String> = task::spawn_blocking(move || {
        let stage_timer = if debug { Some(Instant::now()) } else { None };

        // Load image
        let img = image::load_from_memory(&data)
            .map_err(|e| format!("Image load error: {e}"))?;
        if debug {
            log_time("Image loading", stage_timer.unwrap());
        }

        // Crop to square
        let stage_timer = debug.then(Instant::now);
        let cropped = square_crop(img);
        if debug {
            log_time("Square cropping", stage_timer.unwrap());
        }

        // Resize
        let stage_timer = debug.then(Instant::now);
        let resized = cropped.resize_to_fill(
            width,
            height,
            imageops::FilterType::Lanczos3,
        );
        if debug {
            log_time("Image resizing", stage_timer.unwrap());
        }

        // Convert to RGB
        let stage_timer = debug.then(Instant::now);
        let rgb_img = resized.to_rgb8();
        if debug {
            log_time("RGB conversion", stage_timer.unwrap());
        }

        // WebP Encoding
        let stage_timer = debug.then(Instant::now);
        let encoder = Encoder::from_rgb(&rgb_img, width, height);
        let webp_data = encoder.encode(60.0);
        if debug {
            log_time("WebP encoding", stage_timer.unwrap());
        }

        Ok::<Bytes, String>(Bytes::copy_from_slice(&webp_data))
    })
    .await
    .map_err(|e| format!("Task execution failed: {e}"))?;

    if debug {
        println!("Total processing time: {:.2}ms", timer.elapsed().as_millis());
    }

    result
}


/// Creates a square crop from any image aspect ratio by centering the crop area
/// 
/// # Arguments
/// * `img` - Input image to crop
/// 
/// # Returns
/// Square-cropped image with dimensions (min(width, height), min(width, height))
/// 
/// # Panics
/// Never panics - uses safe integer math for crop calculations
fn square_crop(img: DynamicImage) -> DynamicImage {
    let (width, height) = img.dimensions();
    let size = width.min(height);
    let x = (width - size) / 2;
    let y = (height - size) / 2;
    img.crop_imm(x, y, size, size)
}

/// Helper function for debug timing
fn log_time(stage: &str, timer: Instant) {
    println!("{} took {:.2}ms", stage, timer.elapsed().as_micros() as f32 / 1000.0);
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbImage;

    async fn test_image() -> Bytes {
        let img = RgbImage::new(600, 400);
        let mut buf = Vec::new();
        img.write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
            .unwrap();
        Bytes::from(buf)
    }

    #[tokio::test]
    async fn test_image_processing() {
        let data = test_image().await;
        let result = process_image(data, 300, 300, true).await;
        assert!(result.is_ok());
        let webp = result.unwrap();
        assert!(!webp.is_empty());
        assert!(webp.len() < 100_000); // Should be <100KB for 300x300
    }
}
