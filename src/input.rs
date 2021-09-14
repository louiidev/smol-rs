use crate::math::Vector2;
use crate::MouseButton;
use nalgebra::Vector;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::EventPump;
use std::collections::HashSet;

pub struct Input {
    input_previous_keyboard_state: HashSet<Keycode>,
    pub input_current_keyboard_state: HashSet<Keycode>,
    previous_mouse_state: HashSet<MouseButton>,
    pub current_mouse_state: HashSet<MouseButton>,
    mouse_state: MouseState,
}

impl Input {
    pub fn new() -> Self {
        Input {
            input_previous_keyboard_state: HashSet::new(),
            input_current_keyboard_state: HashSet::new(),
            previous_mouse_state: HashSet::new(),
            current_mouse_state: HashSet::new(),
            mouse_state: MouseState::from_sdl_state(0),
        }
    }
}

impl Input {
    pub fn is_key_down(&self, key: Keycode) -> bool {
        !self.input_previous_keyboard_state.contains(&key)
            && self.input_current_keyboard_state.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.input_previous_keyboard_state.contains(&key)
            && self.input_current_keyboard_state.contains(&key)
    }

    pub fn is_key_released(&self, key: Keycode) -> bool {
        self.input_previous_keyboard_state.contains(&key)
            && !self.input_current_keyboard_state.contains(&key)
    }

    pub fn is_mouse_down(&self, key: MouseButton) -> bool {
        !self.previous_mouse_state.contains(&key) && self.current_mouse_state.contains(&key)
    }

    pub fn is_mouse_pressed(&self, key: MouseButton) -> bool {
        self.previous_mouse_state.contains(&key) && self.current_mouse_state.contains(&key)
    }

    pub fn is_mouse_released(&self, key: MouseButton) -> bool {
        self.previous_mouse_state.contains(&key) && !self.current_mouse_state.contains(&key)
    }

    pub fn set_mouse_state(&mut self, events: &EventPump) {
        let state = events.mouse_state();
        let keys = events.mouse_state().pressed_mouse_buttons().collect();

        self.previous_mouse_state = self.current_mouse_state.clone();
        self.current_mouse_state = keys;
        self.mouse_state = state;
    }

    pub fn get_pressed_keys(&self) -> HashSet<&Keycode> {
        self.input_current_keyboard_state
            .intersection(&self.input_previous_keyboard_state)
            .collect()
    }

    pub fn set_keys(&mut self, events: &EventPump) {
        let keys = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        self.input_previous_keyboard_state = self.input_current_keyboard_state.clone();
        self.input_current_keyboard_state = keys;
    }

    pub fn get_mouse_pos(&mut self) -> Vector2<i32> {
        Vector::from([self.mouse_state.x(), self.mouse_state.y()])
    }
}
