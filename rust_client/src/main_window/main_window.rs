use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use super::wgpu_state::WGPUState;
use super::buffers::VERTICES;
use super::buffers::INDICES;
use super::buffers::GeometryBuffers;
use wgpu::util::DeviceExt;
use std::time::Instant;

use super::super::game::camera::Camera;


pub fn initialize_geometry(device: &wgpu::Device) -> GeometryBuffers {
    let vertex_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            // create_buffer_init needs plain u8 array. Bytemuck is a casting
            // library and we added some traits to struct Vertex to make it work
            // with bytemuck
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
        }
    );

    let index_buffer = device.create_buffer_init(
        &wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(INDICES),
            usage:wgpu::BufferUsages::INDEX,
        }
    );

    GeometryBuffers {
        vertex_buffer,
        index_buffer,
        vertex_count: VERTICES.len() as u32,
        index_count: INDICES.len() as u32,
    }
}

pub fn initialize_camera(config: &wgpu::SurfaceConfiguration) -> Camera {
    Camera::new(
        // position the camera one unit up and 2 units back
        // +z is out of the screen
        (0.0, 1.0, 2.0).into(),
        // have it look at the origin
        (0.0, 0.0, 0.0).into(),
        // which way is "up"
        cgmath::Vector3::unit_y(),
        config.width as f32 / config.height as f32,
        45.0,
        0.1,
        100.0,
    )
}

pub fn handle_input(event: &WindowEvent) -> bool {
    // From sleeping in render() with wgpu::PresentMode::Fifo, it seems like key press
    // events get queued up and processed when control returns to the event loop.
    // It seems like we get at most 1 mouse event per frame with whatever location
    // the mouse is at when control returns to the event loop
    // It looks like key release events may be are reliable: switching windows
    // triggers a key release event, even if key is still pressed
    // Mouse release can be missed: alt-tab to other window the mouse release doesn't
    // get sent unless window is re-focused before the mouse is released.
    // We do get Focused(bool) event when window focus changes 
    println!("{:?}", event);
    false
}

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut wgpu_state = WGPUState::new(&window).await;

    let geometry = initialize_geometry(&wgpu_state.device);

    let camera = initialize_camera(&wgpu_state.config);

    let mut prev_loop_instant = Instant::now();

    event_loop.run(move |event, _, control_flow| { 
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                
                let current_loop_instant = Instant::now();
                println!("{:?}", current_loop_instant.checked_duration_since(prev_loop_instant));
                prev_loop_instant = current_loop_instant;

                // TODO: update camera aspect ratio in case wgpu_state.size changed
                wgpu_state.update(camera.build_view_projection_matrix());
                match wgpu_state.render(&geometry) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => wgpu_state.resize(wgpu_state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors(Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually request it
                window.request_redraw();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => if !handle_input(event) {
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
                    _ => {}
                }
            },
            _ => {}
        }
    });
}