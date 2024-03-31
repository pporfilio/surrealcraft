use std::collections::VecDeque;

use crate::game::camera::Camera;
use crate::game::input_state::{InputEvent, KeyButtonEvent};
use crate::game::state::SceneState;
use crate::geometry::buffers::{
    geometry_buffers_from_mesh, GeometryBuffers, Instance, RenderEntity,
};
use crate::geometry::geometry::{move_sphere_with_collision, TriangleMesh};
use crate::geometry::obj::read_obj;
use crate::levels::scene::{default_camera, default_motion, Scene};
use cgmath::Rotation3;
use winit::event::VirtualKeyCode;

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

pub struct CollisionTestScene {
    pub space_pressed: bool,
}

impl CollisionTestScene {
    pub fn new() -> Self {
        Self {
            space_pressed: false,
        }
    }
}

impl Scene for CollisionTestScene {
    fn initialize_camera(&mut self, config: &wgpu::SurfaceConfiguration) -> Camera {
        return default_camera(config);
    }

    fn initialize_geometry(
        &mut self,
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

    fn update_game_state(
        &mut self,
        event_queue: &VecDeque<InputEvent>,
        state: &SceneState,
        entities_fixme: &mut Vec<RenderEntity>,
        delta_s: f32,
    ) -> SceneState {
        // Track if the space button was already pressed to find only transitions
        // from unpressed to pressed.
        // Move and collide each time space is pressed
        for event in event_queue {
            match event {
                InputEvent::KeyButtonEvent(KeyButtonEvent {
                    logical_button: VirtualKeyCode::Space,
                    is_pressed: false,
                    ..
                }) => {
                    if self.space_pressed {
                        self.space_pressed = false;
                    }
                }
                InputEvent::KeyButtonEvent(KeyButtonEvent {
                    logical_button: VirtualKeyCode::Space,
                    is_pressed: true,
                    ..
                }) => {
                    if !self.space_pressed {
                        self.space_pressed = true;
                        let (new_location, attempts, finished_move) = move_sphere_with_collision(
                            entities_fixme[0].instances[0].position,
                            // cgmath::Vector3::new(0.1, 0.0, -0.1),
                            cgmath::Vector3::new(0.1, 0.0, 0.0),
                            &entities_fixme[1].mesh,
                        );
                        println!("{:?}, {:?}, {:?}", new_location, attempts, finished_move);
                        entities_fixme[0].instances[0].position = new_location;
                    }
                }
                _ => {}
            }
        }

        // Now that we've updated the geometry, do the usual camera update
        default_motion(event_queue, state, entities_fixme, delta_s)
    }
}
