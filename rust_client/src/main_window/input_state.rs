use cgmath::Point2;
use std::collections::HashMap;
use winit::event::{MouseButton, VirtualKeyCode};

pub struct InputState {
    keys_pressed: HashMap<VirtualKeyCode, bool>,
    mouse_buttons_pressed: HashMap<MouseButton, bool>,
    mouse_position_set: bool,
    mouse_delta: cgmath::Vector2<f32>,
    scroll_angle_delta: f32,
    mouse_first_event: bool,
}

impl InputState {
    pub fn new() -> Self {
        let keys_pressed = HashMap::new();
        let mouse_buttons_pressed = HashMap::new();
        let mouse_delta = cgmath::Vector2::new(0.0, 0.0);
        Self {
            // keys_pressed: HashMap<VirtualKeyCode, bool>::new(),
            // mouse_buttons_pressed: HashMap<MouseButton, bool>::new(),
            // mouse_position: Point2<i32>::new(),
            // mouse_delta: Point2<i32>::new(),
            // scroll_angle_delta: 0,
            keys_pressed,
            mouse_buttons_pressed,
            mouse_position_set: false,
            mouse_delta,
            scroll_angle_delta: 0.0,
            mouse_first_event: true,
        }
    }

    pub fn key_pressed(&self, key: &VirtualKeyCode) -> bool {
        self.keys_pressed.get(key).copied().unwrap_or(false)
    }

    pub fn mouse_button_pressed(&self, button: &MouseButton) -> bool {
        self.mouse_buttons_pressed
            .get(button)
            .copied()
            .unwrap_or(false)
    }

    pub fn set_key_pressed(&mut self, key: &VirtualKeyCode) {
        self.keys_pressed.insert(*key, true);
    }

    pub fn set_key_released(&mut self, key: &VirtualKeyCode) {
        self.keys_pressed.insert(*key, false);
    }

    pub fn set_mouse_button_presssed(&mut self, button: &MouseButton) {
        self.mouse_buttons_pressed.insert(*button, true);
    }

    pub fn set_mouse_button_released(&mut self, button: &MouseButton) {
        self.mouse_buttons_pressed.insert(*button, false);
    }

    pub fn mouse_delta(&self) -> cgmath::Vector2<f32> {
        self.mouse_delta
    }

    pub fn mouse_position_set(&self) -> bool {
        self.mouse_position_set
    }

    pub fn add_mouse_delta(&mut self, delta_x: f32, delta_y: f32) {
        // println!("{:?} {:?}", delta_x, delta_y);
        self.mouse_delta.x = delta_x;
        self.mouse_delta.y = delta_y;
        self.mouse_position_set = true;
    }

    pub fn clear_mouse_delta(&mut self) {
        self.mouse_delta.x = 0.0;
        self.mouse_delta.y = 0.0;
        self.mouse_position_set = false;
    }

    pub fn is_first_mouse_event(&mut self) -> bool {
        self.mouse_first_event
    }

    pub fn set_is_first_mouse_event(&mut self, is_first_move: bool) {
        self.mouse_first_event = is_first_move;
    }
}
