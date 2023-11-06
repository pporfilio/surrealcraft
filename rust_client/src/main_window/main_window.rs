use super::super::game::state::{update_game_state, SceneState};
use super::super::geometry::geometry::*;
use super::super::geometry::obj::*;
use super::super::geometry::voxels::*;
use super::buffers::{GeometryBuffers, Instance, InstanceRaw, RenderEntity};
use super::input_state::{
    FocusEvent, InputEvent, InputState, KeyButtonEvent, MouseAccumulator, MouseButtonEvent,
    MouseMoveEvent,
};
use super::wgpu_state::WGPUState;
use crate::game::camera::rad_to_deg;
use crate::geometry;
use cgmath::InnerSpace;
use cgmath::Rotation3;
// use std::iter::Zip;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Instant;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window,
    window::WindowBuilder,
};

use super::super::game::camera::Camera;

pub fn geometry_buffers_from_mesh(
    device: &wgpu::Device,
    mesh: &TriangleMesh,
    instances: &Vec<Instance>,
) -> GeometryBuffers {
    let mut vertex_data: Vec<f32> = Vec::new();
    for (position, color) in mesh.vertices.iter().zip(mesh.colors.iter()) {
        vertex_data.push(position.x);
        vertex_data.push(position.y);
        vertex_data.push(position.z);
        vertex_data.push(color.x);
        vertex_data.push(color.y);
        vertex_data.push(color.z);
    }

    let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        // create_buffer_init needs plain u8 array. Bytemuck is a casting
        // library and we added some traits to struct Vertex to make it work
        // with bytemuck
        contents: bytemuck::cast_slice(&vertex_data[..]),
        usage: wgpu::BufferUsages::VERTEX,
    });

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&mesh.indices[..]),
        usage: wgpu::BufferUsages::INDEX,
    });

    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    });

    // TODO
    // GeometryBuffers should probably be refactored into geometry that has
    // a mesh, instance info, and a buffers sub-struct or something and a function
    // to update the buffers from updated mesh/instances and a way to indicate
    // what needs to be re-sent to the GPU.
    GeometryBuffers {
        vertex_buffer,
        index_buffer,
        vertex_count: mesh.vertices.len() as u32,
        index_count: mesh.indices.len() as u32,
        instance_data,
        instance_buffer,
        instance_count: instances.len() as u32,
    }
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

pub fn initialize_geometry(device: &wgpu::Device) -> (Vec<RenderEntity>, Vec<GeometryBuffers>) {
    // TODO: WASM
    // can't load files from disk in a web browser. They describe a webserver approach
    // here: https://sotrh.github.io/learn-wgpu/beginner/tutorial9-models/#accessing-files-from-wasm

    // initialize_collision_test(device)

    // initialize_voxel_scene(device)

    initialize_coordinate_probe(device)
}

pub fn initialize_camera(config: &wgpu::SurfaceConfiguration) -> Camera {
    Camera::new(
        // position the camera one unit up and 2 units back
        // +z is out of the screen
        (0.0, 0.0, 0.0).into(),
        0.0,
        0.0,
        config.width as f32 / config.height as f32,
        45.0,
        0.1,
        10000.0,
    )
}

