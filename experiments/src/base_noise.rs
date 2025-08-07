use ahash::RandomState;
use cgmath::InnerSpace;
use cgmath::Vector2;
use cgmath::Vector3;
use std::hash::{BuildHasher, Hasher};

fn u64_to_f32(x: u64) -> f32 {
    // Hash values are always u64, but I want an f32 between 0 and 1.
    // Convert to value within range of u32, but store as an f64 to maintain
    // full precision
    // Update: I'm not sure why I did it exactly like this and of course there are
    // collisions, both due to the mod and probably for adjacent values because of
    // the downcast to f32.
    let x = x % u32::MAX as u64;
    return (x as f64 / u32::MAX as f64) as f32;
}

fn hash_i32_to_f32_value(hash_state: &RandomState, value: i32) -> f32 {
    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(value);
    return u64_to_f32(hasher.finish());
}

fn hash_i32_vec2_to_f32_value(hash_state: &RandomState, vector: Vector2<i32>) -> f32 {
    // Maps a point at integer coordinates to an arbitrary
    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(vector.x);
    hasher.write_i32(vector.y);
    return u64_to_f32(hasher.finish());
}

#[allow(unused)]
fn hash_i32_vec3_to_f32_value(hash_state: &RandomState, vector: Vector3<i32>) -> f32 {
    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(vector.x);
    hasher.write_i32(vector.y);
    hasher.write_i32(vector.z);
    return u64_to_f32(hasher.finish());
}

pub fn hash_i32_vec2_to_f32_vec2(hash_state: &RandomState, vector: Vector2<i32>) -> Vector2<f32> {
    // Maps a point at integer coordinates to an arbitrary 2d vector.
    // Output vector is normalized.
    // Both the x and y coordinate are used in generating both points, so that the output
    // is unique per point, i.e. inputs with the same x coordinate do not produce
    // outputs with the same x coordinate.
    // Output vector is normalized.
    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(12345);
    hasher.write_i32(vector.x);
    hasher.write_i32(vector.y);
    let x = u64_to_f32(hasher.finish());

    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(80090021);
    hasher.write_i32(vector.y);
    hasher.write_i32(vector.x);
    let y = u64_to_f32(hasher.finish());

    // u64_to_f32 returns a number in the range 0 to 1 inclusive
    // so subtract 0.5 to get negative values with equal probability.
    return Vector2::new(x - 0.5, y - 0.5 ).normalize();
}


fn hash_i32_vec3_to_f32_vec3(hash_state: &RandomState, vector: Vector3<i32>) -> Vector3<f32> {
    // Similar approach to hash_i32_vec2_to_f32_vec2, but for vec3
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

    return Vector3::new(x - 0.5, y -0.5, z - 0.5).normalize();
}


fn lerp(t: f32, a: f32, b: f32) -> f32 {
    return a + t * (b - a);
}

fn fade(t: f32) -> f32 {
    // Improved perlin noise fade function
    let t_64 = t as f64;

    let t_3 = t_64 * t_64 * t_64;
    let t_4 = t_3 * t_64;
    let t_5 = t_4 * t_64;
    let result = 6.0 * t_5 - 15.0 * t_4 + 10.0 * t_3;

    return result as f32;
}

pub fn noise_1d(hash_state: &RandomState, location: f32, _debug: bool) -> f32 {
    // Produces noise with distinct values for integer inputs and smooth
    // transitions inbetween.
    let x_floor = location.floor();
    let x_ceil = x_floor + 1.0;
    let left = hash_i32_to_f32_value(hash_state, x_floor as i32);
    let right = hash_i32_to_f32_value(hash_state, x_ceil as i32);
    return lerp(fade(location - x_floor), left, right);
}

