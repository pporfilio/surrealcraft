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

pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = WGPUState::new(&window).await;

    let geometry = initialize_geometry(&state.device);

    let camera = initialize_camera(&state.config);

    event_loop.run(move |event, _, control_flow| { 
        match event {
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                state.update(camera.build_view_projection_matrix());
                match state.render(&geometry) {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
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
            } if window_id == window.id() => if !state.input(event) {
                match event {
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    // I have no idea what this syntax means...
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        // new_inner_size is &&mut so we have to dereference it twice
                        state.resize(**new_inner_size);
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