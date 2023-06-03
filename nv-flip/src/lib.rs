pub use nv_flip_sys::calculate_pixels_per_degree;

pub struct FlipImageVec3 {
    inner: *mut nv_flip_sys::FlipImageColor3,
    width: u32,
    height: u32,
}

unsafe impl Send for FlipImageVec3 {}
unsafe impl Sync for FlipImageVec3 {}

impl FlipImageVec3 {
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

    pub fn new_magma_map() -> Self {
        let inner = unsafe { nv_flip_sys::flip_image_color3_magma_map() };
        assert!(!inner.is_null());
        Self {
            inner,
            width: 256,
            height: 1,
        }
    }

    pub fn color_map(&mut self, error_map: &FlipImageFloat, value_mapping: &FlipImageVec3) {
        unsafe {
            nv_flip_sys::flip_image_color3_color_map(
                self.inner,
                error_map.inner,
                value_mapping.inner,
            );
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

impl Drop for FlipImageVec3 {
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

    pub fn to_color3(&self) -> FlipImageVec3 {
        let color3 = FlipImageVec3::new(self.width, self.height);
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

pub fn flip(reference_image: &FlipImageVec3, test_image: &FlipImageVec3) -> FlipImageFloat {
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
            67.0,
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
        let reference_image = FlipImageVec3::with_data(
            reference_image.width(),
            reference_image.height(),
            &reference_image,
        );

        let test_image = image::open("../etc/tree-test.png").unwrap().into_rgb8();
        let test_image =
            FlipImageVec3::with_data(test_image.width(), test_image.height(), &test_image);

        let error_map = flip(&reference_image, &test_image);

        let mut greyscale = error_map.to_color3();

        let magma_map = FlipImageVec3::new_magma_map();
        greyscale.color_map(&error_map, &magma_map);

        let image =
            image::RgbImage::from_raw(greyscale.width(), greyscale.height(), greyscale.to_vec())
                .unwrap();

        let reference = image::open("../etc/tree-comparison-cli.png")
            .unwrap()
            .into_rgb8();

        assert_eq!(image, reference);
    }
}
