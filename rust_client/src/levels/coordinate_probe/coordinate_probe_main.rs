use crate::game::camera::Camera;
use crate::game::input_state::InputEvent;
use crate::game::state::SceneState;
use crate::geometry::buffers::{
    geometry_buffers_from_mesh, GeometryBuffers, Instance, RenderEntity,
};
use crate::geometry::obj::read_obj;
use crate::levels::scene::{default_camera, default_motion, Scene};
use std::collections::VecDeque;

pub struct CoordinateProbeScene {}

impl CoordinateProbeScene {
    pub fn new() -> Self {
        Self {}
    }
}

impl Scene for CoordinateProbeScene {
    fn initialize_camera(&mut self, config: &wgpu::SurfaceConfiguration) -> Camera {
        return default_camera(config);
    }

    fn initialize_geometry(
        &mut self,
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
