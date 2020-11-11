use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use image;
use pyo3::wrap_pyfunction;
use pyo3::prelude::*;

struct Bucket {
    r: f64,
    g: f64,
    b: f64,
    count: f64,
}

/// Calculate the dominant color from an image (as a bytes object)
#[pyfunction]
#[text_signature = "(buffer, /)"]
fn get_dominant_color(buffer: &[u8]) -> PyResult<usize> {
    // Open image from bytes
    let img = image::load_from_memory(buffer).unwrap();

    // Get pixels as a vector
    let pixels = img.to_bytes();

    // Check if image has alpha
    let has_alpha = match img.color() {
        image::ColorType::Rgba8 => true,
        image::ColorType::Bgra8 => true,
        _ => false,
    };

    let bytes_per_pixel = if has_alpha { 4 } else { 3 };
    let pixel_count = pixels.len() / bytes_per_pixel;

    let mut buckets: HashMap<[u8; 3], Bucket, RandomState> = HashMap::new();

    // Iterate over pixels
    for i in 0..pixel_count {
        let r = pixels[i * bytes_per_pixel];
        let g = pixels[i * bytes_per_pixel + 1];
        let b = pixels[i * bytes_per_pixel + 2];

        // Alpha value is used for weighting
        let alpha = if has_alpha {
            pixels[i * bytes_per_pixel + 3] as f64 / 255.0
        } else {
            1.0
        };

        let cluster = [(r >> 6), (g >> 6), (b >> 6)];

        buckets.entry(cluster)
            .and_modify(|e| {
                e.r += r as f64 * alpha;
                e.g += g as f64 * alpha;
                e.b += b as f64 * alpha;
                e.count += alpha;
            })
            .or_insert(Bucket {
                r: r as f64 * alpha,
                g: g as f64 * alpha,
                b: b as f64 * alpha,
                count: alpha,
            });
    }

    // Calculate average of each bucket
    let mut buckets_averages = Vec::new();

    for (_cluster, bucket) in &buckets {
        if bucket.count > 0.0 {
            buckets_averages.push(Bucket {
                r: bucket.r / bucket.count,
                g: bucket.g / bucket.count,
                b: bucket.b / bucket.count,
                count: bucket.count,
            })
        }
    }

    // Sort buckets averages
    buckets_averages.sort_by(|a, b| a.count.partial_cmp(&b.count).unwrap());
    let dominant_bucket = &buckets_averages[0];

    Ok((dominant_bucket.r as usize) << 16 | (dominant_bucket.g as usize) << 8 | dominant_bucket.b as usize)
}

#[pymodule]
fn dominant_color(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_dominant_color, m)?)?;

    Ok(())
}