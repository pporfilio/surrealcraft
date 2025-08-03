// use rustc_hash::FxHasher;
// use std::collections::hash_map::RandomState;

use std::hash::{self, BuildHasher, Hasher};

use ahash::RandomState;
use cgmath::{InnerSpace, Vector3};
use image;

fn u64_to_f32(x: u64) -> f32 {
    // Hash values are always u64, but I want an f32 between 0 and 1.
    // Convert to value within range of u32, but store as an f64 to maintain
    // full precision
    let x = x % u32::MAX as u64;
    return (x as f64 / u32::MAX as f64) as f32;
}

fn hash_i32_to_f32_value(hash_state: &RandomState, value: i32) -> f32 {
    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(value);
    return u64_to_f32(hasher.finish());
}

fn hash_i32_vector_to_f32_vector(hash_state: &RandomState, vector: Vector3<i32>) -> Vector3<f32> {
    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(12345);
    hasher.write_i32(vector.x ^ 3);
    hasher.write_i32(vector.y);
    hasher.write_i32(vector.z);
    let x = u64_to_f32(hasher.finish());

    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(80090021);
    hasher.write_i32(vector.y);
    hasher.write_i32(vector.z ^ 9999);
    hasher.write_i32(vector.x);
    let y = u64_to_f32(hasher.finish());

    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(999067821);
    hasher.write_i32(vector.z);
    hasher.write_i32(vector.x);
    hasher.write_i32(vector.y ^ 987654321);
    let z = u64_to_f32(hasher.finish());

    // return Vector3::new(x, y, z).normalize();
    let mut result = Vector3::new(x, y, z).normalize();
    // result.y = 0.0;
    // result.z = 0.0;
    return result;
}

fn hash_i32_vector_to_f32_value(hash_state: &RandomState, vector: Vector3<i32>) -> f32 {
    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(vector.x);
    hasher.write_i32(vector.y);
    hasher.write_i32(vector.z);
    return u64_to_f32(hasher.finish());
}

fn hash_vector(hash_state: &RandomState, x: i32, y: i32, z: i32) -> Vector3<f32> {
    return hash_i32_vector_to_f32_vector(hash_state, Vector3::new(x, y, z));
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    return a + t * (b - a);
}

fn fade(t: f32) -> f32 {
    // let t_64 = t as f64;

    // let t_3 = t_64 * t_64 * t_64;
    // let t_4 = t_3 * t_64;
    // let t_5 = t_4 * t_64;
    // let result = 6.0 * t_5 - 15.0 * t_4 + 10.0 * t_3;

    // return result as f32;
    return t;
}

fn perlin_layer_2(hash_state: &RandomState, location: Vector3<f32>, debug: bool) -> f32 {
    let x_floor = location.x.floor() as i32;
    let y_floor = location.y.floor() as i32;
    let z_floor = location.z.floor() as i32;
    let x_ceil = x_floor + 1;
    let y_ceil = y_floor + 1;
    let z_ceil = z_floor + 1;

    let x_floor_disp = location.x - location.x.floor();
    let y_floor_disp = location.y - location.y.floor();
    let z_floor_disp = location.z - location.z.floor();

    let x_ceil_disp = location.x - location.x.ceil();
    let y_ceil_disp = location.y - location.y.ceil();
    let z_ceil_disp = location.z - location.z.ceil();

    let s = hash_state;

    let displacement =
        location - Vector3::new(location.x.floor(), location.y.floor(), location.z.floor());
    let x_fade = fade(displacement.x);
    let y_fade = fade(displacement.y);
    let z_fade = fade(displacement.z);

    let result = lerp(
        z_fade,
        lerp(
            y_fade,
            lerp(
                x_fade,
                Vector3::new(x_floor_disp, y_floor_disp, z_floor_disp)
                    .dot(hash_vector(&s, x_floor, y_floor, z_floor)),
                Vector3::new(x_ceil_disp, y_floor_disp, z_floor_disp)
                    .dot(hash_vector(&s, x_ceil, y_floor, z_floor)),
            ),
            lerp(
                x_fade,
                Vector3::new(x_floor_disp, y_ceil_disp, z_floor_disp)
                    .dot(hash_vector(&s, x_floor, y_ceil, z_floor)),
                Vector3::new(x_ceil_disp, y_ceil_disp, z_floor_disp)
                    .dot(hash_vector(&s, x_ceil, y_ceil, z_floor)),
            ),
        ),
        lerp(
            y_fade,
            lerp(
                x_fade,
                Vector3::new(x_floor_disp, y_floor_disp, z_ceil_disp)
                    .dot(hash_vector(&s, x_floor, y_floor, z_ceil)),
                Vector3::new(x_ceil_disp, y_floor_disp, z_ceil_disp)
                    .dot(hash_vector(&s, x_ceil, y_floor, z_ceil)),
            ),
            lerp(
                x_fade,
                Vector3::new(x_floor_disp, y_ceil_disp, z_ceil_disp)
                    .dot(hash_vector(&s, x_floor, y_ceil, z_ceil)),
                Vector3::new(x_ceil_disp, y_ceil_disp, z_ceil_disp)
                    .dot(hash_vector(&s, x_ceil, y_ceil, z_ceil)),
            ),
        ),
    );

    return (result + 1.0) / 2.0;
}

