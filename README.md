# nv-flip

Bindings to NVIDIA's [ꟻLIP] image comparison library.

This library allows you to visualize and reason about the human-noticable differences
between rendered images. Especially when comparing images that are noisy or other small
differences, FLIP's comparison can be more meaningful than a simple pixel-wise comparison.

![comp](https://raw.githubusercontent.com/NVlabs/flip/main/images/teaser.png)

In order to keep a small dependency closure, this crate does not depend on `image`,
but interop is simple.

## Example

```rust
// First we load the "reference image". This is the image we want to compare against.
//
// We make sure to turn the image into RGB8 as FLIP doesn't deal with alpha.
let ref_image_data = image::open("../etc/tree-ref.png").unwrap().into_rgb8();
let ref_image = nv_flip::FlipImageRgb8::with_data(
    ref_image_data.width(),
    ref_image_data.height(),
    &ref_image_data
);

// We then load the "test image". This is the image we want to compare to the reference.
let test_image_data = image::open("../etc/tree-test.png").unwrap().into_rgb8();
let test_image = nv_flip::FlipImageRgb8::with_data(
    test_image_data.width(),
    test_image_data.height(),
    &test_image_data
);

// We now run the comparison. This will produce a "error map" that that is the per-pixel
// visual difference between the two images between 0 and 1.
//
// The last parameter is the number of pixels per degree of visual angle. This is used
// to determine the size of imperfections that can be seen. See the `pixels_per_degree`
// for more information. By default this value is 67.0.
let error_map = nv_flip::flip(ref_image, test_image, nv_flip::DEFAULT_PIXELS_PER_DEGREE);

// We can now visualize the error map using a LUT that maps the error value to a color.
let visualized = error_map.apply_color_lut(&nv_flip::magma_lut());

// Finally we can the final image into an `image` crate image and save it.
let image = image::RgbImage::from_raw(
    visualized.width(),
    visualized.height(),
    visualized.to_vec()
).unwrap();

// We can get statistics about the error map by using their "Pool" type,
// which is essentially a weighted histogram.
let mut pool = nv_flip::FlipPool::from_image(&error_map);

// These are the same statistics shown by the command line.
//
// The paper's writers recommend that, if you are to use a single number to
// represent the error, they recommend the mean.
println!("Mean: {}", pool.mean());
println!("Weighted median: {}", pool.get_percentile(0.5, true));
println!("1st weighted quartile: {}", pool.get_percentile(0.25, true));
println!("3rd weighted quartile: {}", pool.get_percentile(0.75, true));
println!("Min: {}", pool.min_value());
println!("Max: {}", pool.max_value());
```
The result of this example looks like this:

<!-- This table uses U+2800 BRAILLE PATTERN BLANK in the header make the images vaguely the same size. -->

| Reference | ⠀⠀Test⠀⠀ | ⠀Result⠀ |
|:---------:|:---------:|:---------:|
| ![comp](https://raw.githubusercontent.com/gfx-rs/nv-flip-rs/trunk/etc/tree-ref.png) | ![comp](https://raw.githubusercontent.com/gfx-rs/nv-flip-rs/trunk/etc/tree-test.png)  | ![comp](https://raw.githubusercontent.com/gfx-rs/nv-flip-rs/trunk/etc/tree-comparison-cli.png) |

## License

The binding and rust interop code is tri-licensed under MIT, Apache-2.0, and ZLib.

The ꟻLIP library itself is licensed under the BSD-3-Clause license.

The example images used are licensed under the [Unsplash License].

[ꟻLIP]: https://github.com/NVlabs/flip
[Unsplash License]: https://unsplash.com/license

License: MIT OR Apache-2.0 OR Zlib
