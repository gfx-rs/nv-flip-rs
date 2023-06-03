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
//! let visualized = error_map.apply_color_lut(&nv_flip::FlipImageRgb8::new_magma_lut());
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

pub struct FlipImageRgb8 {
    inner: *mut nv_flip_sys::FlipImageColor3,
    width: u32,
    height: u32,
}

unsafe impl Send for FlipImageRgb8 {}
unsafe impl Sync for FlipImageRgb8 {}

impl FlipImageRgb8 {
    pub fn new(width: u32, height: u32) -> Self {
        let inner = unsafe { nv_flip_sys::flip_image_color3_new(width, height, std::ptr::null()) };
        assert!(!inner.is_null());
        Self {
            inner,
            width,
            height,
        }
    }

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

    pub fn new_magma_lut() -> Self {
        let inner = unsafe { nv_flip_sys::flip_image_color3_magma_map() };
        assert!(!inner.is_null());
        Self {
            inner,
            width: 256,
            height: 1,
        }
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut data = vec![0u8; (self.width * self.height * 3) as usize];
        unsafe {
            nv_flip_sys::flip_image_color3_get_data(self.inner, data.as_mut_ptr());
        }
        data
    }

    pub fn width(&self) -> u32 {
        self.width
    }

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

pub struct FlipImageFloat {
    inner: *mut nv_flip_sys::FlipImageFloat,
    width: u32,
    height: u32,
}

unsafe impl Send for FlipImageFloat {}
unsafe impl Sync for FlipImageFloat {}

impl FlipImageFloat {
    pub fn new(width: u32, height: u32) -> Self {
        let inner = unsafe { nv_flip_sys::flip_image_float_new(width, height, std::ptr::null()) };
        assert!(!inner.is_null());
        Self {
            inner,
            width,
            height,
        }
    }

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

    pub fn apply_color_lut(&self, value_mapping: &FlipImageRgb8) -> FlipImageRgb8 {
        let output = FlipImageRgb8::new(self.width, self.height);
        unsafe {
            nv_flip_sys::flip_image_color3_color_map(output.inner, self.inner, value_mapping.inner);
        }
        output
    }

    pub fn to_color3(&self) -> FlipImageRgb8 {
        let color3 = FlipImageRgb8::new(self.width, self.height);
        unsafe {
            nv_flip_sys::flip_image_float_copy_float_to_color3(self.inner, color3.inner);
        }
        color3
    }

    pub fn to_vec(&self) -> Vec<f32> {
        let mut data = vec![0f32; (self.width * self.height) as usize];
        unsafe {
            nv_flip_sys::flip_image_float_get_data(self.inner, data.as_mut_ptr());
        }
        data
    }

    pub fn width(&self) -> u32 {
        self.width
    }

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

        let magma_lut = FlipImageRgb8::new_magma_lut();
        let color = error_map.apply_color_lut(&magma_lut);

        let image =
            image::RgbImage::from_raw(color.width(), color.height(), color.to_vec()).unwrap();

        let reference = image::open("../etc/tree-comparison-cli.png")
            .unwrap()
            .into_rgb8();

        assert_eq!(image, reference);
    }
}
