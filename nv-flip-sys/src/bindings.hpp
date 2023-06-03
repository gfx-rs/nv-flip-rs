#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif
    struct FlipImageColor3;
    struct FlipImageFloat;

    FlipImageColor3* flip_image_color3_new(uint32_t width, uint32_t height, uint8_t const* data);
    void flip_image_color3_get_data(FlipImageColor3 const* image, uint8_t* data);
    void flip_image_color3_free(FlipImageColor3* image);

    FlipImageColor3* flip_image_color3_magma_map();
    void flip_image_color3_color_map(FlipImageColor3* output, FlipImageFloat* error_map, FlipImageColor3* value_mapping);
    
    FlipImageFloat* flip_image_float_new(uint32_t width, uint32_t height, float const* data);
    void flip_image_float_get_data(FlipImageFloat const* image, float* data);
    void flip_image_float_free(FlipImageFloat* image);

    void flip_image_float_flip(FlipImageFloat* error_map, FlipImageColor3* reference_image, FlipImageColor3* test_image, float pixels_per_degree);

    void flip_image_float_copy_float_to_color3(FlipImageFloat* error_map, FlipImageColor3* output);

#ifdef __cplusplus
}
#endif