fn perlin_layer(hash_state: &RandomState, location: Vector3<f32>, debug: bool) -> f32 {
    let x_floor = location.x.floor() as i32;
    let y_floor = location.y.floor() as i32;
    let z_floor = location.z.floor() as i32;
    let x_ceil = x_floor + 1;
    let y_ceil = y_floor + 1;
    let z_ceil = z_floor + 1;

    // For each vertex of the cube
    //    Find the vector from the corner to location
    //    Find the random vector at the corner
    //    compute the dot product of these two vectors
    //    scale the effect from each corner by the location's distance from that corner
    // sum these values?

    // We compute the gradient between each corner and our point, then we want to
    // interpolate between the gradients. We interpolate between pairs of
    // corners along the x direction by the x amount, then we interpolate between
    // pairs of x-interpolations along the y direction, and finally interpolate those
    // two numbers along the z direction.
    // The interpolation is linear along the fade function,
    // not linear in the displacement, I think.

    let s = hash_state;

    let displacement =
        location - Vector3::new(location.x.floor(), location.y.floor(), location.z.floor());
    let x_fade = fade(displacement.x);
    let y_fade = fade(displacement.y);
    let z_fade = fade(displacement.z);

    // Note that for each of the x_fade lerps, the corner vectors have x_floor and x_ceil
    // For the y_fade lerps, the corner vectors in the sub-lerps have y_floor and y_ceil
    // For the z_fade lerp, the z_floor and z_ceil are split between the first and second
    // half of the sub-lerps.

    let d = displacement;
    let floor_hash = hash_vector(&s, x_floor, y_floor, z_floor);
    let ceil_hash = hash_vector(&s, x_ceil, y_floor, z_floor);
    let floor_grad = Vector3::new(1.0 - d.x, 1.0 - d.y, 1.0 - d.z)
        .dot(hash_vector(&s, x_floor, y_floor, z_floor));
    let ceil_grad =
        Vector3::new(d.x, 1.0 - d.y, 1.0 - d.z).dot(hash_vector(&s, x_ceil, y_floor, z_floor));

    if debug {
        println!("x_floor {},\tx_ceil {},\tx_fade {},\tfloor_hash {},\tceil_hash {},\tfloor_grad {},\tceil_grad {},\tresult {}",
                x_floor, x_ceil, x_fade, floor_hash.x, ceil_hash.x, floor_grad, ceil_grad,
                lerp(x_fade, floor_grad, ceil_grad));
    }

    let result = lerp(
        z_fade,
        lerp(
            y_fade,
            lerp(
                x_fade,
                Vector3::new(1.0 - d.x, 1.0 - d.y, 1.0 - d.z)
                    .dot(hash_vector(&s, x_floor, y_floor, z_floor)),
                Vector3::new(d.x, 1.0 - d.y, 1.0 - d.z)
                    .dot(hash_vector(&s, x_ceil, y_floor, z_floor)),
            ),
            lerp(
                x_fade,
                Vector3::new(1.0 - d.x, d.y, 1.0 - d.z)
                    .dot(hash_vector(&s, x_floor, y_ceil, z_floor)),
                Vector3::new(d.x, d.y, 1.0 - d.z).dot(hash_vector(&s, x_ceil, y_ceil, z_floor)),
            ),
        ),
        lerp(
            y_fade,
            lerp(
                x_fade,
                Vector3::new(1.0 - d.x, 1.0 - d.y, d.z)
                    .dot(hash_vector(&s, x_floor, y_floor, z_ceil)),
                Vector3::new(d.x, 1.0 - d.y, d.z).dot(hash_vector(&s, x_ceil, y_floor, z_ceil)),
            ),
            lerp(
                x_fade,
                Vector3::new(1.0 - d.x, d.y, d.z).dot(hash_vector(&s, x_floor, y_ceil, z_ceil)),
                Vector3::new(d.x, d.y, d.z).dot(hash_vector(&s, x_ceil, y_ceil, z_ceil)),
            ),
        ),
    );

    // let mut result = 0.0;
    // for x in vec![x_floor, x_ceil] {
    //     for y in vec![y_floor, y_ceil] {
    //         for z in vec![z_floor, z_ceil] {
    //             let displacement = location - Vector3::new(x, y, z);
    //             println!(
    //                 "displacement {}, {}, {}",
    //                 displacement.x, displacement.y, displacement.z
    //             );
    //             let corner_vector = hash_i32_vector_to_f32_vector(
    //                 hash_state,
    //                 Vector3::new(x as i32, y as i32, z as i32),
    //             );
    //             let gradient = displacement.dot(corner_vector);
    //             println!("gradient {}", gradient);

    //             // let fade_x = fade(displacement.x.abs());
    //             // let fade_y = fade(displacement.y.abs());
    //             // let fade_z = fade(displacement.z.abs());

    //             // let fade_x = fade(displacement.x);
    //             // let fade_y = fade(displacement.y);
    //             // let fade_z = fade(displacement.z);

    //             println!("{}", displacement.magnitude());
    //             let fade = fade(displacement.magnitude());

    //             println!("fade: {}", fade);
    //             // println!("fade: {}, {}, {}", fade_x, fade_y, fade_z);

    //             // result += gradient * fade_x * fade_y * fade_z;
    //             result += gradient * fade;
    //         }
    //     }
    // }

    // println!("{}", result);
    return (result + 1.0) / 2.0;
    // return result;
}

