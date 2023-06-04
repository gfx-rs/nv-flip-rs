#include <cmath> // std::sqrt, std::exp
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
        if (data) {
            auto image = new FlipImageColor3 { FLIP::image<FLIP::color3>(width, height) };
            for (uint32_t y = 0; y < height; y++) {
                for (uint32_t x = 0; x < width; x++) {
                    image->inner.set(x, y, FLIP::color3(
                        float(data[0]) / 255.0f,
                        float(data[1]) / 255.0f,
                        float(data[2]) / 255.0f
                    ));
                    data += 3;
                }
            }
            return image;
        } else {
            return new FlipImageColor3 { FLIP::image<FLIP::color3>(width, height, FLIP::color3(0.0f, 0.0f, 0.0f)) };
        }
    }

    FlipImageColor3* flip_image_color3_clone(FlipImageColor3* image) {
        return new FlipImageColor3 { FLIP::image<FLIP::color3>(image->inner) };
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
        if (data) {
            auto image = new FlipImageFloat { FLIP::image<float>(width, height) };
            for (uint32_t y = 0; y < height; y++) {
                for (uint32_t x = 0; x < width; x++) {
                    image->inner.set(x, y, *data);
                    data += 1;
                }
            }
            return image;
        } else {
            return new FlipImageFloat { FLIP::image<float>(width, height, 0.0f) };
        }
    }

    FlipImageFloat* flip_image_float_clone(FlipImageFloat* image) {
        return new FlipImageFloat { FLIP::image<float>(image->inner) };
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

    struct FlipImageHistogramRef {
        histogram<float>& inner;
    };

    size_t flip_image_histogram_ref_get_bucket_size(FlipImageHistogramRef const* histogram) {
        return histogram->inner.getBucketSize();
    }
    size_t flip_image_histogram_ref_get_bucket_id_min(FlipImageHistogramRef const* histogram) {
        return histogram->inner.getBucketIdMin();
    }
    size_t flip_image_histogram_ref_get_bucket_id_max(FlipImageHistogramRef const* histogram) {
        return histogram->inner.getBucketIdMax();
    }
    size_t flip_image_histogram_ref_get_bucket_value(FlipImageHistogramRef const* histogram, size_t bucket_id) {
        return histogram->inner.getBucketValue(bucket_id);
    }
    size_t flip_image_histogram_ref_size(FlipImageHistogramRef const* histogram) {
        return histogram->inner.size();
    }
    float flip_image_histogram_ref_get_min_value(FlipImageHistogramRef const* histogram) {
        return histogram->inner.getMinValue();
    }
    float flip_image_histogram_ref_get_max_value(FlipImageHistogramRef const* histogram) {
        return histogram->inner.getMaxValue();
    }
    void flip_image_histogram_ref_clear(FlipImageHistogramRef* histogram) {
        histogram->inner.clear();
    }
    void flip_image_histogram_ref_resize(FlipImageHistogramRef* histogram, size_t buckets) {
        histogram->inner.resize(buckets);
    }
    size_t flip_image_histogram_ref_value_bucket_id(FlipImageHistogramRef* histogram, float buckets) {
        return histogram->inner.valueBucketId(buckets);
    }
    void flip_image_histogram_ref_inc_value(FlipImageHistogramRef* histogram, float value, size_t count) {
        histogram->inner.inc(value, count);
    }
    void flip_image_histogram_ref_inc_image(FlipImageHistogramRef* histogram, FlipImageFloat const* image) {
        for (uint32_t y = 0; y < image->inner.getHeight(); y++) {
            for (uint32_t x = 0; x < image->inner.getWidth(); x++) {
                auto value = image->inner.get(x, y);
                histogram->inner.inc(value, 1);
            }
        }
    }
    void flip_image_histogram_ref_free(FlipImageHistogramRef* histogram) {
        delete histogram;
    }

    struct FlipImagePool {
        pooling<float> inner;
    };

    FlipImagePool* flip_image_pool_new(size_t buckets) {
        return new FlipImagePool { pooling<float>(buckets) };
    }
    FlipImageHistogramRef* flip_image_pool_get_histogram(FlipImagePool* pool) {
        return new FlipImageHistogramRef { pool->inner.getHistogram() };
    }
    float flip_image_pool_get_min_value(FlipImagePool const* pool) {
        return pool->inner.getMinValue();
    }
    float flip_image_pool_get_max_value(FlipImagePool const* pool) {
        return pool->inner.getMaxValue();
    }
    float flip_image_pool_get_mean(FlipImagePool const* pool) {
        return pool->inner.getMean();
    }
    double flip_image_pool_get_weighted_percentile(FlipImagePool const* pool, double percentile) {
        return pool->inner.getWeightedPercentile(percentile);
    }
    float flip_image_pool_get_percentile(FlipImagePool* pool, float percentile, bool weighted) {
        return pool->inner.getPercentile(percentile, weighted);
    }
    void flip_image_pool_update_image(FlipImagePool* pool, FlipImageFloat const* image) {
        for (uint32_t y = 0; y < image->inner.getHeight(); y++) {
            for (uint32_t x = 0; x < image->inner.getWidth(); x++) {
                pool->inner.update(x, y, image->inner.get(x, y));
            }
        }
    }
    void flip_image_pool_clear(FlipImagePool* pool) {
        pool->inner.clear();
    }
    void flip_image_pool_free(FlipImagePool* pool) {
        delete pool;
    }
}