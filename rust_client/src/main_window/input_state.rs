use std::collections::HashMap;
use std::time::{Duration, Instant};
use winit::event::{KeyboardInput, MouseButton, VirtualKeyCode};

pub enum InputEvent {
    KeyButtonEvent(KeyButtonEvent),
    MouseButtonEvent(MouseButtonEvent),
    MouseMoveEvent(MouseMoveEvent),
    FocusEvent(FocusEvent),
}

pub struct KeyButtonEvent {
    pub logical_button: VirtualKeyCode,
    pub is_pressed: bool,
    pub timestamp: Instant,
}
pub struct MouseButtonEvent {
    pub logical_button: MouseButton,
    pub is_pressed: bool,
    pub timestamp: Instant,
}
pub struct MouseMoveEvent {
    pub delta_x: f64,
    pub delta_y: f64,
    pub timestamp: Instant,
}

pub struct FocusEvent {
    pub focused: bool,
    pub timestamp: Instant,
}

#[derive(Clone)]
pub struct KeyButtonState {
    pub logical_button: VirtualKeyCode,
    pub is_pressed: bool,
    pub last_transition: Option<Instant>,
}

#[derive(Clone)]
pub struct MouseButtonState {
    pub logical_button: MouseButton,
    pub is_pressed: bool,
    pub last_transition: Option<Instant>,
}

#[derive(Clone)]
pub struct MousePositionState {
    pub current_x: f64,
    pub current_y: f64,
    pub previous_x: Option<f64>,
    pub previous_y: Option<f64>,
    pub current_timestamp: Option<Instant>,
    pub previous_timestamp: Option<Instant>,
}

#[derive(Clone)]
pub struct InputState {
    pub key_buttons: HashMap<VirtualKeyCode, KeyButtonState>,
    pub mouse_buttons: HashMap<MouseButton, MouseButtonState>,
    pub mouse_position: MouseAccumulator,
}

#[derive(Clone)]
pub struct MouseAccumulator {
    pub mouse_position: Option<MousePositionState>,
}

impl MouseAccumulator {
    pub fn initialize_mouse_position(mut self, prior_state: MousePositionState) {
        self.mouse_position = Some(prior_state);
    }

    pub fn difference_from(&self, other: &MouseAccumulator) -> (f64, f64, Duration) {
        match (&self.mouse_position, &other.mouse_position) {
            (Some(s), Some(o)) => {
                let mut duration = Duration::ZERO;
                match (s.current_timestamp, o.current_timestamp) {
                    (Some(s_timestamp), Some(o_timestamp)) => {
                        duration = s_timestamp.saturating_duration_since(o_timestamp);
                    }
                    _ => {}
                }
                return (
                    s.current_x - o.current_x,
                    s.current_y - o.current_y,
                    duration,
                );
            }
            _ => {
                return (0.0, 0.0, Duration::ZERO);
            }
        }
    }

    pub fn apply_mouse_move(&mut self, event: &MouseMoveEvent) {
        match &mut self.mouse_position {
            Some(mouse) => {
                mouse.previous_x = Some(mouse.current_x);
                mouse.previous_y = Some(mouse.current_y);
                mouse.current_x += event.delta_x;
                mouse.current_y += event.delta_y;
                mouse.previous_timestamp = mouse.current_timestamp;
                mouse.current_timestamp = Some(event.timestamp);
            }
            None => {
                self.mouse_position = Some(MousePositionState {
                    current_x: event.delta_x,
                    current_y: event.delta_y,
                    previous_x: None,
                    previous_y: None,
                    current_timestamp: Some(event.timestamp),
                    previous_timestamp: None,
                })
            }
        }
    }
}

impl InputState {
    pub fn key_pressed(&self, key: &VirtualKeyCode) -> bool {
        match self.key_buttons.get(key) {
            Some(KeyButtonState {
                is_pressed: true, .. // .. means the other fields don't matter
            }) => {
                return true
            },
            _ => { return false }
        }
    }

