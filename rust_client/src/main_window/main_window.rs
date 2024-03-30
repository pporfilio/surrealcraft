use super::super::game::state::{update_game_state, SceneState};
use super::super::geometry::buffers::{GeometryBuffers, RenderEntity};
use super::super::levels::collision_test::collision_test_main::initialize_collision_test;
use super::super::levels::coordinate_probe::coordinate_probe_main::initialize_coordinate_probe;
use super::super::levels::voxel_scene::voxel_scene_main::initialize_voxel_scene;
use super::input_state::{
    FocusEvent, InputEvent, InputState, KeyButtonEvent, MouseAccumulator, MouseButtonEvent,
    MouseMoveEvent,
};
use super::wgpu_state::WGPUState;
use cgmath::InnerSpace;
// use std::iter::Zip;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::Instant;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window,
    window::WindowBuilder,
};

use super::super::game::camera::Camera;

pub struct WindowMetadata {
    is_first_mouse_move: bool,
}

// Next things:
// look into adding normals and basic lighting, maybe even w/o a matcap
// could add tests that the camera behaves correctly based on input state
//   and make the constants properties of the camera to make tests consistent
// log event stream
// could test event processing

// Something like on each event, update InputState, then pass
// both the new event and InputState to each input state machine
// so the the SMs can check if they need to transition

// State machines probably live in individual levels
// Right now it's just handling input and real states would come in
// once there's game logic.
// Or if I can figure out how to handle the stuff that's currently in main_window:
// on mouse move
//    if current position is 0, 0
//        pass
//    else if previous position doesn't exist:
//        reset to 0, 0
//    else:
//        apply delta to camera
//        reset to 0, 0
// or capture could be set up as part of a state machine
// where a key press would transition between captured and uncaptured
//
// More elaborate state machines might track sequences of mouse moves
// or track whether the current move is a "click and drag" or
// in a certain region of the screen, etc.

pub fn initialize_geometry(device: &wgpu::Device) -> (Vec<RenderEntity>, Vec<GeometryBuffers>) {
    // TODO: WASM
    // can't load files from disk in a web browser. They describe a webserver approach
    // here: https://sotrh.github.io/learn-wgpu/beginner/tutorial9-models/#accessing-files-from-wasm

    // initialize_collision_test(device)

    initialize_voxel_scene(device)

    // initialize_coordinate_probe(device)
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
    window_metadata: &mut WindowMetadata,
    event_queue: &mut VecDeque<InputEvent>,
    mouse_captured: bool,
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
            if !mouse_captured {
                return;
            }

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

            // Only record mouse move events if the move was away from the origin
            // Since we reset to the origin after every move.
            let delta_x = position.x as f64 / scale as f64 - w as f64 / 2.0;
            let delta_y = position.y as f64 / scale as f64 - h as f64 / 2.0;

            // I had trouble getting this to work in update_game_state, so reset the
            // mouse to (0, 0) here and only enqueue the event if the event was not
            // the move back to (0, 0)

            if window_metadata.is_first_mouse_move {
                // Note that we've had a mouse move and ignore this first move
                // This keeps the view from jumping when we first mouse into the window
                window_metadata.is_first_mouse_move = false;
            } else if !(delta_x == 0.0 && delta_y == 0.0) {
                event_queue.push_back(InputEvent::MouseMoveEvent(MouseMoveEvent {
                    delta_x,
                    delta_y,
                    timestamp: Instant::now(),
                }));

                if let Err(err) = window.set_cursor_position(winit::dpi::PhysicalPosition::new(
                    w * scale / 2.0,
                    h * scale / 2.0,
                )) {
                    println!("Error centering cursor position: {:?}", err);
                }
            }
        }
        WindowEvent::Focused(focused) => {
            // TODO: should I reset is_first_mouse_move if the focus changes?
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
    let mut window_metadata = WindowMetadata {
        is_first_mouse_move: true, // will be set to false once we've had a mouse move
    };

    let mut wgpu_state = WGPUState::new(&window).await;

    let (mut entities, mut geometry_buffers) = initialize_geometry(&wgpu_state.device);

    let mut camera = initialize_camera(&wgpu_state.config);

    let mut event_queue: VecDeque<InputEvent> = VecDeque::new();

    let mut prev_loop_instant = Instant::now();

    let mut scene_state = SceneState {
        camera: camera,
        input_state: InputState {
            key_buttons: HashMap::new(),
            mouse_buttons: HashMap::new(),
            mouse_position: MouseAccumulator {
                mouse_position: None,
            },
        },
    };

    let mut mouse_captured = false;

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

                scene_state = update_game_state(&mut event_queue, &scene_state, &entities, delta_s);
                event_queue.clear();

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
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::C),
                                ..
                            },
                        ..
                    } => mouse_captured = !mouse_captured,
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
                        handle_input(
                            event,
                            &window,
                            &mut window_metadata,
                            &mut event_queue,
                            mouse_captured,
                        );
                    }
                }
            }
            _ => {}
        }
    });
}
