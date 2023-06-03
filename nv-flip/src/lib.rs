//! Bindings to NVIDIA's [ꟻLIP] image comparison library.
//!
//! This library allows you to visualize and reason about the human-noticable differences
//! between rendered images. Especially when comparing images that are noisy or other small
//! differences, FLIP's comparison can be more meaningful than a simple pixel-wise comparison.
//!
//! ![comp](https://raw.githubusercontent.com/NVlabs/flip/main/images/teaser.png)
//!
//! In order to keep a small dependency closure, this crate does not depend on `image`,
//! but interop is simple.
//!
//! # Example
//!
//! ```rust
//! // First we load the "reference image". This is the image we want to compare against.
//! //
//! // We make sure to turn the image into RGB8 as FLIP doesn't deal with alpha.
//! let ref_image_data = image::open("../etc/tree-ref.png").unwrap().into_rgb8();
//! let ref_image = nv_flip::FlipImageRgb8::with_data(
//!     ref_image_data.width(),
//!     ref_image_data.height(),
//!     &ref_image_data
//! );
//!
//! // We then load the "test image". This is the image we want to compare to the reference.
//! let test_image_data = image::open("../etc/tree-test.png").unwrap().into_rgb8();
//! let test_image = nv_flip::FlipImageRgb8::with_data(
//!     test_image_data.width(),
//!     test_image_data.height(),
//!     &test_image_data
//! );
//!
//! // We now run the comparison. This will produce a "error map" that that is the per-pixel
//! // visual difference between the two images between 0 and 1.
//! //
//! // The last parameter is the number of pixels per degree of visual angle. This is used
//! // to determine the size of imperfections that can be seen. See the `pixels_per_degree`
//! // for more information. By default this value is 67.0.
//! let error_map = nv_flip::flip(&ref_image, &test_image, nv_flip::DEFAULT_PIXELS_PER_DEGREE);
//!
//! // We can now visualize the error map using a LUT that maps the error value to a color.
//! let visualized = error_map.apply_color_lut(&nv_flip::magma_lut());
//!
//! // Finally we can the final image into an `image` crate image and save it.
//! let image = image::RgbImage::from_raw(
//!     visualized.width(),
//!     visualized.height(),
//!     visualized.to_vec()
//! ).unwrap();
//! # let _ = image;
//! ```
//! The result of this example looks like this:
//!
//! <!-- This table uses U+2800 BRAILLE PATTERN BLANK in the header make the images vaguely the same size. -->
//!
//! | Reference | ⠀⠀Test⠀⠀ | ⠀Result⠀ |
//! |:---------:|:---------:|:---------:|
//! | ![comp](https://raw.githubusercontent.com/gfx-rs/nv-flip-rs/trunk/etc/tree-ref.png) | ![comp](https://raw.githubusercontent.com/gfx-rs/nv-flip-rs/trunk/etc/tree-test.png)  | ![comp](https://raw.githubusercontent.com/gfx-rs/nv-flip-rs/trunk/etc/tree-comparison-cli.png) |
//!
//!
//! # License
//!
//! The binding and rust interop code is tri-licensed under MIT, Apache-2.0, and ZLib.
//!
//! The ꟻLIP library itself is licensed under the BSD-3-Clause license.
//!
//! [ꟻLIP]: https://github.com/NVlabs/flip

pub use nv_flip_sys::{pixels_per_degree, DEFAULT_PIXELS_PER_DEGREE};

/// 2D FLIP image that is accessed as Rgb8.
///
/// Internally this is Rgb32f, but the values are converted when read.
pub struct FlipImageRgb8 {
    inner: *mut nv_flip_sys::FlipImageColor3,
    width: u32,
    height: u32,
}

unsafe impl Send for FlipImageRgb8 {}
unsafe impl Sync for FlipImageRgb8 {}

impl FlipImageRgb8 {
    /// Create a new image with the given dimensions and zeroed contents.
    pub fn new(width: u32, height: u32) -> Self {
        let inner = unsafe { nv_flip_sys::flip_image_color3_new(width, height, std::ptr::null()) };
        assert!(!inner.is_null());
        Self {
            inner,
            width,
            height,
        }
    }

    /// Creates a new image with the given dimensions and copies the data into it.
    ///
    /// The data must be in Rgb8 format. Do not include alpha.
    ///
    /// Data is expected in row-major orderm from the top left, tightly packed.
    ///
    /// # Panics
    ///
    /// - If the data is not large enough to fill the image.
    pub fn with_data(width: u32, height: u32, data: &[u8]) -> Self {
        assert!(data.len() >= (width * height * 3) as usize);
        let inner = unsafe { nv_flip_sys::flip_image_color3_new(width, height, data.as_ptr()) };
        assert!(!inner.is_null());
        Self {
            inner,
            width,
            height,
        }
    }

    /// Extracts the data from the image and returns it as a vector.
    ///
    /// Data is returned in row-major order, from the top left, tightly packed.
    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = vec![0u8; (self.width * self.height * 3) as usize];
        unsafe {
            nv_flip_sys::flip_image_color3_get_data(self.inner, data.as_mut_ptr());
        }
        data
    }

    /// Returns the width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for FlipImageRgb8 {
    fn drop(&mut self) {
        unsafe {
            nv_flip_sys::flip_image_color3_free(self.inner);
        }
    }
}

/// 2D FLIP image that stores a single float per pixel.
pub struct FlipImageFloat {
    inner: *mut nv_flip_sys::FlipImageFloat,
    width: u32,
    height: u32,
}

