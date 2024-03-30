use crate::geometry::buffers::{
    geometry_buffers_from_mesh, GeometryBuffers, Instance, RenderEntity,
};
use crate::geometry::geometry::TriangleMesh;
use crate::geometry::obj::read_obj;
use cgmath::Rotation3;

pub fn collision_mesh_1() -> TriangleMesh {
    let color = cgmath::Vector3::new(115.0 / 255.0, 147.0 / 255.0, 179.0 / 255.0);
    let mut tm = TriangleMesh::new(200, 200);

    // Square at x = 5 facing toward origin, with a slight tilt
    tm.vertices.push(cgmath::Vector3::new(5.5, 2.0, 1.8));
    tm.vertices.push(cgmath::Vector3::new(5.5, 2.0, -1.8));
    tm.vertices.push(cgmath::Vector3::new(4.5, -2.0, 2.0));
    tm.vertices.push(cgmath::Vector3::new(4.5, -2.0, -2.0));

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

pub fn initialize_collision_test(
    device: &wgpu::Device,
) -> (Vec<RenderEntity>, Vec<GeometryBuffers>) {
    //let collision_mesh = collision_mesh_2();
    let collision_mesh = collision_mesh_1();
    let mut collision_mesh_instances: Vec<Instance> = Vec::new();
    collision_mesh_instances.push(Instance::new());

    // let collision_mesh = collision_mesh_2();

    let unit_sphere_mesh = read_obj(
        "resources/unit_sphere.obj",
        cgmath::Vector3::new(0.2, 0.3, 0.4),
    )
    .unwrap();

    let mut unit_sphere_instances: Vec<Instance> = Vec::new();
    unit_sphere_instances.push(Instance {
        // position: cgmath::Vector3::new(0.0, 0.0, 2.0),
        position: cgmath::Vector3::new(0.0, 0.0, 0.0),
        rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)),
    });
    // for x in 0..10 {
    //     for z in 0..10 {
    //         unit_sphere_instances.push(Instance {
    //             position: cgmath::Vector3::new((x * 2) as f32, 0.0, (z * 2) as f32),
    //             rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)),
    //         })
    //     }
    // }

    let mut entities: Vec<RenderEntity> = Vec::new();
    let mut geometry_buffers: Vec<GeometryBuffers> = Vec::new();
    geometry_buffers.push(geometry_buffers_from_mesh(
        device,
        &unit_sphere_mesh,
        &unit_sphere_instances,
    ));
    entities.push(RenderEntity {
        mesh: unit_sphere_mesh,
        instances: unit_sphere_instances,
    });

    geometry_buffers.push(geometry_buffers_from_mesh(
        device,
        &collision_mesh,
        &collision_mesh_instances,
    ));
    entities.push(RenderEntity {
        mesh: collision_mesh,
        instances: collision_mesh_instances,
    });

    (entities, geometry_buffers)
}
