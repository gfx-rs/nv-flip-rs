#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif
    struct FlipImage;
    FlipImage* flip_image_new(uint32_t width, uint32_t height, uint8_t const* data);
    void flip_image_free(FlipImage* image);

    void flip_image_flip(FlipImage* error_map, FlipImage* reference_image, FlipImage* test_image);

#ifdef __cplusplus
}
#endif
