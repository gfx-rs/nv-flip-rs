#include "FLIP.h"

#include "bindings.hpp"

extern "C" {
    struct FlipImageColor3 {
        FLIP::image<FLIP::color3> inner;
    };

    struct FlipImageFloat {
        FLIP::image<float> inner;
    };

    FlipImageColor3* flip_image_color3_new(uint32_t width, uint32_t height, uint8_t const* data) {
        auto image = new FlipImageColor3 { FLIP::image<FLIP::color3>(width, height) };
        if (data) {
            for (uint32_t y = 0; y < height; y++) {
                for (uint32_t x = 0; x < width; x++) {
                    image->inner.set(x, y, FLIP::color3(
                        float(data[0]) / 255.0,
                        float(data[1]) / 255.0,
                        float(data[2]) / 255.0
                    ));
                    data += 3;
                }
            }
        }
        return image;
    }

    inline static float fClamp(float value) { return std::max(0.0f, std::min(1.0f, value)); }

    void flip_image_color3_get_data(FlipImageColor3 const* image, uint8_t* data) {
        for (uint32_t y = 0; y < image->inner.getHeight(); y++) {
            for (uint32_t x = 0; x < image->inner.getWidth(); x++) {
                auto color = image->inner.get(x, y);
                data[0] = uint8_t(fClamp(color.r) * 255.0f + 0.5f);
                data[1] = uint8_t(fClamp(color.g) * 255.0f + 0.5f);
                data[2] = uint8_t(fClamp(color.b) * 255.0f + 0.5f);
                data += 3;
            }
        }
    }

    void flip_image_color3_free(FlipImageColor3* image) {
        delete image;
    }

    FlipImageColor3* flip_image_color3_magma_map() {
        return new FlipImageColor3 { FLIP::image<FLIP::color3>(FLIP::MapMagma, 256) };
    }

    void flip_image_color3_color_map(FlipImageColor3* result_image, FlipImageFloat* error_map, FlipImageColor3* value_mapping) {
        result_image->inner.colorMap(error_map->inner, value_mapping->inner);
    }

    FlipImageFloat* flip_image_float_new(uint32_t width, uint32_t height, float const* data) {
        auto image = new FlipImageFloat { FLIP::image<float>(width, height) };
        if (data) {
            for (uint32_t y = 0; y < height; y++) {
                for (uint32_t x = 0; x < width; x++) {
                    image->inner.set(x, y, *data);
                    data += 1;
                }
            }
        }
        return image;
    }

    void flip_image_float_get_data(FlipImageFloat const* image, float* data) {
        for (uint32_t y = 0; y < image->inner.getHeight(); y++) {
            for (uint32_t x = 0; x < image->inner.getWidth(); x++) {
                auto value = image->inner.get(x, y);
                *data = value;
                data += 1;
            }
        }
    }

    void flip_image_float_free(FlipImageFloat* image) {
        delete image;
    }

    void flip_image_float_flip(FlipImageFloat* error_map, FlipImageColor3* reference_image, FlipImageColor3* test_image, float pixels_per_degree) {
        error_map->inner.FLIP(reference_image->inner, test_image->inner, pixels_per_degree);
    }

    void flip_image_float_copy_float_to_color3(FlipImageFloat* error_map, FlipImageColor3* output) {
        output->inner.copyFloat2Color3(error_map->inner);
    }

}