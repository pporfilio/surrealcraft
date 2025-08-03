use ahash::RandomState;
use cgmath::InnerSpace;
use cgmath::Vector2;
use std::hash::{self, BuildHasher, Hasher};

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

fn hash_i32_vec2_to_f32_value(hash_state: &RandomState, vector: Vector2<i32>) -> f32 {
    let mut hasher = hash_state.build_hasher();
    hasher.write_i32(vector.x);
    hasher.write_i32(vector.y);
    return u64_to_f32(hasher.finish());
}

pub fn hash_i32_vec2_to_f32_vec2(hash_state: &RandomState, vector: Vector2<i32>) -> Vector2<f32> {
    // Maps a point at integer coordinates to an arbitrary 2d vector.
    // Output vector is normalized.
    // Both the x and y coordinate are used in generating both points, so that
    // (a, b) and (b, a) produce unrelated output.
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

    let mut result = Vector2::new(x, y).normalize();
    return result;
}

fn lerp(t: f32, a: f32, b: f32) -> f32 {
    return a + t * (b - a);
}

fn fade(t: f32) -> f32 {
    let t_64 = t as f64;

    let t_3 = t_64 * t_64 * t_64;
    let t_4 = t_3 * t_64;
    let t_5 = t_4 * t_64;
    let result = 6.0 * t_5 - 15.0 * t_4 + 10.0 * t_3;

    return result as f32;
}

pub fn noise_1d(hash_state: &RandomState, location: f32, debug: bool) -> f32 {
    let x_floor = location.floor();
    let x_ceil = x_floor + 1.0;
    let left = hash_i32_to_f32_value(hash_state, x_floor as i32);
    let right = hash_i32_to_f32_value(hash_state, x_ceil as i32);
    return lerp(fade(location - x_floor), left, right);
}

pub fn noise_2d(hash_state: &RandomState, location: Vector2<f32>, debug: bool) -> f32 {
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

pub fn perlin_layer_2d(hash_state: &RandomState, location: Vector2<f32>, debug: bool) -> f32 {
    // Produces noise according to a single octave of the perlin noise algorithm
    let x_floor = location.x.floor();
    let y_floor = location.y.floor();
    let x_ceil = x_floor + 1.0;
    let y_ceil = y_floor + 1.0;

    let ul_grad =
        hash_i32_vec2_to_f32_vec2(hash_state, Vector2::new(x_floor as i32, y_floor as i32));
    let ur_grad =
        hash_i32_vec2_to_f32_vec2(hash_state, Vector2::new(x_ceil as i32, y_floor as i32));
    let ll_grad =
        hash_i32_vec2_to_f32_vec2(hash_state, Vector2::new(x_floor as i32, y_ceil as i32));
    let lr_grad = hash_i32_vec2_to_f32_vec2(hash_state, Vector2::new(x_ceil as i32, y_ceil as i32));

    let ul_value = ul_grad.dot(Vector2::new(location.x - x_floor, location.y - y_floor));
    let ur_value = ur_grad.dot(Vector2::new(location.x - x_ceil, location.y - y_floor));
    let ll_value = ll_grad.dot(Vector2::new(location.x - x_floor, location.y - y_ceil));
    let lr_value = lr_grad.dot(Vector2::new(location.x - x_ceil, location.y - y_ceil));

    return lerp(
        fade(location.y - y_floor),
        lerp(fade(location.x - x_floor), ul_value, ur_value),
        lerp(fade(location.x - x_floor), ll_value, lr_value),
    );
}
