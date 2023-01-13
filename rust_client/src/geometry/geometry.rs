use cgmath::InnerSpace;

use super::voxels::{Voxel, VoxelData};

pub struct TriangleMesh {
    pub vertices: Vec<cgmath::Vector3<f32>>,
    pub indices: Vec<u32>,
    pub colors: Vec<cgmath::Vector3<f32>>,
}

impl TriangleMesh {
    pub fn new(vertex_count_hint: usize, index_count_hint: usize) -> Self {
        let mut vertices: Vec<cgmath::Vector3<f32>> = Vec::new();
        let mut indices = Vec::new();
        let mut colors: Vec<cgmath::Vector3<f32>> = Vec::new();

        vertices.reserve(vertex_count_hint);
        colors.reserve(vertex_count_hint);
        indices.reserve(index_count_hint);

        Self {
            vertices,
            indices,
            colors,
        }
    }

    pub fn trim(&mut self) {
        self.vertices.shrink_to_fit();
        self.indices.shrink_to_fit();
        self.colors.shrink_to_fit();
    }
}

pub fn collision_mesh_1() -> TriangleMesh {
    let color = cgmath::Vector3::new(115.0 / 255.0, 147.0 / 255.0, 179.0 / 255.0);
    let mut tm = TriangleMesh::new(200, 200);

    // Square at x = 5 facing toward origin
    tm.vertices.push(cgmath::Vector3::new(5.1, 1.0, 0.5));
    tm.vertices.push(cgmath::Vector3::new(5.1, 1.0, -0.5));
    tm.vertices.push(cgmath::Vector3::new(4.9, -1.0, 0.5));
    tm.vertices.push(cgmath::Vector3::new(4.9, -1.0, -0.5));

    tm.indices.push(0);
    tm.indices.push(1);
    tm.indices.push(3);

    tm.indices.push(3);
    tm.indices.push(2);
    tm.indices.push(0);

    for _ in 0..4 {
        tm.colors.push(color);
    }

    tm
}

pub fn collision_mesh_2() -> TriangleMesh {
    let color = cgmath::Vector3::new(115.0 / 255.0, 147.0 / 255.0, 179.0 / 255.0);
    let mut tm = TriangleMesh::new(200, 200);

    tm.vertices.push(cgmath::Vector3::new(-5.0, 0.0, 0.0));
    tm.vertices.push(cgmath::Vector3::new(7.0, -5.0, 0.0));
    tm.vertices.push(cgmath::Vector3::new(7.0, 5.0, 0.0));

    tm.indices.push(0);
    tm.indices.push(1);
    tm.indices.push(2);

    for _ in 0..3 {
        tm.colors.push(color);
    }

    tm
}

pub const CUBE_SCALE: f32 = 0.5;