pub fn noise_2d(hash_state: &RandomState, location: Vector2<f32>, _debug: bool) -> f32 {
    // Produces noise with distinct values at each integer coordinates and smooth
    // transitions inbetween.
    let x_floor = location.x.floor();
    let y_floor = location.y.floor();
    let x_ceil = x_floor + 1.0;
    let y_ceil = y_floor + 1.0;

    let ul_value =
        hash_i32_vec2_to_f32_value(hash_state, Vector2::new(x_floor as i32, y_floor as i32));
    let ur_value =
        hash_i32_vec2_to_f32_value(hash_state, Vector2::new(x_ceil as i32, y_floor as i32));
    let ll_value =
        hash_i32_vec2_to_f32_value(hash_state, Vector2::new(x_floor as i32, y_ceil as i32));
    let lr_value =
        hash_i32_vec2_to_f32_value(hash_state, Vector2::new(x_ceil as i32, y_ceil as i32));

    return lerp(
        fade(location.y - y_floor),
        lerp(fade(location.x - x_floor), ul_value, ur_value),
        lerp(fade(location.x - x_floor), ll_value, lr_value),
    );
}

pub fn perlin_layer_2d(hash_state: &RandomState, location: Vector2<f32>, _debug: bool) -> f32 {
    // Produces noise according to a single octave of the perlin noise algorithm,
    // except modified to use arbitrary vectors at each coordinate point and not using
    // the original permutation table. This also means that the pattern does not repeat/tile
    // Also uses the old version of random gradient vectors (rather than a set of 12 from improved
    // Perlin noise).
    // But does use the updated smoothstep function.
    let x_floor = location.x.floor();
    let y_floor = location.y.floor();
    let x_ceil = x_floor + 1.0;
    let y_ceil = y_floor + 1.0;

    // Get the vectors for each grid point surrounding the location.
    // These are pseudo-random but will always be the same for the same grid point
    // when the hash_state was initialized with the same seed.
    let ul_vector =
        hash_i32_vec2_to_f32_vec2(hash_state, Vector2::new(x_floor as i32, y_floor as i32));
    let ur_vector =
        hash_i32_vec2_to_f32_vec2(hash_state, Vector2::new(x_ceil as i32, y_floor as i32));
    let ll_vector =
        hash_i32_vec2_to_f32_vec2(hash_state, Vector2::new(x_floor as i32, y_ceil as i32));
    let lr_vector = hash_i32_vec2_to_f32_vec2(hash_state, Vector2::new(x_ceil as i32, y_ceil as i32));

    if _debug {
        println!("ul: ({}, {}) ur: ({}, {}) ll: ({}, {}) lr: ({}, {})", ul_vector.x, ul_vector.y, ur_vector.x, ur_vector.y, ll_vector.x, ll_vector.y, lr_vector.x, lr_vector.y);
    }

    let ul_value = ul_vector.dot(Vector2::new(location.x - x_floor, location.y - y_floor));
    let ur_value = ur_vector.dot(Vector2::new(location.x - x_ceil, location.y - y_floor));
    let ll_value = ll_vector.dot(Vector2::new(location.x - x_floor, location.y - y_ceil));
    let lr_value = lr_vector.dot(Vector2::new(location.x - x_ceil, location.y - y_ceil));

    let u = fade(location.x - x_floor);
    let v = fade(location.y - y_floor);

    return lerp(v,
        lerp(u, ul_value, ur_value),
        lerp(u, ll_value, lr_value)
    );
}

fn hash_vector(hash_state: &RandomState, x: i32, y: i32, z: i32) -> Vector3<f32> {
    return hash_i32_vec3_to_f32_vec3(hash_state, Vector3::new(x, y, z));
}

fn perlin_layer_3d_draft_2(hash_state: &RandomState, location: Vector3<f32>, _debug: bool) -> f32 {
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

fn perlin_layer_3d_draft_1(hash_state: &RandomState, location: Vector3<f32>, debug: bool) -> f32 {
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
    //             let corner_vector = hash_i32_vec3_to_f32_vec3(
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