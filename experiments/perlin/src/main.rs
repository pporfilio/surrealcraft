// use rustc_hash::FxHasher;
// use std::collections::hash_map::RandomState;

use ahash::RandomState;
use cgmath::Vector2;
use cgmath::Vector3;
use image;
mod base_noise;

fn test_1d_noise(imgx: u32, imgy: u32, random_state: &RandomState, output_path: &str) {
    // Visualize a sequence of floats as a 2d plot
    // noise_1d produces a smooth transition of values between integers
    let mut imgbuf = image::RgbImage::new(imgx, imgy);
    for x in 0..imgx - 1 {
        let y = base_noise::noise_1d(&random_state, x as f32 / 40.0, false);
        // println!("{}", y);
        let pixel = imgbuf.get_pixel_mut(x, (y * imgy as f32).floor() as u32);
        *pixel = image::Rgb([255, 255, 255]);
    }
    imgbuf.save(output_path).unwrap();
}

fn test_2d_value_noise(imgx: u32, imgy: u32, random_state: &RandomState, output_path: &str) {
    // Generate an image with random values at integer locations and smooth
    // transitions between. Looks like a blurred grid of squares
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let value = base_noise::noise_2d(
            &random_state,
            Vector2::new(x as f32 / 40.0, y as f32 / 40.0),
            false,
        );
        *pixel = image::Luma([(value * 80.0).floor() as u8]);
    }
    imgbuf.save(output_path).unwrap();
}

fn perlin_value_to_color(
    perlin_value: f32,
    negative_rgb: Vector3<f32>,
    zero_rgb: Vector3<f32>,
    positive_rgb: Vector3<f32>,
    blend_zero: bool,
    blend_curve_exponent: f32,
) -> image::Rgb<u8> {
    let mut current_color = zero_rgb;
    let mut perlin_multiplier = 1.0;
    if perlin_value < -0.005 {
        current_color = negative_rgb;
        perlin_multiplier = perlin_value.abs();
    } else if perlin_value > 0.005 {
        current_color = positive_rgb;
        perlin_multiplier = perlin_value;
    }

    perlin_multiplier = 1.0 - (1.0 - perlin_multiplier).powf(blend_curve_exponent);

    // If zero_rgb is (0, 0, 0) then both cases should behave the same
    let mut result_float_color = Vector3::new(0.0, 0.0, 0.0);
    if blend_zero {
        result_float_color =
            perlin_multiplier * current_color + (1.0 - perlin_multiplier) * zero_rgb;
    } else {
        result_float_color = perlin_multiplier * current_color;
    }

    return image::Rgb([
        (result_float_color.x * 255.0) as u8,
        (result_float_color.y * 255.0) as u8,
        (result_float_color.z * 255.0) as u8,
    ]);
}

fn main() {
    // Creating a hasher from RandomState with a particular seed always
    // hashes the same inputs to the same outputs.
    // Is not dependent on other hashers that have been made from the RandomState.
    // let s = RandomState::with_seeds(678091, 323, 1981243789, 90123);
    let s = RandomState::with_seeds(6791, 33423, 1243893479, 1223);

    // Debug colors
    // let negative_rgb = Vector3::new(1.0, 0.0, 0.0);
    // let zero_rgb = Vector3::new(0.0, 1.0, 0.0);
    // let positive_rgb = Vector3::new(0.0, 0.0, 1.0);
    // let blend_zero = false;
    // let blend_curve_exponent = 1.0;

    // Landscape colors
    let negative_rgb = Vector3::new(0.09, 0.42, 0.0);
    let zero_rgb = Vector3::new(0.123, 0.22, 0.64);
    let positive_rgb = Vector3::new(0.0, 0.65, 0.08);
    let blend_zero = true;
    let blend_curve_exponent = 10.0;

    let imgx = 800;
    let imgy = 800;

    test_1d_noise(imgx, imgy, &s, "smooth_value_noise_1d.png");

    test_2d_value_noise(imgx, imgy, &s, "smooth_value_noise_2d.png");

    // Generate an image of a single layer of perlin noise
    let mut imgbuf = image::RgbImage::new(imgx, imgy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let value = base_noise::perlin_layer_2d(
            &s,
            // Vector2::new(x as f32 / 400.0, y as f32 / 400.0),
            Vector2::new(x as f32 / 200.0, y as f32 / 200.0),
            false,
        );
        *pixel = perlin_value_to_color(
            value,
            negative_rgb,
            zero_rgb,
            positive_rgb,
            blend_zero,
            blend_curve_exponent,
        );
    }
    imgbuf.save("perlin_2d.png").unwrap();

    // Generate "multi-octave" perlin noise
    // Probably doesn't meet the formal definition
    let mut imgbuf = image::RgbImage::new(imgx, imgy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut value = 0.0;
        value += 0.5
            * base_noise::perlin_layer_2d(
                &s,
                Vector2::new(x as f32 / 200.0, y as f32 / 200.0),
                false,
            );
        value += 0.25
            * base_noise::perlin_layer_2d(
                &s,
                Vector2::new(x as f32 / 100.0, y as f32 / 100.0),
                false,
            );
        value += 0.125
            * base_noise::perlin_layer_2d(
                &s,
                Vector2::new(x as f32 / 50.0, y as f32 / 50.0),
                false,
            );
        value += 0.0625
            * base_noise::perlin_layer_2d(
                &s,
                Vector2::new(x as f32 / 25.0, y as f32 / 25.0),
                false,
            );
        *pixel = perlin_value_to_color(
            value,
            negative_rgb,
            zero_rgb,
            positive_rgb,
            blend_zero,
            blend_curve_exponent,
        );
    }
    imgbuf.save("perlin_2d_multi.png").unwrap();

    // TODO: ideas to try
    // * when the random vector at each grid point had only positive components,
    //   the output was similar to perlin noise but was much clearer that it was on a grid.
    //   Could play with vectors limited to various ranges
}
