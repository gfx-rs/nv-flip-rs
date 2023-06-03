#pragma once

#include <stdint.h>
#include <stdlib.h>

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

    struct FlipImageHistogramRef;

    FlipImageHistogramRef* flip_image_histogram_ref_new(size_t buckets, float min_value, float max_value);
    size_t flip_image_histogram_ref_get_bucket_size(FlipImageHistogramRef const* histogram);
    size_t flip_image_histogram_ref_get_bucket_id_min(FlipImageHistogramRef const* histogram);
    size_t flip_image_histogram_ref_get_bucket_id_max(FlipImageHistogramRef const* histogram);
    size_t flip_image_histogram_ref_get_bucket_value(FlipImageHistogramRef const* histogram, size_t bucket_id);
    size_t flip_image_histogram_ref_size(FlipImageHistogramRef const* histogram);
    float flip_image_histogram_ref_get_min_value(FlipImageHistogramRef const* histogram);
    float flip_image_histogram_ref_get_max_value(FlipImageHistogramRef const* histogram);
    float flip_image_histogram_ref_bucket_step(FlipImageHistogramRef const* histogram);
    void flip_image_histogram_ref_clear(FlipImageHistogramRef* histogram);
    void flip_image_histogram_ref_resize(FlipImageHistogramRef* histogram, size_t buckets);
    size_t flip_image_histogram_ref_value_bucket_id(FlipImageHistogramRef* histogram, float buckets);
    void flip_image_histogram_ref_inc_value(FlipImageHistogramRef* histogram, float value, size_t count);
    void flip_image_histogram_ref_inc_image(FlipImageHistogramRef* histogram, FlipImageFloat const* image);
    void flip_image_histogram_ref_free(FlipImageHistogramRef* histogram);

    struct FlipImagePool;

    FlipImagePool* flip_image_pool_new(size_t buckets);
    FlipImageHistogramRef* flip_image_pool_get_histogram(FlipImagePool* pool);
    float flip_image_pool_get_min_value(FlipImagePool const* pool);
    float flip_image_pool_get_max_value(FlipImagePool const* pool);
    float flip_image_pool_get_mean(FlipImagePool const* pool);
    double flip_image_pool_get_weighted_percentile(FlipImagePool const* pool, double percentile);
    float flip_image_pool_get_percentile(FlipImagePool* pool, float percentile, bool weighted);
    void flip_image_pool_update_image(FlipImagePool* pool, FlipImageFloat* image);
    void flip_image_pool_clear(FlipImagePool* pool);
    void flip_image_pool_free(FlipImagePool* pool);


#ifdef __cplusplus
}
#endif
