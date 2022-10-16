use super::voxels::{Voxel, VoxelData};

pub struct TriangleMesh {
    pub vertices: Vec<cgmath::Vector3<f32>>,
    pub indices: Vec<u32>,
    pub colors: Vec<cgmath::Vector3<f32>>,
}

impl TriangleMesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            colors: Vec::new(),
        }
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

pub fn triangles_from_voxel_data(voxel_data: &VoxelData<Voxel>) -> TriangleMesh {
    let mut tm = TriangleMesh::new();

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

    tm
}
