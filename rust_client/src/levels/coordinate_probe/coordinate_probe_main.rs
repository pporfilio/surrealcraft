use super::super::super::geometry::buffers::{
    geometry_buffers_from_mesh, GeometryBuffers, Instance, RenderEntity,
};
use super::super::super::geometry::obj::read_obj;

pub fn initialize_coordinate_probe(
    device: &wgpu::Device,
) -> (Vec<RenderEntity>, Vec<GeometryBuffers>) {
    let tm = read_obj(
        "C:\\source\\surrealcraft\\terrain_generation\\coordinate_probe\\coordinate_probe.obj",
        cgmath::Vector3::new(0.2, 0.3, 0.4),
    )
    .unwrap();

    let mut tm_instances: Vec<Instance> = Vec::new();
    tm_instances.push(Instance::new());

    let mut entities: Vec<RenderEntity> = Vec::new();
    let mut geometry_buffers: Vec<GeometryBuffers> = Vec::new();
    geometry_buffers.push(geometry_buffers_from_mesh(device, &tm, &tm_instances));
    entities.push(RenderEntity {
        mesh: tm,
        instances: tm_instances,
    });

    (entities, geometry_buffers)
}
