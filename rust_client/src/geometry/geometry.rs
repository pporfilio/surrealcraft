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

    let voxel_guess = (voxel_data.dimensions().x as usize
        * voxel_data.dimensions().y as usize
        * voxel_data.dimensions().z as usize);

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
