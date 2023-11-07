// TODO: ugh input_state should probalby not be in main_window
// maybe we should be in like interface_layer or something at the
// same level as game, geometry, and main_window
use super::super::main_window::buffers::RenderEntity;
use super::super::main_window::input_state::{InputEvent, InputState};
use super::camera::Camera;
use std::collections::VecDeque;

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
pub struct SceneState {
    pub camera: Camera,
    pub input_state: InputState,
}

pub fn update_game_state(
    event_queue: &VecDeque<InputEvent>,
    state: &SceneState,
    entities_fixme: &Vec<RenderEntity>,
    delta_s: f32,
) {
    // state should be the output from the last call to this `update` function

    // TODO

    // Initialize this update's mouse accumulator with the state of the previous frame
    let mut mouse_accumulator = state.input_state.mouse_position.clone();
    for e in event_queue {
        // If this mouse move went to the center of the window, that was us resetting
        // the cursor position. Since we reset to (0, 0) after every move, the player
        // should never be able to move to that location/moving to that location is a
        // 0-distance move.
        match e {
            InputEvent::MouseMoveEvent(mouse_move_event) => {
                if mouse_move_event.position_x == 0.0 && mouse_move_event.position_y == 0.0 {
                    continue;
                } else {
                    mouse_accumulator.apply_mouse_move(mouse_move_event);
                }
            }
            _ => {}
        }
    }
    let (delta_x, delta_y, duration) =
        mouse_accumulator.difference_from(&state.input_state.mouse_position);

    // Process events to get mouse diff from last frame and current button state

    // use this updated input state to update the camera state
    //state.input_state.mouse_position = mouse_accumulator;

    // return new_state;
}

// Next things:
// move geometry initialization functions out of main_window.rs
// look into adding normals and basic lighting, maybe even w/o a matcap
// could add tests that the camera behaves correctly based on input state
//   and make the constants properties of the camera to make tests consistent
// log event stream
// could test event processing

// pub fn update_game_state(
//     input_state: &mut InputState,
//     camera: &mut Camera,
//     entities: &mut Vec<RenderEntity>,
//     delta_s: f32,
// ) {
//     let movement_scale: f32 = 10.0;
//     let rotation_scale: f32 = 0.2;
//     camera.add_position_delta(
//         movement_scale * get_camera_position_delta(camera, input_state, delta_s),
//     );
//     camera.add_pitch_deg(rotation_scale * get_camera_pitch_deg_delta(input_state));
//     camera.add_yaw_deg(rotation_scale * get_camera_yaw_deg_delta(input_state));
//     input_state.clear_mouse_delta();

//     if input_state.key_pressed(&VirtualKeyCode::R) {
//         camera.set_position(cgmath::Vector3::new(0.0, 0.0, 0.0));
//         camera.set_pitch_deg(0.0);
//         camera.set_yaw_deg(0.0);
//     }

//     if input_state.consume_key_pressed(&VirtualKeyCode::Space) {
//         let (new_location, attempts, finished_move) = move_sphere_with_collision(
//             entities[0].instances[0].position,
//             // cgmath::Vector3::new(0.1, 0.0, -0.1),
//             cgmath::Vector3::new(0.1, 0.0, 0.0),
//             &entities[1].mesh,
//         );
//         println!("{:?}, {:?}, {:?}", new_location, attempts, finished_move);
//         entities[0].instances[0].position = new_location;
//     }
// }
