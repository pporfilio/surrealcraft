// use rustc_hash::FxHasher;
// use std::collections::hash_map::RandomState;

use ahash::RandomState;
use cgmath::Vector2;
use image;
mod base_noise;

fn main() {
    // Creating a hasher from RandomState with a particular seed always
    // hashes the same inputs to the same outputs.
    // Is not dependent on other hashers that have been made from the RandomState.
    // let s = RandomState::with_seeds(678091, 323, 1981243789, 90123);
    let s = RandomState::with_seeds(6791, 33423, 1243893479, 1223);

    let imgx = 800;
    let imgy = 800;

    // Visualize a sequence of floats as a 2d plot
    // noise_1d produces a smooth transition of values between integers
    let mut imgbuf = image::RgbImage::new(imgx, imgy);
    for x in 0..imgx - 1 {
        let y = base_noise::noise_1d(&s, x as f32 / 40.0, false);
        // println!("{}", y);
        let pixel = imgbuf.get_pixel_mut(x, (y * imgy as f32).floor() as u32);
        *pixel = image::Rgb([255, 255, 255]);
    }
    imgbuf.save("smooth_value_noise_1d.png").unwrap();

    // Generate an image with random values at integer locations and smooth
    // transitions between. Looks like a blurred grid of squares
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let value = base_noise::noise_2d(&s, Vector2::new(x as f32 / 40.0, y as f32 / 40.0), false);
        *pixel = image::Luma([(value * 80.0).floor() as u8]);
    }
    imgbuf.save("smooth_value_noise_2d.png").unwrap();

    // Generate an image of a single layer of perlin noise
    let mut imgbuf = image::RgbImage::new(imgx, imgy);
    let mut values = Vec::<f32>::new();
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let value = base_noise::perlin_layer_2d(
            &s,
            Vector2::new(x as f32 / 400.0, y as f32 / 400.0),
            false,
        );
        if value < -0.01 {
            *pixel = image::Rgb([(-1.0 * value * 255.0).floor() as u8, 0, 0]);
        } else if value > 0.01 {
            *pixel = image::Rgb([0, 0, (value * 255.0).floor() as u8]);
        } else {
            *pixel = image::Rgb([0, 255, 0]);
        }
        values.push(value);
    }
    imgbuf.save("perlin_2d.png").unwrap();

    // TODO: ideas to try
    // * when the random vector at each grid point had only positive components,
    //   the output was similar to perlin noise but was much clearer that it was on a grid.
    //   Could play with vectors limited to various ranges
}
