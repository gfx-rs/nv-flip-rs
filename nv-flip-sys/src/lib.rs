include!("bindings.rs");

/// Default configuration for pixels per degree.
///
/// Corresponds to _roughly_ a 31.6" monitor at 3840x2160 resolution, viewed from 70cm away.
///
/// This value is how you adjust the sensitivity of the comparison.
/// Use [`pixels_per_degree`] to compute a custom value for your situation.
///
/// ```rust
/// let computed = nv_flip::pixels_per_degree(0.7, 3840.0, 0.7);
/// assert!((computed - nv_flip::DEFAULT_PIXELS_PER_DEGREE).abs() < 0.05);
/// ```
pub const DEFAULT_PIXELS_PER_DEGREE: f32 = 67.0;

/// Computes the pixels per degree of arc given a monitor configuration.
///
/// - `distance` - Distance from the monitor in meters.
/// - `resolution_x` - Horizontal resolution of the monitor in pixels.
/// - `monitor_width` - Width of the monitor in meters.
///
/// This value is how you adjust the sensitivity of the comparison.
///
/// If you don't care, use [`DEFAULT_PIXELS_PER_DEGREE`].
pub fn pixels_per_degree(distance: f32, resolution_x: f32, monitor_width: f32) -> f32 {
    distance * (resolution_x / monitor_width) * (std::f32::consts::PI / 180.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_deletion() {
        unsafe {
            let image = flip_image_color3_new(10, 10, std::ptr::null_mut());
            assert!(!image.is_null());
            flip_image_color3_free(image);
        }
    }

    #[test]
    fn creation_with_data_and_deletion() {
        let data = vec![0u8; 10 * 10 * 3];
        unsafe {
            let image = flip_image_color3_new(10, 10, data.as_ptr());
            assert!(!image.is_null());
            flip_image_color3_free(image);
        }
    }

    #[test]
    fn end_to_end() {
        let ref_image = image::open("../etc/tree-ref.png").unwrap().into_rgb8();
        let test_image = image::open("../etc/tree-test.png").unwrap().into_rgb8();

        let ref_flip = unsafe {
            flip_image_color3_new(ref_image.width(), ref_image.height(), ref_image.as_ptr())
        };
        let test_flip = unsafe {
            flip_image_color3_new(test_image.width(), test_image.height(), test_image.as_ptr())
        };

        let error_map = unsafe {
            flip_image_float_new(ref_image.width(), ref_image.height(), std::ptr::null())
        };

        unsafe {
            flip_image_float_flip(error_map, ref_flip, test_flip, 67.0);
        }

        let histogram = unsafe { flip_image_pool_new(256) };
        unsafe { flip_image_pool_update_image(histogram, error_map) }
        println!("{:.6}", unsafe {
            flip_image_pool_get_percentile(histogram, 0.75, true)
        });

        let output_flip = unsafe {
            flip_image_color3_new(ref_image.width(), ref_image.height(), std::ptr::null())
        };

        let magma_flip = unsafe { flip_image_color3_magma_map() };

        unsafe {
            flip_image_float_copy_float_to_color3(error_map, output_flip);
            flip_image_color3_color_map(output_flip, error_map, magma_flip);
        };

        let mut output_image = image::RgbImage::new(ref_image.width(), ref_image.height());

        unsafe {
            flip_image_color3_get_data(output_flip, output_image.as_mut_ptr());
        }

        unsafe {
            flip_image_color3_free(ref_flip);
            flip_image_color3_free(test_flip);
            flip_image_color3_free(output_flip);
            flip_image_color3_free(magma_flip);
            flip_image_float_free(error_map);
            flip_image_pool_free(histogram);
        }

        let sample = image::open("../etc/tree-comparison-cli.png")
            .unwrap()
            .into_rgb8();

        for (a, b) in output_image.pixels().zip(sample.pixels()) {
            assert!(a.0[0].abs_diff(b.0[0]) <= 3);
            assert!(a.0[1].abs_diff(b.0[1]) <= 3);
            assert!(a.0[2].abs_diff(b.0[2]) <= 3);
        }
    }
}