    pub fn apply_event(&mut self, event: &InputEvent) {
        match event {
            InputEvent::KeyButtonEvent(KeyButtonEvent {
                logical_button,
                is_pressed,
                timestamp,
            }) => {
                // This says to look up a key in the map [entry()]
                // and if that key exists, modify it in-place [and_modify()]
                // or if the key doesn't exist, create a new entry [or_insert()]
                self.key_buttons
                    .entry(*logical_button)
                    .and_modify(|entry| {
                        if *is_pressed != entry.is_pressed {
                            entry.is_pressed = *is_pressed;
                            entry.last_transition = Some(*timestamp);
                        }
                    })
                    .or_insert(KeyButtonState {
                        logical_button: *logical_button,
                        is_pressed: *is_pressed,
                        last_transition: None,
                    });
            }
            InputEvent::MouseButtonEvent(MouseButtonEvent {
                logical_button,
                is_pressed,
                timestamp,
            }) => {
                // TODO: The only difference between this and the KeyButtonEvent is
                // the name of the map and the type of logical_button. There should
                // be a way to factor it out.
                self.mouse_buttons
                    .entry(*logical_button)
                    .and_modify(|entry| {
                        if *is_pressed != entry.is_pressed {
                            entry.is_pressed = *is_pressed;
                            entry.last_transition = Some(*timestamp);
                        }
                    })
                    .or_insert(MouseButtonState {
                        logical_button: *logical_button,
                        is_pressed: *is_pressed,
                        last_transition: None,
                    });
            }
            InputEvent::MouseMoveEvent(event) => self.mouse_position.apply_mouse_move(&event),
            InputEvent::FocusEvent(event) => {
                // TODO
            }
        }
    }
}

// Something like on each event, update InputState, then pass
// both the new event and InputState to each input state machine
// so the the SMs can check if they need to transition

// State machines probably live in `game/`
// Mouse "state machine" right now will be something like
// on mouse move
//    if current position is 0, 0
//        pass
//    else if previous position doesn't exist:
//        reset to 0, 0
//    else:
//        apply delta to camera
//        reset to 0, 0
// real states would come in if we wanted to have a capture/uncapture
// option, so there would be the uncaptured state where we don't
// move the camera and a key press would transition to the captured
// state which is described above.
//
// More elaborate state machines might track sequences of mouse moves
// or track whether the current move is a "click and drag" or
// in a certain region of the screen, etc.

// pub fn get_camera_position_delta(
//     camera: &Camera,
//     input_state: &InputState,
//     delta_s: f32,
// ) -> cgmath::Vector3<f32> {
//     let mut delta_forward: f32 = 0.0;
//     let mut delta_up: f32 = 0.0;
//     let mut delta_right: f32 = 0.0;
//     if input_state.key_pressed(&VirtualKeyCode::W) || input_state.key_pressed(&VirtualKeyCode::Up) {
//         delta_forward += delta_s;
//     }
//     if input_state.key_pressed(&VirtualKeyCode::S) || input_state.key_pressed(&VirtualKeyCode::Down)
//     {
//         delta_forward -= delta_s;
//     }
//     if input_state.key_pressed(&VirtualKeyCode::D)
//         || input_state.key_pressed(&VirtualKeyCode::Right)
//     {
//         delta_right += delta_s;
//     }
//     if input_state.key_pressed(&VirtualKeyCode::A) || input_state.key_pressed(&VirtualKeyCode::Left)
//     {
//         delta_right -= delta_s;
//     }
//     if input_state.key_pressed(&VirtualKeyCode::Q) {
//         delta_up += delta_s;
//     }
//     if input_state.key_pressed(&VirtualKeyCode::E) {
//         delta_up -= delta_s;
//     }

//     delta_forward * camera.look_vector()
//         + delta_up * camera.up_vector()
//         + delta_right * camera.look_vector().cross(camera.up_vector()).normalize()
// }

// pub fn get_camera_yaw_deg_delta(input_state: &InputState) -> f32 {
//     if input_state.mouse_position_set() {
//         // Yaw increases as the mouse moves left, because our coordinate frame is
//         // X foward, Y to the left.
//         return -1.0 * input_state.mouse_delta().x;
//     } else {
//         return 0.0;
//     }
// }

// pub fn get_camera_pitch_deg_delta(input_state: &InputState) -> f32 {
//     if input_state.mouse_position_set() {
//         return -1.0 * input_state.mouse_delta().y;
//     } else {
//         return 0.0;
//     }
// }
