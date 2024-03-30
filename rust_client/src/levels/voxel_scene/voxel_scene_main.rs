use crate::geometry::buffers::{
    geometry_buffers_from_mesh, GeometryBuffers, Instance, RenderEntity,
};
use crate::geometry::geometry::triangles_from_voxel_data;
use crate::geometry::voxels::voxel_data_from_file;

pub fn initialize_voxel_scene(device: &wgpu::Device) -> (Vec<RenderEntity>, Vec<GeometryBuffers>) {
    let vd =
        voxel_data_from_file("C:\\source\\surrealcraft\\terrain_generation\\kaladesh_island.vd")
            .unwrap();

    // let vd = voxel_test_geometry();

    let mut entities: Vec<RenderEntity> = Vec::new();
    let mut geometry_buffers: Vec<GeometryBuffers> = Vec::new();

    let voxel_mesh = triangles_from_voxel_data(&vd);

    let mut voxel_instances: Vec<Instance> = Vec::new();
    voxel_instances.push(Instance::new());

    geometry_buffers.push(geometry_buffers_from_mesh(
        device,
        &voxel_mesh,
        &voxel_instances,
    ));
    entities.push(RenderEntity {
        mesh: voxel_mesh,
        instances: voxel_instances,
    });

    (entities, geometry_buffers)
}
