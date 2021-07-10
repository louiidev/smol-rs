use std::collections::HashSet;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::rect::Point;
use sdl2::EventPump;
use crate::core::get_context;
use crate::math::Vector2Int;

pub struct Input {
    pub input_previous_keyboard_state: HashSet<Keycode>,
    pub input_current_keyboard_state: HashSet<Keycode>,
    mouse_state: MouseState,
}

impl Input {
    pub fn new() -> Self {
        Input {
            input_previous_keyboard_state: HashSet::new(),
            input_current_keyboard_state: HashSet::new(),
            mouse_state: MouseState::from_sdl_state(0),
        }
    }
}

impl Input {
    pub fn is_key_down(&self, key: Keycode) -> bool {
        !self.input_previous_keyboard_state.contains(&key) && self.input_current_keyboard_state.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.input_previous_keyboard_state.contains(&key) && self.input_current_keyboard_state.contains(&key)
    }

    pub fn is_key_released(&self, key: Keycode) -> bool {
        self.input_previous_keyboard_state.contains(&key) && !self.input_current_keyboard_state.contains(&key)
    }

    pub fn set_mouse_state(&mut self, events: &EventPump) {
        let state = events.mouse_state();
        self.mouse_state = state;
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

    pub fn get_mouse_pos(&mut self) -> Point {
        Point::new(self.mouse_state.x(), self.mouse_state.y())
    }
}


pub fn get_player_direction() -> Vector2Int {
    let mut temp_pos = Vector2Int::default();
    let input = &get_context().input;
    if input.is_key_down(Keycode::W) {
        temp_pos.y -= 1;
    } else if input.is_key_down(Keycode::S) {
        temp_pos.y += 1;
    }

    if input.is_key_down(Keycode::A) {
        temp_pos.x -= 1;
    } else if input.is_key_down(Keycode::D) {
        temp_pos.x += 1;
    }

    temp_pos
}