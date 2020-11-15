use std::cmp::Ordering;
use std::collections::HashMap;

use colorsys::{Hsl, Rgb};
use pyo3::{create_exception, wrap_pyfunction};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

create_exception!(dominantcolor, DecodingError, PyException);

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

/// Calculates the dominant color of an image.
///
/// The dominant color is computed using HSL value of each pixel. Each pixel is classified using
/// its hue, and the average value of the biggest group of pixels is returned as an int.
/// The image must be as raw bytes. It will be decoded during processing.
///
/// :param bytes buffer: The image from which to calculate the dominant color.
/// :return int: The dominant color of the image.
/// :raises .ConversionError: Failed to decode buffer
#[pyfunction]
#[text_signature = "(buffer, /)"]
fn get_dominant_color(buffer: &[u8]) -> PyResult<usize> {
    // Open image from bytes
    let img = match image::load_from_memory(buffer) {
        Ok(img) => img,
        Err(_) => return Err(DecodingError::new_err("Failed to decode buffer")),
    };

    // Get pixels as a vector
    let pixels = img.to_bytes();

    // Check if image has alpha
    let has_alpha = matches!(
        img.color(),
        image::ColorType::Rgba8 | image::ColorType::Bgra8
    );

    let bytes_per_pixel = if has_alpha { 4 } else { 3 };
    let pixel_count = pixels.len() / bytes_per_pixel;
    let step = (pixel_count / 50_000).max(1); // Analyze at most 50k pixels

    let mut buckets: HashMap<usize, Bucket> = HashMap::new();

    // Iterate over pixels
    for pixel in pixels.windows(bytes_per_pixel).step_by(step) {
        let (r, g, b, a) = match *pixel {
            [r, g, b, a] => (r, g, b, a as f64 / 255.0),
            [r, g, b] => (r, g, b, 1.0),
            _ => unreachable!(),
        };

        // Convert rgb to hsl
        let hsl: Hsl = Rgb::from((r, g, b)).into();

        // Clustering using hue
        let cluster = (hsl.get_hue() as usize) >> 3;

        buckets
            .entry(cluster)
            .and_modify(|e| {
                e.h += hsl.get_hue() * a;
                e.s += hsl.get_saturation() * a;
                e.l += hsl.get_lightness() * a;
                e.count += a;
            })
            .or_insert(Bucket {
                h: hsl.get_hue() * a,
                s: hsl.get_saturation() * a,
                l: hsl.get_lightness() * a,
                count: a,
            });
    }

    // Sort buckets
    let mut sorted_buckets: Vec<_> = buckets.into_iter().map(|(_, b)| b).collect();
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
fn dominantcolor(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_dominant_color, m)?)?;
    m.add("DecodingError", py.get_type::<DecodingError>())?;

    Ok(())
}