pub fn handle_input(
    event: &WindowEvent,
    window: &window::Window,
    event_queue: &mut VecDeque<InputEvent>,
) {
    // From sleeping in render() with wgpu::PresentMode::Fifo, it seems like key press
    // events get queued up and processed when control returns to the event loop.
    // It seems like we get at most 1 mouse event per frame with whatever location
    // the mouse is at when control returns to the event loop
    // It looks like key release events maybe are reliable: switching windows
    // triggers a key release event, even if key is still pressed
    // Mouse release can be missed: alt-tab to other window the mouse release doesn't
    // get sent unless window is re-focused before the mouse is released.
    // We do get Focused(bool) event when window focus changes

    // println!("Event: {:?}", event);
    match event {
        WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some(virtual_keycode),
                    state,
                    ..
                },
            ..
        } => {
            event_queue.push_back(InputEvent::KeyButtonEvent(KeyButtonEvent {
                logical_button: *virtual_keycode,
                is_pressed: *state == ElementState::Pressed,
                timestamp: Instant::now(),
            }));
        }
        WindowEvent::MouseInput { state, button, .. } => {
            event_queue.push_back(InputEvent::MouseButtonEvent(MouseButtonEvent {
                logical_button: *button,
                is_pressed: *state == ElementState::Pressed,
                timestamp: Instant::now(),
            }));
        }
        WindowEvent::CursorMoved { position, .. } => {
            // Winit docs say this should not be used for 3d camera control
            // but don't say what should be used instead.
            // https://docs.rs/winit/latest/winit/event/enum.WindowEvent.html#variant.CursorMoved
            let (w, h): (f32, f32) = window
                .inner_size()
                .to_logical::<f32>(window.scale_factor())
                .into();
            let scale = window.scale_factor() as f32;
            // let delta_x = position.x as f32 / scale - w / 2.0;
            // let delta_y = position.y as f32 / scale - h / 2.0;

            event_queue.push_back(InputEvent::MouseMoveEvent(MouseMoveEvent {
                position_x: position.x as f64 / scale as f64,
                position_y: position.y as f64 / scale as f64,
                timestamp: Instant::now(),
            }));

            if let Err(err) = window.set_cursor_position(winit::dpi::PhysicalPosition::new(
                w * scale / 2.0,
                h * scale / 2.0,
            )) {
                println!("Error centering cursor position: {:?}", err);
            }

            // // println!("w: {:?} h: {:?}", w, h);
            // if delta_x == 0.0 && delta_y == 0.0 {
            //     // println!("Cursor still centered");
            // } else {
            //     // println!("Delta Y: {:?}", delta_y);
            //     if !input_state.is_first_mouse_event() {
            //         input_state.add_mouse_delta(delta_x, delta_y);
            //     } else {
            //         input_state.set_is_first_mouse_event(false);
            //     }
            //     // println!("scale factor: {:?}", window.scale_factor());
            //     let new_x = w * scale / 2.0;
            //     let new_y = h * scale / 2.0;
            //     // println!("new width: {:?} new_height: {:?}", new_x, new_y);
            //     if let Err(err) =
            //         window.set_cursor_position(winit::dpi::PhysicalPosition::new(new_x, new_y))
            //     {
            //         println!("Error centering cursor position: {:?}", err);
            //     }
            // }
        }
        WindowEvent::Focused(focused) => {
            event_queue.push_back(InputEvent::FocusEvent(FocusEvent {
                focused: *focused,
                timestamp: Instant::now(),
            }))
        }
        _ => (),
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut wgpu_state = WGPUState::new(&window).await;

    let (mut entities, mut geometry_buffers) = initialize_geometry(&wgpu_state.device);

    let mut camera = initialize_camera(&wgpu_state.config);

    let mut event_queue: VecDeque<InputEvent> = VecDeque::new();

    let mut prev_loop_instant = Instant::now();

    let scene_state = SceneState {
        camera: camera,
        entities: entities,
        input_state: InputState {
            key_buttons: HashMap::new(),
            mouse_buttons: HashMap::new(),
            mouse_position: MouseAccumulator {
                mouse_position: None,
            },
        },
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // TODO: How long does this block the event loop? Ideally I think
                // drawing would happen in a differnt thread so that we don't block
                // input events. If we get input events frequently, the timestamp
                // when we receive the event is probably good enough (pure speculation,
                // untested) for any sub-frame calculations
                // See also https://github.com/rust-windowing/winit/issues/1194#issuecomment-890672399

                let current_loop_instant = Instant::now();
                let delta_s = current_loop_instant
                    .saturating_duration_since(prev_loop_instant)
                    .as_secs_f32();
                // println!("{:?}", delta_s);
                prev_loop_instant = current_loop_instant;

                update_game_state(&mut event_queue, &scene_state, delta_s);

                // println!(
                //     "Camera yaw: {:?} pitch: {:?} position: {:?} look: {:?}",
                //     rad_to_deg(camera.yaw_rad()),
                //     rad_to_deg(camera.pitch_rad()),
                //     camera.position(),
                //     camera.look_vector()
                // );
                // use super::super::game::camera::rad_to_deg;
                // println!(
                //     "Camera: pitch: {:?} yaw: {:?}",
                //     rad_to_deg(camera.pitch_rad()),
                //     rad_to_deg(camera.yaw_rad())
                // );

                // This is some garbage maintaining these separate lists, but I don't
                // want wgpu_state to know about TriangleMesh or Instance
                // I ran into reference lifetime issues when I tried to store
                // GeometryBuffers in the RenderEntity and then pull out a list of
                // references to pass to the wgpu_state functions.
                for entity_i in 0..entities.len() {
                    for instance_i in 0..entities[entity_i].instances.len() {
                        geometry_buffers[entity_i].instance_data[instance_i] =
                            entities[entity_i].instances[instance_i].to_raw();
                    }
                }

                // TODO: update camera aspect ratio in case wgpu_state.size changed
                wgpu_state.update(
                    scene_state.camera.build_view_projection_matrix(),
                    &geometry_buffers,
                );
                match wgpu_state.render(&geometry_buffers) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => wgpu_state.resize(wgpu_state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors(Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("Error: {:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        wgpu_state.resize(*physical_size);
                    }
                    // I have no idea what this syntax means...
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        wgpu_state.resize(**new_inner_size);
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    _ => {
                        handle_input(event, &window, &mut event_queue);
                    }
                }
            }
            _ => {}
        }
    });
}
