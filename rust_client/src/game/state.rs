// TODO: ugh input_state should probalby not be in main_window
// maybe we should be in like interface_layer or something at the
// same level as game, geometry, and main_window
use super::super::main_window::buffers::RenderEntity;
use super::super::main_window::input_state::{InputEvent, InputState};
use super::camera::Camera;
use std::collections::VecDeque;

pub struct SceneState {
    pub camera: Camera,
    pub entities: Vec<RenderEntity>,
    pub input_state: InputState,
}

pub fn update_game_state(event_queue: &VecDeque<InputEvent>, state: &SceneState, delta_s: f32) {
    // state should be the output from the last call to this `update` function

    // TODO

    // Initialize this update's mouse accumulator with the state of the previous frame
    let mouse_accumulator = state.input_state.mouse_position;
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
        mouse_accumulator.difference_from(state.input_state.mouse_position);

    // Process events to get mouse diff from last frame and current button state

    // use this updated input state to update the camera state
    state.input_state.mouse_position = mouse_accumulator;

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
