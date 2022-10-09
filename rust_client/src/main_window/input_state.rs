use cgmath::Point2;
use std::collections::HashMap;
use winit::event::{MouseButton, VirtualKeyCode};

pub struct InputState {
    keys_pressed: HashMap<VirtualKeyCode, bool>,
    mouse_buttons_pressed: HashMap<MouseButton, bool>,
    mouse_position: Point2<i32>,
    mouse_delta: Point2<i32>,
    scroll_angle_delta: f32,
}

impl InputState {
    pub fn new() -> Self {
        let keys_pressed = HashMap::new();
        let mouse_buttons_pressed = HashMap::new();
        let mouse_position = Point2::new(0, 0);
        let mouse_delta = Point2::new(0, 0);
        Self {
            // keys_pressed: HashMap<VirtualKeyCode, bool>::new(),
            // mouse_buttons_pressed: HashMap<MouseButton, bool>::new(),
            // mouse_position: Point2<i32>::new(),
            // mouse_delta: Point2<i32>::new(),
            // scroll_angle_delta: 0,
            keys_pressed,
            mouse_buttons_pressed,
            mouse_position,
            mouse_delta,
            scroll_angle_delta: 0.0,
        }
    }

    pub fn key_presssed(&self, key: &VirtualKeyCode) -> bool {
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
}
