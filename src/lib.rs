use std::cmp::Ordering;
use std::collections::HashMap;

use colorsys::{Hsl, Rgb};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::{create_exception, wrap_pyfunction};

create_exception!(dominant_color, ConversionError, PyException);

struct Bucket {
    h: f64,
    s: f64,
    l: f64,
    count: f64,
}

impl Bucket {
    fn get_rgb(&self) -> Rgb {
        let rgb: Rgb = Hsl::from((self.h, self.s, self.l)).into();
        rgb
    }
}

/// Calculate the dominant color from an image (as a bytes object)
#[pyfunction]
#[text_signature = "(buffer, /)"]
fn get_dominant_color(buffer: &[u8]) -> PyResult<usize> {
    // Open image from bytes
    let img = match image::load_from_memory(buffer) {
        Ok(img) => img,
        Err(_) => return Err(ConversionError::new_err("Unable to convert image")),
    };

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
    let step = (pixel_count / 50_000).max(1); // Analyze at most 50k pixels

    let mut buckets: HashMap<usize, Bucket> = HashMap::new();

    // Iterate over pixels
    for i in (0..pixel_count).step_by(step) {
        let r = pixels[i * bytes_per_pixel];
        let g = pixels[i * bytes_per_pixel + 1];
        let b = pixels[i * bytes_per_pixel + 2];

        // Convert rgb to hsl
        let hsl: Hsl = Rgb::from((r, g, b)).into();

        // Alpha value is used for weighting
        let alpha = if has_alpha {
            pixels[i * bytes_per_pixel + 3] as f64 / 255.0
        } else {
            1.0
        };

        // Clustering using hue
        let cluster = (hsl.get_hue() as usize) >> 3;

        buckets
            .entry(cluster)
            .and_modify(|e| {
                e.h += hsl.get_hue() * alpha;
                e.s += hsl.get_saturation() * alpha;
                e.l += hsl.get_lightness() * alpha;
                e.count += alpha;
            })
            .or_insert(Bucket {
                h: hsl.get_hue() * alpha,
                s: hsl.get_saturation() * alpha,
                l: hsl.get_lightness() * alpha,
                count: alpha,
            });
    }

    // Sort buckets
    let mut sorted_buckets: Vec<Bucket> = buckets.into_iter().map(|(_, b)| b).collect();
    sorted_buckets.sort_by(|a, b| b.count.partial_cmp(&a.count).unwrap_or(Ordering::Equal));

    // Calculate average of dominant bucket
    let dominant_bucket = Bucket {
        h: sorted_buckets[0].h / sorted_buckets[0].count,
        s: sorted_buckets[0].s / sorted_buckets[0].count,
        l: sorted_buckets[0].l / sorted_buckets[0].count,
        count: 1_f64,
    }
    .get_rgb();

    Ok((dominant_bucket.get_red() as usize) << 16
        | (dominant_bucket.get_green() as usize) << 8
        | dominant_bucket.get_blue() as usize)
}

#[pymodule]
fn dominant_color(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_dominant_color, m)?)?;
    m.add("ConversionError", py.get_type::<ConversionError>())?;

    Ok(())
}
