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

    struct FlipImageHistogram;

    FlipImageHistogram* flip_image_histogram_new(size_t buckets, float min_value, float max_value);
    size_t flip_image_histogram_get_bucket_size(FlipImageHistogram const* histogram);
    size_t flip_image_histogram_get_bucket_id_min(FlipImageHistogram const* histogram);
    size_t flip_image_histogram_get_bucket_id_max(FlipImageHistogram const* histogram);
    size_t flip_image_histogram_get_bucket_value(FlipImageHistogram const* histogram, size_t bucket_id);
    size_t flip_image_histogram_size(FlipImageHistogram const* histogram);
    float flip_image_histogram_get_min_value(FlipImageHistogram const* histogram);
    float flip_image_histogram_get_max_value(FlipImageHistogram const* histogram);
    float flip_image_histogram_bucket_step(FlipImageHistogram const* histogram);
    void flip_image_histogram_clear(FlipImageHistogram* histogram);
    void flip_image_histogram_resize(FlipImageHistogram* histogram, size_t buckets);
    void flip_image_histogram_value_bucket_id(FlipImageHistogram* histogram, size_t buckets);
    void flip_image_histogram_inc_value(FlipImageHistogram* histogram, float value, size_t count);
    void flip_image_histogram_inc_image(FlipImageHistogram* histogram, FlipImageFloat* image);
    void flip_image_histogram_free(FlipImageHistogram* histogram);

    struct FlipImagePool;

    FlipImagePool* flip_image_pool_new(size_t buckets);
    float flip_image_pool_get_min_value(FlipImagePool const* pool);
    float flip_image_pool_get_max_value(FlipImagePool const* pool);
    float flip_image_pool_get_mean_value(FlipImagePool const* pool);
    double flip_image_pool_get_weighted_percentile(FlipImagePool const* pool, double percentile);
    float flip_image_pool_get_percentile(FlipImagePool* pool, float percentile, bool weighted);
    void flip_image_pool_update_image(FlipImagePool* pool, FlipImageFloat* image);
    void flip_image_pool_free(FlipImagePool* pool);


#ifdef __cplusplus
}
#endif
