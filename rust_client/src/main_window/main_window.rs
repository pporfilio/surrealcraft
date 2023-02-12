use super::super::geometry::geometry::*;
use super::super::geometry::obj::*;
use super::super::geometry::voxels::*;
use super::buffers::GeometryBuffers;
use super::buffers::Instance;
use super::buffers::InstanceRaw;
use super::input_state::InputState;
use super::wgpu_state::WGPUState;
use crate::game::camera::rad_to_deg;
use cgmath::InnerSpace;
use cgmath::Rotation3;
// use std::iter::Zip;
use std::time::Instant;
use wgpu::util::DeviceExt;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window,
    window::WindowBuilder,
};

use super::super::game::camera::Camera;

pub fn initialize_geometry(device: &wgpu::Device) -> Vec<GeometryBuffers> {
    // TODO: WASM
    // can't load files from disk in a web browser. They describe a webserver approach
    // here: https://sotrh.github.io/learn-wgpu/beginner/tutorial9-models/#accessing-files-from-wasm

    // let vd =
    //     voxel_data_from_file("C:\\source\\surrealcraft\\terrain_generation\\kaladesh_island.vd")
    //         .unwrap();

    // let vd = voxel_test_geometry();

    // let tm = triangles_from_voxel_data(&vd);

    // let tm = read_obj(
    //     "C:\\Users\\parker\\Downloads\\coordinate_probe.obj",
    //     cgmath::Vector3::new(0.2, 0.3, 0.4),
    // )
    // .unwrap();

    // let tm = collision_mesh_1();

    let tm = read_obj(
        "resources/unit_sphere.obj",
        cgmath::Vector3::new(0.2, 0.3, 0.4),
    )
    .unwrap();

    // let tm = collision_mesh_2();

    let mut vertex_data: Vec<f32> = Vec::new();
    for (position, color) in tm.vertices.iter().zip(tm.colors) {
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
        contents: bytemuck::cast_slice(&tm.indices[..]),
        usage: wgpu::BufferUsages::INDEX,
    });

    let mut instances: Vec<Instance> = Vec::new();
    for x in 0..10 {
        for z in 0..10 {
            instances.push(Instance {
                position: cgmath::Vector3::new((x * 2) as f32, 0.0, (z * 2) as f32),
                rotation: cgmath::Quaternion::from_angle_x(cgmath::Deg(0.0)),
            })
        }
    }

    let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
    let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(&instance_data),
        usage: wgpu::BufferUsages::VERTEX,
    });

    vec![GeometryBuffers {
        vertex_buffer,
        index_buffer,
        vertex_count: tm.vertices.len() as u32,
        index_count: tm.indices.len() as u32,
        instance_buffer,
        instance_count: instances.len() as u32,
    }]
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

pub fn handle_input(event: &WindowEvent, window: &window::Window, input_state: &mut InputState) {
    // From sleeping in render() with wgpu::PresentMode::Fifo, it seems like key press
    // events get queued up and processed when control returns to the event loop.
    // It seems like we get at most 1 mouse event per frame with whatever location
    // the mouse is at when control returns to the event loop
    // It looks like key release events may be are reliable: switching windows
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
            // println!("Matched key code {:?}", virtual_keycode);
            match state {
                ElementState::Pressed => {
                    input_state.set_key_pressed(virtual_keycode);
                }
                ElementState::Released => {
                    input_state.set_key_released(virtual_keycode);
                }
            }
        }
        WindowEvent::MouseInput { state, button, .. } => {
            // println!("Matched MouseInput {:?} {:?}", button, state);
            match state {
                ElementState::Pressed => {
                    input_state.set_mouse_button_presssed(button);
                }
                ElementState::Released => {
                    input_state.set_mouse_button_released(button);
                }
            }
        }
        WindowEvent::CursorMoved { position, .. } => {
            let (w, h): (f32, f32) = window
                .inner_size()
                .to_logical::<f32>(window.scale_factor())
                .into();
            let scale = window.scale_factor() as f32;
            let delta_x = position.x as f32 / scale - w / 2.0;
            let delta_y = position.y as f32 / scale - h / 2.0;
            // println!("w: {:?} h: {:?}", w, h);
            if delta_x == 0.0 && delta_y == 0.0 {
                // println!("Cursor still centered");
            } else {
                // println!("Delta Y: {:?}", delta_y);
                input_state.add_mouse_delta(delta_x, delta_y);
                // println!("scale factor: {:?}", window.scale_factor());
                let new_x = w * scale / 2.0;
                let new_y = h * scale / 2.0;
                // println!("new width: {:?} new_height: {:?}", new_x, new_y);
                if let Err(err) =
                    window.set_cursor_position(winit::dpi::PhysicalPosition::new(new_x, new_y))
                {
                    println!("Error centering cursor position: {:?}", err);
                }
            }
        }
        _ => (),
    }
}

