use crate::game::camera::Camera;
use crate::game::input_state::{InputEvent, InputState, KeyButtonState};
use crate::game::state::SceneState;
use crate::geometry::buffers::{GeometryBuffers, RenderEntity};
use cgmath::InnerSpace;
use std::collections::VecDeque;
use winit::event::VirtualKeyCode;

// For update_game_state:
// Logically, I'd like to receive the previous frame's state, make a copy,
// edit the copy with any changes since the last frame, return the copy,
// render the copy, pass the copy into update_game_state for the next frame.
// In practice, I don't want to copy the data in static assets such as meshes
// because those will be big enough to be slow.
// But, some meshes may change frame to frame, e.g. stuff that is procedurally
// generated over time or edited by the player.

// Okay, if meshes were all static, then we have 3 things to connect:
// - TriangleMesh that loads/defines/collides the mesh
// - RenderEntity (to be renamed) that defines mesh "metadata", which right now
//   is just the list of instances, which are just position and rotation info
// - GeometryBuffers that are the buffers we send to the GPU, which include
//   vertex/color/normal as well as instance data
// Then we just have a map in main_window from id -> (TriangleMesh, GeometryBuffers)
// and in SceneState we have a list of whatever the game cares about that can reference
// the mesh/buffer pairs by id. In fact, multiple game entities can use the same mesh,
// or we can have one game entity that has a bunch of instances of the same mesh

// But probably I should get the game running again with the new input refactor before
// refactoring the entities.
// Probably for now I will copy the Camera and InputState but will pass entities
// in separately and mutate them in place.

// I can move them back into SceneState once I move the meshes to be referenced by id.

// Still not sure how to handle meshes that need to change frame to frame, but I can
// cross that bridge. Maybe they just aren't nicely tracked, or maybe I come up with a
// way to diff them, or maybe I track the inputs to whatever function changes them
// rather than tracking the changes directly...

pub trait Scene {
    // Set up camera
    fn initialize_camera(&mut self, config: &wgpu::SurfaceConfiguration) -> Camera;

    // Set up geometry
    // TODO: WASM
    // can't load files from disk in a web browser. They describe a webserver approach
    // here: https://sotrh.github.io/learn-wgpu/beginner/tutorial9-models/#accessing-files-from-wasm
    fn initialize_geometry(
        &mut self,
        device: &wgpu::Device,
    ) -> (Vec<RenderEntity>, Vec<GeometryBuffers>);

    // Return a new scene state with updates from this frame
    fn update_game_state(
        &mut self,
        event_queue: &VecDeque<InputEvent>,
        state: &SceneState,
        entities_fixme: &mut Vec<RenderEntity>,
        delta_s: f32,
    ) -> SceneState;
}

pub fn default_camera(config: &wgpu::SurfaceConfiguration) -> Camera {
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

pub fn default_motion(
    event_queue: &VecDeque<InputEvent>,
    state: &SceneState,
    entities_fixme: &Vec<RenderEntity>,
    delta_s: f32,
) -> SceneState {
    let mut new_input_state = state.input_state.clone();

    // Process events to get mouse diff from last frame and current button state
    for e in event_queue {
        new_input_state.apply_event(e);
    }

    // For now I just want the total mouse movement during the frame, so compare
    // end position to starting position
    let (delta_x, delta_y, duration) = new_input_state
        .mouse_position
        .difference_from(&state.input_state.mouse_position);

    // use the difference to update the camera state
    let rotation_scale: f32 = 0.2;
    let mut new_camera = state.camera.clone();

    new_camera.add_pitch_deg(-1.0 * rotation_scale * delta_y as f32);
    new_camera.add_yaw_deg(-1.0 * rotation_scale * delta_x as f32);

    //     input_state.clear_mouse_delta();

    match new_input_state.key_buttons.get(&VirtualKeyCode::R) {
        Some(KeyButtonState {
            is_pressed: true, .. // .. means the other fields don't matter
        }) => {
            new_camera.set_position(cgmath::Vector3::new(0.0, 0.0, 0.0));
            new_camera.set_pitch_deg(0.0);
            new_camera.set_yaw_deg(0.0);
        }
        _ => {}
    }

    new_camera.add_position_delta(default_get_camera_position_delta(
        &new_camera,
        &new_input_state,
        delta_s,
    ));

    // Return a new scene state with updates from this frame
    SceneState {
        camera: new_camera,
        input_state: new_input_state,
    }
}

pub fn default_get_camera_position_delta(
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
