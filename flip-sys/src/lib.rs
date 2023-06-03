include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation_deletion() {
        unsafe {
            let image = flip_image_new(10, 10, std::ptr::null_mut());
            assert!(!image.is_null());
            flip_image_free(image);
        }
    }

    #[test]
    fn creation_with_data_and_deletion() {
        let data = vec![0u8; 10 * 10 * 3];
        unsafe {
            let image = flip_image_new(10, 10, data.as_ptr());
            assert!(!image.is_null());
            flip_image_free(image);
        }
    }
}