pub fn get_camera_position_delta(
    camera: &Camera,
    input_state: &InputState,
    delta_s: f32,
) -> cgmath::Vector3<f32> {
    let mut delta_forward: f32 = 0.0;
    let mut delta_up: f32 = 0.0;
    let mut delta_right: f32 = 0.0;
    if input_state.key_pressed(&VirtualKeyCode::W) || input_state.key_pressed(&VirtualKeyCode::Up) {
        delta_forward += delta_s;
    }
    if input_state.key_pressed(&VirtualKeyCode::S) || input_state.key_pressed(&VirtualKeyCode::Down)
    {
        delta_forward -= delta_s;
    }
    if input_state.key_pressed(&VirtualKeyCode::D)
        || input_state.key_pressed(&VirtualKeyCode::Right)
    {
        delta_right += delta_s;
    }
    if input_state.key_pressed(&VirtualKeyCode::A) || input_state.key_pressed(&VirtualKeyCode::Left)
    {
        delta_right -= delta_s;
    }
    if input_state.key_pressed(&VirtualKeyCode::Q) {
        delta_up += delta_s;
    }
    if input_state.key_pressed(&VirtualKeyCode::E) {
        delta_up -= delta_s;
    }

    delta_forward * camera.look_vector()
        + delta_up * camera.up_vector()
        + delta_right * camera.look_vector().cross(camera.up_vector()).normalize()
}

pub fn get_camera_yaw_deg_delta(input_state: &InputState) -> f32 {
    if input_state.mouse_position_set() {
        // Yaw increases as the mouse moves left, because our coordinate frame is
        // X foward, Y to the left.
        return -1.0 * input_state.mouse_delta().x;
    } else {
        return 0.0;
    }
}

pub fn get_camera_pitch_deg_delta(input_state: &InputState) -> f32 {
    if input_state.mouse_position_set() {
        return -1.0 * input_state.mouse_delta().y;
    } else {
        return 0.0;
    }
}

pub fn update_game_state(input_state: &mut InputState, camera: &mut Camera, delta_s: f32) {
    let movement_scale: f32 = 10.0;
    let rotation_scale: f32 = 0.2;
    camera.add_position_delta(
        movement_scale * get_camera_position_delta(camera, input_state, delta_s),
    );
    camera.add_pitch_deg(rotation_scale * get_camera_pitch_deg_delta(input_state));
    camera.add_yaw_deg(rotation_scale * get_camera_yaw_deg_delta(input_state));
    input_state.clear_mouse_delta();

    if input_state.key_pressed(&VirtualKeyCode::R) {
        camera.set_position(cgmath::Vector3::new(0.0, 0.0, 0.0));
        camera.set_pitch_deg(0.0);
        camera.set_yaw_deg(0.0);
    }
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut wgpu_state = WGPUState::new(&window).await;

    let geometry = initialize_geometry(&wgpu_state.device);

    let mut camera = initialize_camera(&wgpu_state.config);

    let mut input_state = InputState::new();

    let mut prev_loop_instant = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                let current_loop_instant = Instant::now();
                let delta_s = current_loop_instant
                    .saturating_duration_since(prev_loop_instant)
                    .as_secs_f32();
                // println!("{:?}", delta_s);
                prev_loop_instant = current_loop_instant;

                update_game_state(&mut input_state, &mut camera, delta_s);

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

                // TODO: update camera aspect ratio in case wgpu_state.size changed
                wgpu_state.update(camera.build_view_projection_matrix());
                match wgpu_state.render(&geometry) {
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
                        handle_input(event, &window, &mut input_state);
                    }
                }
            }
            _ => {}
        }
    });
}
