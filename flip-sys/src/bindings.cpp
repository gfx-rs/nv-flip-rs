#include "bindings.hpp"
#include "FLIP.h"

extern "C" {
    struct FlipImage {
        FLIP::image<FLIP::color3> inner;
    };

    FlipImage* flip_image_new(uint32_t width, uint32_t height, uint8_t const* data) {
        auto image = new FlipImage { FLIP::image<FLIP::color3>(width, height) };
        if (data) {
            for (uint32_t x = 0; x < width; x++) {
                for (uint32_t y = 0; y < height; y++) {
                    image->inner.set(x, y, FLIP::color3(data[0], data[1], data[2]));
                    data += 3;
                }
            }
        }
        return image;
    }

    void flip_image_free(FlipImage* image) {
        delete image;
    }
}