unsafe impl Send for FlipImageFloat {}
unsafe impl Sync for FlipImageFloat {}

impl FlipImageFloat {
    /// Create a new image with the given dimensions and zeroed contents.
    pub fn new(width: u32, height: u32) -> Self {
        let inner = unsafe { nv_flip_sys::flip_image_float_new(width, height, std::ptr::null()) };
        assert!(!inner.is_null());
        Self {
            inner,
            width,
            height,
        }
    }

    /// Creates a new image with the given dimensions and copies the data into it.
    ///
    /// Data is expected in row-major order, from the top left, tightly packed.
    ///
    /// # Panics
    ///
    /// - If the data is not large enough to fill the image.
    pub fn with_data(width: u32, height: u32, data: &[f32]) -> Self {
        assert!(data.len() >= (width * height) as usize);
        let inner = unsafe { nv_flip_sys::flip_image_float_new(width, height, data.as_ptr()) };
        assert!(!inner.is_null());
        Self {
            inner,
            width,
            height,
        }
    }

    /// Applies the given 1D color lut to turn this single channel values into 3 channel values.
    ///
    /// Applies the following algorithm to each pixel:
    ///
    /// ```text
    /// value_mapping[(pixel_value * 255).round() % value_mapping.width()]
    /// ```
    pub fn apply_color_lut(&self, value_mapping: &FlipImageRgb8) -> FlipImageRgb8 {
        let output = FlipImageRgb8::new(self.width, self.height);
        unsafe {
            nv_flip_sys::flip_image_color3_color_map(output.inner, self.inner, value_mapping.inner);
        }
        output
    }

    /// Converts the image to a color image by copying the single channel value to all 3 channels.
    pub fn to_color3(&self) -> FlipImageRgb8 {
        let color3 = FlipImageRgb8::new(self.width, self.height);
        unsafe {
            nv_flip_sys::flip_image_float_copy_float_to_color3(self.inner, color3.inner);
        }
        color3
    }

    /// Extracts the data from the image and returns it as a vector.
    ///
    /// Data is returned in row-major order, from the top left, tightly packed.
    pub fn to_vec(&self) -> Vec<f32> {
        let mut data = vec![0f32; (self.width * self.height) as usize];
        unsafe {
            nv_flip_sys::flip_image_float_get_data(self.inner, data.as_mut_ptr());
        }
        data
    }

    /// Returns the width of the image.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the image.
    pub fn height(&self) -> u32 {
        self.height
    }
}

impl Drop for FlipImageFloat {
    fn drop(&mut self) {
        unsafe {
            nv_flip_sys::flip_image_float_free(self.inner);
        }
    }
}

/// Generates a 1D lut using the builtin magma colormap for mapping error values to colors.
pub fn magma_lut() -> FlipImageRgb8 {
    let inner = unsafe { nv_flip_sys::flip_image_color3_magma_map() };
    assert!(!inner.is_null());
    FlipImageRgb8 {
        inner,
        width: 256,
        height: 1,
    }
}

/// Performs a FLIP comparison between the two images.
///
/// The images must be the same size.
///
/// Returns an error map image, where each pixel represents the error between the two images
/// at that location between 0 and 1.
///
/// The pixels_per_degree parameter is used to determine the sensitivity to differences. See the
/// documentation for [`DEFAULT_PIXELS_PER_DEGREE`] and [`pixels_per_degree`] for more information.
///
/// # Panics
///
/// - If the images are not the same size.
pub fn flip(
    reference_image: &FlipImageRgb8,
    test_image: &FlipImageRgb8,
    pixels_per_degree: f32,
) -> FlipImageFloat {
    assert_eq!(
        reference_image.width(),
        test_image.width(),
        "Width mismatch between reference and test image"
    );
    assert_eq!(
        reference_image.height(),
        test_image.height(),
        "Height mismatch between reference and test image"
    );

    let error_map = FlipImageFloat::new(reference_image.width(), reference_image.height());
    unsafe {
        nv_flip_sys::flip_image_float_flip(
            error_map.inner,
            reference_image.inner,
            test_image.inner,
            pixels_per_degree,
        );
    }
    error_map
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn zeroed_init() {
        assert_eq!(FlipImageRgb8::new(10, 10).to_vec(), vec![0u8; 10 * 10 * 3]);
        assert_eq!(FlipImageFloat::new(10, 10).to_vec(), vec![0.0f32; 10 * 10]);
    }

    #[test]
    fn end_to_end() {
        let reference_image = image::open("../etc/tree-ref.png").unwrap().into_rgb8();
        let reference_image = FlipImageRgb8::with_data(
            reference_image.width(),
            reference_image.height(),
            &reference_image,
        );

        let test_image = image::open("../etc/tree-test.png").unwrap().into_rgb8();
        let test_image =
            FlipImageRgb8::with_data(test_image.width(), test_image.height(), &test_image);

        let error_map = flip(&reference_image, &test_image, DEFAULT_PIXELS_PER_DEGREE);

        let magma_lut = magma_lut();
        let color = error_map.apply_color_lut(&magma_lut);

        let image =
            image::RgbImage::from_raw(color.width(), color.height(), color.to_vec()).unwrap();

        let reference = image::open("../etc/tree-comparison-cli.png")
            .unwrap()
            .into_rgb8();

        for (a, b) in image.pixels().zip(reference.pixels()) {
            assert!(a.0[0].abs_diff(b.0[0]) <= 3);
            assert!(a.0[1].abs_diff(b.0[1]) <= 3);
            assert!(a.0[2].abs_diff(b.0[2]) <= 3);
        }
    }
}
