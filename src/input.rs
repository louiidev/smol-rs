use std::collections::HashSet;
use hecs::{Entity, World};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::rect::Point;
use sdl2::EventPump;
use crate::components::Actor;
use crate::core::{MouseButton, get_context};
use crate::events::{Action, Events};
use crate::math::Vector2Int;
use crate::systems::run_actor_actions;

pub struct Input {
    pub input_previous_keyboard_state: HashSet<Keycode>,
    pub input_current_keyboard_state: HashSet<Keycode>,
    pub previous_mouse_state: HashSet<MouseButton>,
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
        !self.input_previous_keyboard_state.contains(&key) && self.input_current_keyboard_state.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.input_previous_keyboard_state.contains(&key) && self.input_current_keyboard_state.contains(&key)
    }

    pub fn is_key_released(&self, key: Keycode) -> bool {
        self.input_previous_keyboard_state.contains(&key) && !self.input_current_keyboard_state.contains(&key)
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
        let keys = events
            .mouse_state()
            .pressed_mouse_buttons()
            .collect();
        
        self.previous_mouse_state = self.current_mouse_state.clone();
        self.current_mouse_state = keys;
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

    pub fn get_mouse_pos(&mut self) -> Vector2Int {
        Vector2Int::new(self.mouse_state.x(), self.mouse_state.y())
    }
}


pub fn query_player_input(world: &mut World, player: Entity) {
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

    if temp_pos != Vector2Int::default() {
        {
            let mut actor = world.get_mut::<Actor>(player).unwrap();
            actor.action = Some(Action {
                cost: 1.,
                action: Events::Move(temp_pos)
            });
        }
        

        run_actor_actions(world);
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

pub fn get_mouse_pos() -> Vector2Int {
    get_context().input.get_mouse_pos()
}

pub fn is_key_down(key: Keycode) -> bool {
    let ctx = get_context();
    ctx.input.is_key_down(key)
}

pub fn is_key_released(key: Keycode) -> bool {
    let ctx = get_context();
    ctx.input.is_key_released(key)
}

pub fn is_key_pressed(key: Keycode) -> bool {
    let ctx = get_context();
    ctx.input.is_key_pressed(key)
}

pub fn is_mouse_down(key: MouseButton) -> bool {
    let ctx = get_context();
    ctx.input.is_mouse_down(key)
}

pub fn is_mouse_released(key: MouseButton) -> bool {
    let ctx = get_context();
    ctx.input.is_mouse_released(key)
}

pub fn is_mouse_pressed(key: MouseButton) -> bool {
    let ctx = get_context();
    ctx.input.is_mouse_pressed(key)
}