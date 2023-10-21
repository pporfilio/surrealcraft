pub enum LogicalButton {
    MouseLeft,
    KeyA,
    KeyB,
    KeyLeftCtrl,
}

pub struct ButtonEvent {
    pub logical_button: LogicalButton,
    pub is_pressed: bool,
    pub timestamp: Instant,
}

pub struct MouseMoveEvent {
    pub position_x: f64,
    pub position_y: f64,
    pub timestamp: Instant,
}

pub struct ButtonState {
    pub logical_button: VirtualKeyCode,
    pub is_pressed: bool,
    pub last_transition: Option<Instant>,
}

pub struct MouseState {
    pub current_x: f64,
    pub current_y: f64,
    pub previous_x: Option<f64>,
    pub previous_y: Option<f64>,
    pub current_timestamp: Option<Instant>,
    pub previous_timestamp: Option<Instant>,
}

pub struct InputState {
    pub keys: HashMap<LogicalButton, ButtonState>,
    pub mouse: Option<MouseState>,
}

impl InputState {
    pub fn apply_button_event(event: ButtonEvent) {
        // This says to look up a key in the map [entry()]
        // and if that key exists, modify it in-place [and_modify()]
        // or if the key doesn't exist, create a new entry [or_insert()]
        self.keys
            .entry(event.logical_button)
            .and_modify(|entry| {
                if event.is_pressed != *entry.is_pressed {
                    *entry.is_pressed = event.is_pressed;
                    *entry.last_transition = Some(event.timestamp);
                }
            })
            .or_insert(ButtonState {
                logical_button: event.logical_button,
                is_pressed: event.is_pressed,
                last_transition: None,
            })
    }
    pub fn apply_mouse_event(event: MouseEvent) {
        match self.mouse {
            Some(mouse) => {
                mouse.previous_x = mouse.current_x;
                mouse.previous_y = mouse.current_y;
                mouse.current_x = event.position_x;
                mouse.current_y = event.position_y;
                mouse.previous_timestamp = mouse.current_timestamp;
                mouse.current_timestamp = event.timestamp;
            }
            None => {
                self.mouse = Some(MouseState {
                    current_x: event.position_x,
                    current_y: event.position_y,
                    previous_x: None,
                    previous_y: None,
                    current_timestamp: Some(event.timestamp),
                    previous_timestamp: None,
                })
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
