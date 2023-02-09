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
    let point_on_plane = tp1;

    // This signed_distance is similar to distance_to_plane and
    // https://mathworld.wolfram.com/Point-PlaneDistance.html
    // except that we don't take the absolute value of the numerator and the
    // denominator is 1 so it is omitted.
    let signed_distance = (unit_sphere_start - point_on_plane).dot(plane_normal);

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
///
/// Returns true if point is withing the triangle
/// Returns false if point is equal to one of the vertices, if point is on an edge, or
///     if point is entirely outside the triangle.

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
    // Currently no good way to tell if point started inside sphere: looks the same
    // as a point outside the sphere
    assert!((point - unit_sphere_start).dot(point - unit_sphere_start) >= 1.0);

    // If velocity is 0, a is 0, which leads to NaN.
    // TODO: If velocity is 0, it's easy to check if a sphere is currently touching
    // a point.
    assert!(unit_sphere_velocity.dot(unit_sphere_velocity) != 0.0);
    return collide_point_unchecked(unit_sphere_start, unit_sphere_velocity, point);
}

pub fn collide_point_unchecked(
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

/// Given a point in space and p1, p2 on a line, returns the closest point on that
/// line and where that closest point is relative to p1 and p2.
///
/// Returns (t, closest_point)
/// If the closest point is between p1 and p2, t will be between 0 and 1
pub fn closest_point_on_line(
    point: cgmath::Vector3<f32>,
    line_point_1: cgmath::Vector3<f32>,
    line_point_2: cgmath::Vector3<f32>,
) -> (f32, cgmath::Vector3<f32>) {
    // math.stackexchange.com/questions/1521128/given-a-line-and-a-point-in-3d-how-to-find-the-closest-point-on-the-line
    // mathworld.wolfram.com/Point-LineDistance3-Dimensional.html
    let t = -1.0 * (line_point_2 - line_point_1).dot(line_point_1 - point)
        / (line_point_2 - line_point_1).dot(line_point_2 - line_point_1);

    let closest = line_point_1 + t * (line_point_2 - line_point_1);
    return (t, closest);
}

/// Given a unit sphere starting location, a velcoity, and a line segment to collide
/// against (p2 - p1), determines the first time the outside of the sphere collides
/// with the line segment as the sphere moves forward along the velocity. Returning
/// 0 means the sphere starts touching the line segment and returning 1 means the
/// sphere ends touching the line segment. Returning > 1 means the sphere would
/// eventually touch the line segment if it kept moving in the same direction.
///
/// returns (t, p, bool) where t is how far along the velocity the sphere touches
/// the line segment, p is the point where it first touches, and bool is true
/// if it touches eventually or false otherwise.
///
/// Fails assertion if the sphere starts intersecting the line segment. Does not fail
/// assertion if sphere is just touching the line segment.
pub fn collide_edge(
    unit_sphere_start: cgmath::Vector3<f32>,
    unit_sphere_velocity: cgmath::Vector3<f32>,
    p1: cgmath::Vector3<f32>,
    p2: cgmath::Vector3<f32>,
) -> (f32, cgmath::Vector3<f32>, bool) {
    // This function doesn't give useful results if the line segment starts embedded in the
    // sphere: it seems to give the time and point where the line segment becomes
    // tangent to the sphere.
    let (t, point) = closest_point_on_line(unit_sphere_start, p1, p2);
    // If the closest point on the line is within the line segment and within 1 unit,
    // we're intersecting it
    if t > 0.0 && t < 1.0 {
        if (unit_sphere_start - point).dot(unit_sphere_start - point) < 1.0 {
            panic!(
                "Unit sphere start intersected line segment.\nstart: {:?}\np1: {:?}\np2 {:?}",
                unit_sphere_start, p1, p2
            );
        }
    }

    // If the closest point on the line is not within the line segment, then the
    // closest part of the line segment is one of its endpoints, so make sure both
    // endpoints are more than 1 unit away.
    if (unit_sphere_start - p1).dot(unit_sphere_start - p1) < 1.0
        || (unit_sphere_start - p2).dot(unit_sphere_start - p2) < 1.0
    {
        panic!(
            "Unit sphere start intersected line endpoint.\nstart: {:?}\np1: {:?}\np2 {:?}",
            unit_sphere_start, p1, p2
        );
    }

    let edge = p2 - p1;
    let base_to_vertex = p1 - unit_sphere_start;

    let a = edge.dot(edge) * -1.0 * unit_sphere_velocity.dot(unit_sphere_velocity)
        + edge.dot(unit_sphere_velocity).powf(2.0);
    let b = edge.dot(edge) * 2.0 * unit_sphere_velocity.dot(base_to_vertex)
        - 2.0 * edge.dot(unit_sphere_velocity) * edge.dot(base_to_vertex);
    let c = edge.dot(edge) * (1.0 - base_to_vertex.dot(base_to_vertex))
        + edge.dot(base_to_vertex).powf(2.0);

    let determinant = b * b - 4.0 * a * c;
    if determinant < 0.0 || a.abs() < 0.00001 {
        return (0.0, cgmath::Vector3::new(0.0, 0.0, 0.0), false);
    }

    let sqrt_d = determinant.sqrt();
    let r1 = (-1.0 * b - sqrt_d) / (2.0 * a);
    let r2 = (-1.0 * b + sqrt_d) / (2.0 * a);

    let r_min = r1.min(r2);
    let r_max = r1.max(r2);

    let mut t = -1.0;
    if r_min >= 0.0 {
        t = r_min;
    } else if r_max >= 0.0 {
        t = r_max;
    }

    if t < 0.0 {
        return (t, cgmath::Vector3::new(0.0, 0.0, 0.0), false);
    }

    // Now we know that the sphere will touch the line at time t but we
    // need to find if it's in the line segment and if so where.
    let f0 = (edge.dot(unit_sphere_velocity) * t - edge.dot(base_to_vertex)) / edge.dot(edge);

    if f0 >= 0.0 && f0 <= 1.0 {
        return (t, p1 + f0 * edge, true);
    } else {
        return (0.0, cgmath::Vector3::new(0.0, 0.0, 0.0), false);
    }
}

/// Checks if a unit sphere will intersect a triangle along its velocity.
/// Returns fraction along the velocity (t), intersection point, and a bool indicating
/// if there was a collision.
pub fn collide_triangle(
    unit_sphere_start: cgmath::Vector3<f32>,
    unit_sphere_velocity: cgmath::Vector3<f32>,
    tp1: cgmath::Vector3<f32>,
    tp2: cgmath::Vector3<f32>,
    tp3: cgmath::Vector3<f32>,
) -> (f32, cgmath::Vector3<f32>, bool) {
    let (t0, _, plane_intersection_point, status) =
        intersect_plane(unit_sphere_start, unit_sphere_velocity, tp1, tp2, tp3);
    if status == IntersectionStatus::Crosses && t0 >= 0.0 && t0 <= 1.0 {
        // If the first place we touch this plane is inside the triangle, then that's
        // the earliest collision and we don't have to check edges and vertices.
        if point_in_triangle(plane_intersection_point, tp1, tp2, tp3) {
            return (t0, plane_intersection_point, true);
        }

        // If we collide with the plane but the first place on the plane we touch is
        // not inside the triangle, then we have to check if we touch an edge or vertex
        // (which would occur after touching the plane outside the triangle)
        // Check each vertex and each edge and return the closest collision.
        // TODO: if we're checking edges, doesn't that include the vertices? Why
        // do we also have to check the vertices?
        let mut t_min = 2.0; // only counts as a collision if <= 1.0
        let mut collision_point = cgmath::Vector3::new(0.0, 0.0, 0.0);
        for point in [tp1, tp2, tp3] {
            let (t_point, collides) = collide_point(unit_sphere_start, unit_sphere_velocity, point);
            if collides && t_point < t_min {
                t_min = t_point;
                collision_point = point;
            }
        }
        for (edge_start, edge_end) in [(tp1, tp2), (tp2, tp3), (tp3, tp1)] {
            let (t_edge, edge_collision_point, collides) = collide_edge(
                unit_sphere_start,
                unit_sphere_velocity,
                edge_start,
                edge_end,
            );
            if collides && t_edge < t_min {
                t_min = t_edge;
                collision_point = edge_collision_point;
            }
        }

        if t_min <= 1.0 {
            return (t_min, collision_point, true);
        }

        return (0.0, cgmath::Vector3::new(0.0, 0.0, 0.0), false);
    } else {
        // TODO: could check if status is NeverCrossesEmbedded if it's actually
        // embedded in the triangle or just the plane.
        return (0.0, cgmath::Vector3::new(0.0, 0.0, 0.0), false);
    }
}

/// Finds the first triangle a unit sphere traveling along a velocity vector will
/// intersect. Returns the fraction along the velocity, intersection point, and
/// index of the first triangle it would collide with.
/// The index is the location in the indices array that points to the first vertex of
/// the collided triangle.
pub fn collide_mesh(
    unit_sphere_start: cgmath::Vector3<f32>,
    unit_sphere_velocity: cgmath::Vector3<f32>,
    mesh: &TriangleMesh,
) -> (f32, cgmath::Vector3<f32>, usize, bool) {
    let mut nearest_t = 2.0; // Greater than 1 doesn't collide in this time step.
    let mut collision_point = cgmath::Vector3::new(0.0, 0.0, 0.0);
    let mut collides = false;
    let mut triangle_start_index = 0;
    for i in 0..(mesh.indices.len() / 3) {
        let (current_t, current_intersection_point, current_collides) = collide_triangle(
            unit_sphere_start,
            unit_sphere_velocity,
            mesh.vertices[mesh.indices[i] as usize],
            mesh.vertices[mesh.indices[i + 1] as usize],
            mesh.vertices[mesh.indices[i + 2] as usize],
        );
        if (current_collides && current_t >= 0.0 && current_t <= 1.0 && current_t < nearest_t) {
            nearest_t = current_t;
            collision_point = current_intersection_point;
            collides = true;
            triangle_start_index = i;
        }
    }
    return (nearest_t, collision_point, triangle_start_index, collides);
}

pub fn distance_to_plane(
    point: cgmath::Vector3<f32>,
    plane_origin: cgmath::Vector3<f32>,
    plane_normal: cgmath::Vector3<f32>,
) -> f32 {
    // https://mathworld.wolfram.com/Point-PlaneDistance.html
    let w = point - plane_origin;
    return plane_normal.dot(w).abs() / plane_normal.dot(plane_normal);
}

/// Given a sphere starting position, velocity, and mesh to collide against, returns
/// the location of the sphere after it has finished colliding and sliding against the
/// mesh
/// Returns ending position, number of recursions, and true if the motion finished
/// (false if it stopped early, e.g. due to error or max recursion)
pub fn move_sphere_with_collision(
    unit_sphere_start: cgmath::Vector3<f32>,
    unit_sphere_velocity: cgmath::Vector3<f32>,
    mesh: &TriangleMesh,
) -> (cgmath::Vector3<f32>, u32, bool) {
    let mut attempts = 0;
    let mut current_start = unit_sphere_start;
    let mut current_velocity = unit_sphere_velocity;
    while attempts < 5 {
        // See where the remaining motion collides
        let (t, collision_point, triangle_start_index, collides) =
            collide_mesh(current_start, current_velocity, mesh);

        // If we didn't collide (or collided at the end of our movement)
        // we don't need to take any further action
        if !collides || t == 1.0 {
            return (current_start + current_velocity, attempts, true);
        }

        assert!(t >= 0.0 && t <= 1.0);

        // Otherwise, we have to figure out how to slide against the collided triangle
        // and see if the resulting slide has any collisions.
        // I'm backing off by a tiny amount so that we don't immediately collide with
        // the triangle again at time 0 when we start sliding. I made this up so I
        // don't know if it's a good idea or not.
        let new_position = current_start + (t - 0.00001) * current_velocity;

        // I'll make the plane at the actual collision point, and not backed off a bit,
        // so that the normal is as close to accurate as possible.
        let plane_origin = collision_point;
        let mut plane_normal = current_start + t * current_velocity - collision_point;
        plane_normal = plane_normal.normalize();

        // To find the new velocity vector, we want to find the projection of the point
        // at the end of the original velocity onto the plane. The vector from the
        // collision point to that projected point is our new velocity vector.
        //
        // We find the projected point by moving the end of the original velocity
        // (current_start + current_velocity) back along the plane's normal. The
        // amount we need to move it back is the distance from that point to the plane,
        // as determined by distance_to_plane().
        let distance =
            distance_to_plane(current_start + current_velocity, plane_origin, plane_normal);
        let current_destination = current_start + current_velocity;
        let destination_on_plane = current_destination - distance * plane_normal;
        // collision_point is the same as plane_origin
        let new_velocity = destination_on_plane - collision_point;

        attempts += 1;
        current_start = new_position;
        current_velocity = new_velocity;
    }
    // We get here if we maxed out recursions without using up all the original
    // velocity. Don't add the latest current_velocity, because we haven't checked if
    // we'd collide while moving along it.
    return (current_start, attempts, false);
}

#[cfg(test)]
mod tests {
    use super::*;

    static COLLIDE_EPS: f32 = 0.00001;

    // TODO: there are lots of degenerate cases like triangles that are lines or points,
    // edges that are points, sphere that isn't moving, and probably more.
    // Anything that leads to a being 0 in the quadratic equations is not good.

    // There are a couple places where I compare against 0.0 or check that something is
    // less than 0.00001. These probably both give weird results in certain situations,
    // like objects moving really fast or very large/small geometries.

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
        assert_eq_eps_f32(t1, 0.75, COLLIDE_EPS);

        // Triangle is the plane z = 0
        // sphere starts at x = 0, y = 0, z = 2
        // sphere has radius 1
        // sphere moves in +x, -z equal amounts
        // when it moves -1 z it touches the plane 1 unit away from its center, at z = 0
        // it's center is at x = 1, y = 0, z = 1 at that point,
        //  so it touches at x = 1, y = 0, z = 0
        assert_vector_eq_eps_f32(
            plane_intersection_point,
            cgmath::Vector3::new(1.0, 0.0, 0.0),
            COLLIDE_EPS,
        );
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

        assert_eq_eps_f32(t0, 0.25, COLLIDE_EPS);
        assert_eq_eps_f32(t1, 0.75, COLLIDE_EPS);
        assert_vector_eq_eps_f32(
            plane_intersection_point,
            cgmath::Vector3::new(1.0, 0.0, 0.0),
            COLLIDE_EPS,
        );
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

        assert_eq_eps_f32(t0, 2.0, COLLIDE_EPS);
        assert_eq_eps_f32(t1, 6.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(
            plane_intersection_point,
            cgmath::Vector3::new(1.0, 0.0, 0.0),
            COLLIDE_EPS,
        );
        assert_eq!(status, IntersectionStatus::Crosses);
    }

    #[test]
    fn intersect_plane_before_move() {
        let (t0, t1, plane_intersection_point, status) = intersect_plane(
            cgmath::Vector3::new(0.0, 0.0, 2.0),
            cgmath::Vector3::new(-4.0, 0.0, 4.0),
            cgmath::Vector3::new(-5.0, 0.0, 0.0),
            cgmath::Vector3::new(7.0, -5.0, 0.0),
            cgmath::Vector3::new(7.0, 5.0, 0.0),
        );

        assert_eq_eps_f32(t0, -0.25, COLLIDE_EPS);
        assert_eq_eps_f32(t1, -0.75, COLLIDE_EPS);
        assert_vector_eq_eps_f32(
            plane_intersection_point,
            cgmath::Vector3::new(1.0, 0.0, 0.0),
            COLLIDE_EPS,
        );
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

        assert_eq_eps_f32(t0, -0.25, COLLIDE_EPS);
        assert_eq_eps_f32(t1, 0.25, COLLIDE_EPS);
        assert_vector_eq_eps_f32(
            plane_intersection_point,
            cgmath::Vector3::new(-1.0, 0.0, 0.0),
            COLLIDE_EPS,
        );
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

        assert_eq_eps_f32(t0, 0.0, COLLIDE_EPS);
        assert_eq_eps_f32(t1, 0.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(
            plane_intersection_point,
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            COLLIDE_EPS,
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

        assert_eq_eps_f32(t0, 0.0, COLLIDE_EPS);
        assert_eq_eps_f32(t1, 0.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(
            plane_intersection_point,
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            COLLIDE_EPS,
        );
        assert_eq!(status, IntersectionStatus::NeverCrosses);
    }

    #[test]
    fn point_in_triangle_true() {
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
        assert_eq_eps_f32(t, 0.5, COLLIDE_EPS);
    }

    #[test]
    fn collide_point_start() {
        let (t, collides) = collide_point(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 3.0, 0.0),
            cgmath::Vector3::new(0.0, 1.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 0.0, COLLIDE_EPS);
    }

    #[test]
    fn collide_point_end() {
        let (t, collides) = collide_point(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 3.0, 0.0),
            cgmath::Vector3::new(0.0, 4.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 1.0, COLLIDE_EPS);
    }

    #[test]
    fn collide_point_future() {
        let (t, collides) = collide_point(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 3.0, 0.0),
            cgmath::Vector3::new(0.0, 7.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 2.0, COLLIDE_EPS);
    }

    #[test]
    fn collide_point_past() {
        let (t, collides) = collide_point(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(0.0, 3.0, 0.0),
            cgmath::Vector3::new(0.0, -2.0, 0.0),
        );
        assert_eq!(false, collides);
        assert_eq_eps_f32(t, -1.0, COLLIDE_EPS);
    }

    #[test]
    fn collide_point_no_collision() {
        let (t, collides) = collide_point(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(2.0, 0.0, 0.0),
            cgmath::Vector3::new(1.0, 10.0, 0.0),
        );
        assert_eq!(false, collides);
        assert_eq_eps_f32(t, 0.0, COLLIDE_EPS);
    }

    #[test]
    fn collide_point_start_inside() {
        let (t, collides) = collide_point_unchecked(
            cgmath::Vector3::new(0.0, 0.0, 0.0),
            cgmath::Vector3::new(2.0, 0.0, 0.0),
            cgmath::Vector3::new(-0.5, 0.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 0.25, COLLIDE_EPS);
    }

    #[test]
    fn closest_point_on_line_happy_path() {
        let (t, point) = closest_point_on_line(
            cgmath::Vector3::new(3.0, 1.0, 0.0),
            cgmath::Vector3::new(1.0, 1.0, 0.0),
            cgmath::Vector3::new(3.0, 3.0, 0.0),
        );
        assert_eq_eps_f32(t, 0.5, COLLIDE_EPS);
        assert_vector_eq_eps_f32(point, cgmath::Vector3::new(2.0, 2.0, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn closest_point_on_line_segment_endpoints() {
        let (start_t, start_point) = closest_point_on_line(
            cgmath::Vector3::new(2.0, 0.0, 0.0),
            cgmath::Vector3::new(1.0, 1.0, 0.0),
            cgmath::Vector3::new(3.0, 3.0, 0.0),
        );
        assert_eq_eps_f32(start_t, 0.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(
            start_point,
            cgmath::Vector3::new(1.0, 1.0, 0.0),
            COLLIDE_EPS,
        );

        let (end_t, end_point) = closest_point_on_line(
            cgmath::Vector3::new(4.0, 2.0, 0.0),
            cgmath::Vector3::new(1.0, 1.0, 0.0),
            cgmath::Vector3::new(3.0, 3.0, 0.0),
        );
        assert_eq_eps_f32(end_t, 1.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(end_point, cgmath::Vector3::new(3.0, 3.0, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn closest_point_on_line_outside_segment() {
        let (t, point) = closest_point_on_line(
            cgmath::Vector3::new(5.0, 3.0, 0.0),
            cgmath::Vector3::new(1.0, 1.0, 0.0),
            cgmath::Vector3::new(3.0, 3.0, 0.0),
        );
        assert_eq_eps_f32(t, 1.5, COLLIDE_EPS);
        assert_vector_eq_eps_f32(point, cgmath::Vector3::new(4.0, 4.0, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn closest_point_on_line_distance_zero() {
        let (t, point) = closest_point_on_line(
            cgmath::Vector3::new(1.5, 1.5, 0.0),
            cgmath::Vector3::new(1.0, 1.0, 0.0),
            cgmath::Vector3::new(3.0, 3.0, 0.0),
        );
        assert_eq_eps_f32(t, 0.25, COLLIDE_EPS);
        assert_vector_eq_eps_f32(point, cgmath::Vector3::new(1.5, 1.5, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn collide_edge_happy_path() {
        let (t, p, collides) = collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(5.0, 2.0, 0.0),
            cgmath::Vector3::new(5.0, 5.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 0.3333333, COLLIDE_EPS);
        assert_vector_eq_eps_f32(p, cgmath::Vector3::new(5.0, 4.0, 0.0), COLLIDE_EPS);

        // Check that this is invariant to the direction of the line segment
        let (t, p, collides) = collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(5.0, 5.0, 0.0),
            cgmath::Vector3::new(5.0, 2.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 0.3333333, COLLIDE_EPS);
        assert_vector_eq_eps_f32(p, cgmath::Vector3::new(5.0, 4.0, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn collide_edge_start() {
        let (t, p, collides) = collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(4.0, 2.0, 0.0),
            cgmath::Vector3::new(4.0, 5.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 0.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(p, cgmath::Vector3::new(4.0, 4.0, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn collide_edge_end() {
        let (t, p, collides) = collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(7.0, 2.0, 0.0),
            cgmath::Vector3::new(7.0, 5.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 1.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(p, cgmath::Vector3::new(7.0, 4.0, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn collide_edge_future() {
        let (t, p, collides) = collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(10.0, 2.0, 0.0),
            cgmath::Vector3::new(10.0, 5.0, 0.0),
        );
        assert_eq!(true, collides);
        assert_eq_eps_f32(t, 2.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(p, cgmath::Vector3::new(10.0, 4.0, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn collide_edge_past() {
        let (t, p, collides) = collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(-5.0, 2.0, 0.0),
            cgmath::Vector3::new(-5.0, 5.0, 0.0),
        );
        assert_eq!(false, collides);
        assert_eq_eps_f32(t, -1.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(p, cgmath::Vector3::new(0.0, 0.0, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn collide_edge_no_collision() {
        let (t, p, collides) = collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(1.0, 0.0, 0.0),
            cgmath::Vector3::new(5.0, 0.0, 0.0),
        );
        assert_eq!(false, collides);
        assert_eq_eps_f32(t, 0.0, COLLIDE_EPS);
        assert_vector_eq_eps_f32(p, cgmath::Vector3::new(0.0, 0.0, 0.0), COLLIDE_EPS);
    }

    #[test]
    fn collide_edge_angled_regression_test() {
        let (t, p, collides) = collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(-1.0, 0.0, 0.0),
            cgmath::Vector3::new(9.0, 5.0, 0.0),
        );
        assert_eq!(true, collides);
        println!("\n\nangled: {:?}\n\n", t); // 0.58797735
        println!("\n\nangled: {:?}\n\n", p); // [5.2111454, 3.1055727, 0.0]

        // Wasn't sure how to/didn't try to calculate exact expected values
        // so I just captured the output to use as a regression test.
        assert_eq_eps_f32(t, 0.58797735, COLLIDE_EPS);
        assert_vector_eq_eps_f32(
            p,
            cgmath::Vector3::new(5.2111454, 3.1055727, 0.0),
            COLLIDE_EPS,
        );
    }

    #[test]
    #[should_panic(expected = "Unit sphere start intersected line segment.")]
    fn collide_edge_start_inside() {
        collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(3.1, 2.0, 0.0),
            cgmath::Vector3::new(3.1, 6.0, 0.0),
        );
    }

    #[test]
    #[should_panic(expected = "Unit sphere start intersected line endpoint.")]
    fn collide_edge_endppoint_starts_inside() {
        collide_edge(
            cgmath::Vector3::new(3.0, 4.0, 0.0),
            cgmath::Vector3::new(3.0, 0.0, 0.0),
            cgmath::Vector3::new(3.1, 3.5, 0.0),
            cgmath::Vector3::new(3.1, 0.0, 0.0),
        );
    }
}
