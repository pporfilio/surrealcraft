use crate::game::camera::Camera;
use crate::game::input_state::InputEvent;
use crate::game::state::SceneState;
use crate::geometry::buffers::{
    geometry_buffers_from_mesh, GeometryBuffers, Instance, RenderEntity,
};
use crate::geometry::geometry::triangles_from_voxel_data;
use crate::geometry::voxels::{voxel_data_from_file, voxel_test_geometry};
use crate::levels::scene::{default_camera, default_motion, Scene};
use std::collections::VecDeque;

pub struct VoxelScene {}

impl VoxelScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for VoxelScene {
    fn initialize_camera(&mut self, config: &wgpu::SurfaceConfiguration) -> Camera {
        return default_camera(config);
    }

    fn initialize_geometry(
        &mut self,
        device: &wgpu::Device,
    ) -> (Vec<RenderEntity>, Vec<GeometryBuffers>) {
        let vd =
            voxel_data_from_file("C:\\source\\surrealcraft\\terrain_generation\\perlin_terrain.vd")
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

    fn update_game_state(
        &mut self,
        event_queue: &VecDeque<InputEvent>,
        state: &SceneState,
        entities_fixme: &mut Vec<RenderEntity>,
        delta_s: f32,
    ) -> SceneState {
        return default_motion(event_queue, state, entities_fixme, delta_s);
    }
}
