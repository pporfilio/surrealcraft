// use rustc_hash::FxHasher;
// use std::collections::hash_map::RandomState;

use ahash::RandomState;
use cgmath::Vector2;
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
        let value = base_noise::noise_2d(&random_state, Vector2::new(x as f32 / 40.0, y as f32 / 40.0), false);
        *pixel = image::Luma([(value * 80.0).floor() as u8]);
    }
    imgbuf.save(output_path).unwrap();
}



fn main() {
    // Creating a hasher from RandomState with a particular seed always
    // hashes the same inputs to the same outputs.
    // Is not dependent on other hashers that have been made from the RandomState.
    // let s = RandomState::with_seeds(678091, 323, 1981243789, 90123);
    let s = RandomState::with_seeds(6791, 33423, 1243893479, 1223);

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
        if value < -0.01 {
            *pixel = image::Rgb([(-1.0 * value * 255.0).floor() as u8, 0, 0]);
        } else if value > 0.01 {
            *pixel = image::Rgb([0, 0, (value * 255.0).floor() as u8]);
        } else {
            *pixel = image::Rgb([0, 255, 0]);
        }
    }
    imgbuf.save("perlin_2d.png").unwrap();


    // Generate "multi-octave" perlin noise
    // Probably doesn't meet the formal definition
    let mut imgbuf = image::RgbImage::new(imgx, imgy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let mut value = 0.0;
        value += 0.5 * base_noise::perlin_layer_2d(&s, Vector2::new(x as f32 / 200.0, y as f32 / 200.0), false);
        value += 0.25 * base_noise::perlin_layer_2d(&s, Vector2::new(x as f32 / 100.0, y as f32 / 100.0), false);
        value += 0.125 * base_noise::perlin_layer_2d(&s, Vector2::new(x as f32 / 50.0, y as f32 / 50.0), false);
        value += 0.0625 * base_noise::perlin_layer_2d(&s, Vector2::new(x as f32 / 25.0, y as f32 / 25.0), false);
        if value < -0.001 {
            *pixel = image::Rgb([(-1.0 * value * 255.0).floor() as u8, 0, 0]);
        } else if value > 0.001 {
            *pixel = image::Rgb([0, 0, (value * 255.0).floor() as u8]);
        } else {
            *pixel = image::Rgb([0, 255, 0]);
        }
    }
    imgbuf.save("perlin_2d_multi.png").unwrap();


    // TODO: ideas to try
    // * when the random vector at each grid point had only positive components,
    //   the output was similar to perlin noise but was much clearer that it was on a grid.
    //   Could play with vectors limited to various ranges
}