#[rustfmt::skip]
pub fn add_voxel(
    triangle_mesh: &mut TriangleMesh,
    voxel_center: cgmath::Vector3<f32>,
    color: cgmath::Vector3<f32>,
) {

    let start_index = triangle_mesh.vertices.len() as u32;

    triangle_mesh.vertices.push(cgmath::Vector3::new( CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxel_center); // 000 // 0
    triangle_mesh.vertices.push(cgmath::Vector3::new( CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxel_center); // 001 // 1
    triangle_mesh.vertices.push(cgmath::Vector3::new( CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxel_center); // 010 // 2
    triangle_mesh.vertices.push(cgmath::Vector3::new( CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxel_center); // 011 // 3
    triangle_mesh.vertices.push(cgmath::Vector3::new(-CUBE_SCALE,  CUBE_SCALE,  CUBE_SCALE) + voxel_center); // 100 // 4
    triangle_mesh.vertices.push(cgmath::Vector3::new(-CUBE_SCALE,  CUBE_SCALE, -CUBE_SCALE) + voxel_center); // 101 // 5
    triangle_mesh.vertices.push(cgmath::Vector3::new(-CUBE_SCALE, -CUBE_SCALE,  CUBE_SCALE) + voxel_center); // 110 // 6
    triangle_mesh.vertices.push(cgmath::Vector3::new(-CUBE_SCALE, -CUBE_SCALE, -CUBE_SCALE) + voxel_center); // 111 // 7


    for _ in 0..8 {
        triangle_mesh.colors.push(color);
    }

    // negative Z
    triangle_mesh.indices.push(start_index + 1);
    triangle_mesh.indices.push(start_index + 3);
    triangle_mesh.indices.push(start_index + 7);

    triangle_mesh.indices.push(start_index + 5);
    triangle_mesh.indices.push(start_index + 1);
    triangle_mesh.indices.push(start_index + 7);

    // positive Z
    triangle_mesh.indices.push(start_index + 6);
    triangle_mesh.indices.push(start_index + 2);
    triangle_mesh.indices.push(start_index + 0);

    triangle_mesh.indices.push(start_index + 0);
    triangle_mesh.indices.push(start_index + 4);
    triangle_mesh.indices.push(start_index + 6);

    // negative X
    triangle_mesh.indices.push(start_index + 4);
    triangle_mesh.indices.push(start_index + 5);
    triangle_mesh.indices.push(start_index + 7);

    triangle_mesh.indices.push(start_index + 7);
    triangle_mesh.indices.push(start_index + 6);
    triangle_mesh.indices.push(start_index + 4);

    // positive X
    triangle_mesh.indices.push(start_index + 3);
    triangle_mesh.indices.push(start_index + 1);
    triangle_mesh.indices.push(start_index + 0);

    triangle_mesh.indices.push(start_index + 0);
    triangle_mesh.indices.push(start_index + 2);
    triangle_mesh.indices.push(start_index + 3);

    // negative Y
    triangle_mesh.indices.push(start_index + 7);
    triangle_mesh.indices.push(start_index + 3);
    triangle_mesh.indices.push(start_index + 2);

    triangle_mesh.indices.push(start_index + 2);
    triangle_mesh.indices.push(start_index + 6);
    triangle_mesh.indices.push(start_index + 7);

    // positive Y
    triangle_mesh.indices.push(start_index + 0);
    triangle_mesh.indices.push(start_index + 1);
    triangle_mesh.indices.push(start_index + 5);

    triangle_mesh.indices.push(start_index + 5);
    triangle_mesh.indices.push(start_index + 4);
    triangle_mesh.indices.push(start_index + 0);    
}

// No array size
// Took 586.52ms
// Took 606.5143ms
// Took 615.2295ms

// Vec::resize()
// Took 820.3226ms
// Took 878.7217ms
// Took 871.9675ms

// Vec::reserve(), voxel count / 2
// Took 497.0261ms
// Took 446.4625ms
// Took 450.804ms

// Vec::reserve(), voxel count
// So this suggests that if I had slightly more than voxel count / 2,
// then I guess the last doubling and copy took the bulk of the time.
// So it looks like guessing but going low doesn't really help.
// Took 292.3084ms
// Took 314.831ms
// Took 258.3519ms

// Vec::reserve() voxel count, then Vec::shrink_to_fit() each array
// This way we avoid any copies but also only temporarily use the extra memory
// Took 283.9025ms
// Took 299.5491ms
// Took 287.8983ms
// And wow, this takes 2.3 _seconds_ in non-release build.

pub fn triangles_from_voxel_data(voxel_data: &VoxelData<Voxel>) -> TriangleMesh {
    use std::time::Instant;
    let start = Instant::now();

    let voxel_guess = voxel_data.dimensions().x as usize
        * voxel_data.dimensions().y as usize
        * voxel_data.dimensions().z as usize;

    let vertex_guess = 8 * voxel_guess;
    let index_guess = 36 * voxel_guess;

    let mut tm = TriangleMesh::new(vertex_guess, index_guess);

    let vd_x = voxel_data.dimensions().x;
    let vd_y = voxel_data.dimensions().y;
    let vd_z = voxel_data.dimensions().z;

    for x in 0..vd_x {
        for y in 0..vd_y {
            for z in 0..vd_z {
                let voxel_indices = cgmath::Vector3::new(x, y, z);
                let voxel = voxel_data.data_at(voxel_indices);
                if voxel.value != 0 {
                    let voxel_center = cgmath::Vector3::new(x as f32, y as f32, z as f32);
                    let color = cgmath::Vector3::new(voxel.r, voxel.g, voxel.b);
                    add_voxel(&mut tm, voxel_center, color)
                }
            }
        }
    }

    tm.trim();

    let stop = Instant::now();
    let delta_s = stop.saturating_duration_since(start);
    println!("Took {:?}", delta_s);

    tm
}

#[derive(PartialEq, Debug)]
pub enum IntersectionStatus {
    Crosses,
    NeverCrosses,
    NeverCrossesEmbedded,
}

/// Determines when a unit sphere starts and finishes intersecting/colliding a plane.
/// * `unit_sphere_start` - the initial position of the center of a unit sphere to check
/// * `unit_sphere_velocity` - a vector indicating how far the unit sphere will move
/// * `tp1`, `tp2`, `tp3` - three points that will be used to define a plane. Expected in CCW order.
///
/// Returns t0, t1 as the fraction of `unit_sphere_velocity` the sphere needs to travel
/// to start and finish colliding with the plane, respectively. 0 means the start position
/// and 1 means the end position of the sphere, but returned values could be
/// outside that range if the sphere is already colliding or does not collide with the plane.
/// Returns intersection point: the point where the sphere touches the plane at t0, whether
/// or not that point is within the triangle that defines the plane.
/// Returns IntersectionStatus to indicate if the velocity crosses the plane normally,
/// is parallel to the plane but the sphere is outside the plane, or is parallel to the
/// plane but the sphere is already intersecting the plane.
pub fn intersect_plane(
    unit_sphere_start: cgmath::Vector3<f32>,
    unit_sphere_velocity: cgmath::Vector3<f32>,
    tp1: cgmath::Vector3<f32>,
    tp2: cgmath::Vector3<f32>,
    tp3: cgmath::Vector3<f32>,
) -> (f32, f32, cgmath::Vector3<f32>, IntersectionStatus) {
    let plane_normal = (tp2 - tp1).cross(tp3 - tp1).normalize();
    // println!("normal: {:?}", plane_normal);
    let point_on_plane = tp1;

    let signed_distance = (unit_sphere_start - point_on_plane).dot(plane_normal);
    // println!("signed distance: {:?}", signed_distance);
    let denom = unit_sphere_velocity.dot(plane_normal);

    if denom.abs() < 0.00001 {
        if signed_distance < 1.0 {
            return (
                0.0,
                0.0,
                cgmath::Vector3::new(0.0, 0.0, 0.0),
                IntersectionStatus::NeverCrossesEmbedded,
            );
        } else {
            return (
                0.0,
                0.0,
                cgmath::Vector3::new(0.0, 0.0, 0.0),
                IntersectionStatus::NeverCrosses,
            );
        }
    }

    let t0 = (1.0 - signed_distance) / denom;
    let t1 = (-1.0 - signed_distance) / denom;

    let plane_intersection_point = unit_sphere_start - plane_normal + t0 * unit_sphere_velocity;

    (
        t0,
        t1,
        plane_intersection_point,
        IntersectionStatus::Crosses,
    )
}

/// If a point is coplanar with a triangle and inside the triangle, then it makes a
/// triangle with each pair of vertices that has the same winding order. To check this,
/// check if the normal of each of the smaller triangles is the same direction as the
/// normal of the original triangle. Assumes CCW order.
pub fn point_in_triangle(
    point: cgmath::Vector3<f32>,
    tp1: cgmath::Vector3<f32>,
    tp2: cgmath::Vector3<f32>,
    tp3: cgmath::Vector3<f32>,
) -> bool {
    let n = (tp3 - tp2).cross(tp1 - tp2);
    let na = (tp2 - tp1).cross(point - tp1);
    let nb = (tp3 - tp2).cross(point - tp2);
    let nc = (tp1 - tp3).cross(point - tp3);

    println!(
        "point_in_triangle point: {:?} tp1: {:?} tp2: {:?} tp3: {:?}",
        point, tp1, tp2, tp3
    );
    println!("na dot n: {:?}", na.dot(n));
    println!("nb dot n: {:?}", nb.dot(n));
    println!("nc dot n: {:?}", nc.dot(n));

    na.dot(n) > 0.0 && nb.dot(n) > 0.0 && nc.dot(n) > 0.0
}

/// Given a unit sphere starting location, a velocity, and a point to collide against,
/// determines the first time the outside of the sphere collides with the point moving
/// forward along the velocity. Returning 0 means the sphere starts touching the point.
/// Returning 1 means the sphere ends touching the point. Returning > 1 means the
/// sphere would eventually touch the point if it kept moving along the same velocity
/// vector.
///
/// Returns (t, bool) where t is how far along the velocity the sphere reaches the
/// point and bool is true if it touches eventually or false otherwise.
/// If the point starts inside the sphere, t will be the time when the point exits the
/// sphere.
pub fn collide_point(
    unit_sphere_start: cgmath::Vector3<f32>,
    unit_sphere_velocity: cgmath::Vector3<f32>,
    point: cgmath::Vector3<f32>,
) -> (f32, bool) {
    let a = unit_sphere_velocity.dot(unit_sphere_velocity);
    let b = 2.0 * unit_sphere_velocity.dot(unit_sphere_start - point);
    let c = (point - unit_sphere_start).dot(point - unit_sphere_start) - 1.0;

    let determinant = b * b - 4.0 * a * c;
    if determinant < 0.0 {
        return (0.0, false);
    }

    let sqrt_d = determinant.sqrt();
    let r1 = (-1.0 * b - sqrt_d) / (2.0 * a);
    let r2 = (-1.0 * b + sqrt_d) / (2.0 * a);

    let r_min = r1.min(r2);
    let r_max = r1.max(r2);

    if r_min >= 0.0 {
        return (r_min, true);
    } else if r_max >= 0.0 {
        return (r_max, true);
    } else {
        return (-1.0, false);
    }
}

// pub fn intersect_triangle(
//     unit_sphere_start: cgmath::Vector3<f32>,
//     unit_sphere_velocity: cgmath::Vector3<f32>,
//     tp1: cgmath::Vector3<f32>,
//     tp2: cgmath::Vector3<f32>,
//     tp3: cgmath::Vector3<f32>,
// ) -> (f32, f32, cgmath::Vector3<f32>, IntersectionStatus) {
//     let (t0, t1, plane_intersection_point, status) =
//         intersect_plane(unit_sphere_start, unit_sphere_velocity, tp1, tp2, tp3);
//     if status == IntersectionStatus::Crosses {
//         // plane_intersection_point =
//     } else if status == IntersectionStatus::NeverCrossesEmbedded {
//         // TODO: check if sphere is intersecting triangle or just plane
//         return (t0, t1, cgmath::Vector3::new(0.0, 0.0, 0.0), status);
//     } else {
//         return (t0, t1, cgmath::Vector3::new(0.0, 0.0, 0.0), status);
//     }
// }

/*
for each triangle:
    t0, t1, intersection_point, status = intersect_triangle(sphere, triangle)
    if t0 < 0 and t1 > 0 or status == NeverCrossesEmbedded:
        # need to move back out of this triangle
    else if t0 > 1:
        # need to collide and slide along triangle
*/

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_eq_eps_f32(a: f32, b: f32, eps: f32) {
        assert!((a - b).abs() < eps);
    }

    fn assert_vector_eq_eps_f32(v1: cgmath::Vector3<f32>, v2: cgmath::Vector3<f32>, eps: f32) {
        assert_eq_eps_f32(v1.x, v2.x, eps);
        assert_eq_eps_f32(v1.y, v2.y, eps);
        assert_eq_eps_f32(v1.z, v2.z, eps);
    }

    #[test]
    fn intersect_plane_big_triangle() {
        let (t0, t1, plane_intersection_point, status) = intersect_plane(
            cgmath::Vector3::new(0.0, 0.0, 2.0),
            cgmath::Vector3::new(4.0, 0.0, -4.0),
            cgmath::Vector3::new(-5.0, 0.0, 0.0),
            cgmath::Vector3::new(7.0, -5.0, 0.0),
            cgmath::Vector3::new(7.0, 5.0, 0.0),
        );

        assert_eq_eps_f32(t0, 0.25, 0.00001);
        assert_eq_eps_f32(t1, 0.75, 0.00001);
        assert_eq!(status, IntersectionStatus::Crosses);
    }

    #[test]
    fn intersect_plane_small_triangle() {
        let (t0, t1, plane_intersection_point, status) = intersect_plane(
            cgmath::Vector3::new(0.0, 0.0, 2.0),
            cgmath::Vector3::new(4.0, 0.0, -4.0),
            cgmath::Vector3::new(-1.0, 0.0, 0.0),
            cgmath::Vector3::new(2.0, -1.0, 0.0),
            cgmath::Vector3::new(2.0, 1.0, 0.0),
        );

        assert_eq_eps_f32(t0, 0.25, 0.00001);
        assert_eq_eps_f32(t1, 0.75, 0.00001);
        assert_eq!(status, IntersectionStatus::Crosses);
    }

    #[test]
    fn intersect_plane_after_move() {
        let (t0, t1, plane_intersection_point, status) = intersect_plane(
            cgmath::Vector3::new(0.0, 0.0, 2.0),
            cgmath::Vector3::new(0.5, 0.0, -0.5),
            cgmath::Vector3::new(-5.0, 0.0, 0.0),
            cgmath::Vector3::new(7.0, -5.0, 0.0),
            cgmath::Vector3::new(7.0, 5.0, 0.0),
        );

        assert_eq_eps_f32(t0, 2.0, 0.00001);
        assert_eq_eps_f32(t1, 6.0, 0.00001);
        assert_eq!(status, IntersectionStatus::Crosses);
    }

    #[test]
    fn intersect_plane_before_move() {
        let (t0, t1, plane_intersection_point, status) = intersect_plane(
            cgmath::Vector3::new(0.0, 0.0, 2.0),
            cgmath::Vector3::new(4.0, 0.0, 4.0),
            cgmath::Vector3::new(-5.0, 0.0, 0.0),
            cgmath::Vector3::new(7.0, -5.0, 0.0),
            cgmath::Vector3::new(7.0, 5.0, 0.0),
        );

        assert_eq_eps_f32(t0, -0.25, 0.00001);
        assert_eq_eps_f32(t1, -0.75, 0.00001);
        assert_eq!(status, IntersectionStatus::Crosses);
    }

    #[test]
    fn intersect_plane_starts_intersected() {
        let (t0, t1, plane_intersection_point, status) = intersect_plane(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(4.0, 0.0, -4.0),
            cgmath::Vector3::new(-5.0, 0.0, 0.0),
            cgmath::Vector3::new(7.0, -5.0, 0.0),
            cgmath::Vector3::new(7.0, 5.0, 0.0),
        );

        assert_eq_eps_f32(t0, -0.25, 0.00001);
        assert_eq_eps_f32(t1, 0.25, 0.00001);
        assert_eq!(status, IntersectionStatus::Crosses);
    }

    #[test]
    fn intersect_plane_parallel_starts_intersected() {
        let (t0, t1, plane_intersection_point, status) = intersect_plane(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(1.0, 0.0, 0.0),
            cgmath::Vector3::new(-5.0, 0.0, 0.0),
            cgmath::Vector3::new(7.0, -5.0, 0.0),
            cgmath::Vector3::new(7.0, 5.0, 0.0),
        );

        assert_eq_eps_f32(t0, 0.0, 0.00001);
        assert_eq_eps_f32(t1, 0.0, 0.00001);
        assert_vector_eq_eps_f32(
            plane_intersection_point,
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            0.00001,
        );
        assert_eq!(status, IntersectionStatus::NeverCrossesEmbedded);
    }

    #[test]
    fn intersect_plane_parallel_never_intersected() {
        let (t0, t1, plane_intersection_point, status) = intersect_plane(
            cgmath::Vector3::new(0.0, 0.0, 2.0),
            cgmath::Vector3::new(1.0, 0.0, 0.0),
            cgmath::Vector3::new(-5.0, 0.0, 0.0),
            cgmath::Vector3::new(7.0, -5.0, 0.0),
            cgmath::Vector3::new(7.0, 5.0, 0.0),
        );

        assert_eq_eps_f32(t0, 0.0, 0.00001);
        assert_eq_eps_f32(t1, 0.0, 0.00001);
        assert_vector_eq_eps_f32(
            plane_intersection_point,
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            0.00001,
        );
        assert_eq!(status, IntersectionStatus::NeverCrosses);
    }

    #[test]
    fn point_in_triangle_true() {
        println!("point_in_triangle_true");
        assert!(point_in_triangle(
            cgmath::Vector3::new(0.5, 0.5, 0.0),
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(2.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 2.0, 0.0),
        ));
    }

    #[test]
    fn point_in_triangle_false() {
        assert_eq!(
            false,
            point_in_triangle(
                cgmath::Vector3::new(-1.0, 0.5, 0.0),
                cgmath::Vector3::new(0.0, 0.0, 0.0),
                cgmath::Vector3::new(2.0, 0.0, 0.0),
                cgmath::Vector3::new(0.0, 2.0, 0.0),
            )
        );
    }

    #[test]
    fn point_in_triangle_on_edge() {
        assert_eq!(
            false,
            point_in_triangle(
                cgmath::Vector3::new(0.0, 1.0, 0.0),
                cgmath::Vector3::new(0.0, 0.0, 0.0),
                cgmath::Vector3::new(2.0, 0.0, 0.0),
                cgmath::Vector3::new(0.0, 2.0, 0.0),
            )
        );
    }

    #[test]
    fn point_in_triangle_on_vertex() {
        assert_eq!(
            false,
            point_in_triangle(
                cgmath::Vector3::new(0.0, 0.0, 0.0),
                cgmath::Vector3::new(0.0, 0.0, 0.0),
                cgmath::Vector3::new(2.0, 0.0, 0.0),
                cgmath::Vector3::new(0.0, 2.0, 0.0),
            )
        );
    }

    #[test]
    fn collide_point_happy_path() {
        let (t, collides) = collide_point(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 3.0, 0.0),
            cgmath::Vector3::new(0.0, 2.5, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 0.5, 0.00001);
    }

    #[test]
    fn collide_point_start() {
        let (t, collides) = collide_point(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 3.0, 0.0),
            cgmath::Vector3::new(0.0, 1.0, 0.0),
        );
        println!("{}, {}", t, collides);
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 0.0, 0.00001);
    }

    #[test]
    fn collide_point_end() {
        let (t, collides) = collide_point(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 3.0, 0.0),
            cgmath::Vector3::new(0.0, 4.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 1.0, 0.00001);
    }

    #[test]
    fn collide_point_no_collision() {}

    #[test]
    fn collide_point_start_inside() {}

    #[test]
    fn collide_point_start_past() {}
}