#[allow(unused)]
fn _main() {
    let s = RandomState::with_seeds(678091, 323, 1981243789, 90123);

    // for x in 0..5 {
    //     for y in 0..5 {
    //         println!(
    //             "Hash is {}",
    //             hash_i32_vector_to_f32_value(&s, Vector3::new(x, y, 0))
    //         );
    //     }
    // }

    // println!("{}", u32::MAX);

    // println!("{}", 396062919898711887 % u32::MAX as u64);

    // println!("--------------------");

    for x in vec![
        0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0, 1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7,
        1.8, 1.9, 2.0,
    ] {
        perlin_layer_2(&s, Vector3::new(x, 0.0, 0.0), false);
        // println!("{}", perlin_layer(&s, Vector3::new(x, 0.5, 0.5)));
    }

    // let result = perlin_layer(&s, Vector3::new(0.0, 0.0, 0.0));
    // println!("{}", result);

    let imgx = 800;
    let imgy = 800;
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // let value = perlin_layer(
        //     &s,
        //     Vector3::new((x / 20) as f32 + 0.1, 0.0, 0.0),
        //     y == 0 && x < 70,
        // );
        // let value = perlin_layer(
        //     &s,
        //     Vector3::new(x as f32 / 20.0, 0.0, 0.0),
        //     y == 0 && x < 70,
        // );

        let value = perlin_layer(
            &s,
            Vector3::new(x as f32 / 40.0, y as f32 / 40.0, 0.0),
            false,
        );
        *pixel = image::Luma([(value * 80.0).floor() as u8]);
    }

    imgbuf.save("test.png").unwrap();
}

use cgmath::Vector2;

#[allow(unused)]
fn __main() {
    let hv1 = Vector2::new(0.8, 0.4);
    let hv2 = Vector2::new(0.8, 0.4);
    let points = [
        Vector2::new(0.0, 0.0),
        Vector2::new(0.25, 0.0),
        Vector2::new(0.5, 0.0),
        Vector2::new(0.75, 0.0),
        Vector2::new(1.0, 0.0),
        //Vector2::new(1.25, 0.0),
    ];

    let mut left_grad = Vec::<f32>::new();
    let mut right_grad = Vec::<f32>::new();
    let mut lerped = Vec::<f32>::new();
    for point in points {
        left_grad.push((Vector2::new(1.0, 1.0) - point).normalize().dot(hv1));
        right_grad.push(Vector2::new(point.x, 1.0).normalize().dot(hv2));
        lerped.push(
            left_grad.last().unwrap() * (1.0 - point.x) + right_grad.last().unwrap() * (point.x),
        );
    }

    println!("{:?}", left_grad);
    println!("{:?}", right_grad);
    println!("{:?}", lerped);
}

mod base_noise;

fn main() {
    // Creating a hasher from RandomState with a particular seed always
    // hashes the same inputs to the same outputs.
    // Is not dependent on other hashers that have been made from the RandomState.
    let s = RandomState::with_seeds(678091, 323, 1981243789, 90123);

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
    imgbuf.save("1d.png").unwrap();

    // Generate an image with random values at integer locations and smooth
    // transitions between. Looks like a blurred grid of squares
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let value = base_noise::noise_2d(&s, Vector2::new(x as f32 / 40.0, y as f32 / 40.0), false);
        *pixel = image::Luma([(value * 80.0).floor() as u8]);
    }
    imgbuf.save("2d.png").unwrap();

    // Generate an image of a single layer of perlin noise
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    let mut values = Vec::<f32>::new();
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let value = base_noise::perlin_layer_2d(
            &s,
            Vector2::new(
                (x as f32 / 200.0) - 1.0 / 400.0,
                (y as f32 / 200.0) - 1.0 / 400.0,
            ),
            false,
        );
        *pixel = image::Luma([((value + 0.5) * 250.0).floor() as u8]);
        values.push(value);
    }
    imgbuf.save("perlin_2d.png").unwrap();

    // for x in 0..10 {
    //     for y in 0..10 {
    //         println!(
    //             "({}, {}): {:?}",
    //             x,
    //             y,
    //             base_noise::hash_i32_vec2_to_f32_vec2(&s, Vector2::new(x, y))
    //         )
    //     }
    // }

    let mut min = 1000.0;
    let mut max = -1000.0;
    for v in values {
        if v < min {
            min = v;
        }
        if v > max {
            max = v;
        }
    }

    println!("min: {}", min);
    println!("max: {}", max);
}
