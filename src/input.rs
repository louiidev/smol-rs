use std::collections::HashSet;
use hecs::{Entity, World};
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::rect::Point;
use sdl2::EventPump;
use crate::components::Actor;
use crate::core::{MouseButton, get_context, get_window_scale};
use crate::events::{Action, Events};
use crate::map::get_map;
use crate::math::{Vector2, Vector2Int};
use crate::pathfinding::a_star;
use crate::queries::get_entity_grid_position;
use crate::systems::run_actor_actions;
use crate::ui::{self, ContextMenuAction, UiEvent};
use crate::world_setup::WorldPlayer;





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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlType {
    Mouse,
    Keyboard(Vector2Int)
}

impl Default for ControlType {
    fn default() -> Self {
        Self::Mouse
    }
}


// Different from the engine Input struct
// This is more handling player actions for ui/world inputs
#[derive(Debug, Default, Clone)]
pub struct InputState {
    pub path: Vec<Vector2Int>,
    pub start_grid_path: Option<Vector2Int>,
    pub end_grid_path: Option<Vector2Int>,
    pub control_type: ControlType,
    pub ui_action_type: Option<ContextMenuAction>,
    pub ui_event: Option<UiEvent>,
    pub world_input_block: bool,
    pub context_menu_position: Option<Vector2>,
}

pub fn screen_to_grid(screen_pos: Vector2Int) -> Vector2Int {
    let scale = get_window_scale().x;
    let grid_scale = 16 * scale as i32;
    let grid_pos = screen_pos / grid_scale;

    grid_pos
}


pub fn query_world_input(input_state: &mut InputState, world: &World, player: Entity) {

    if input_state.world_input_block {
        return;
    }

    let mut temp_pos = None;

    if is_key_down(Keycode::W) {
        temp_pos = Some(Vector2Int::down())
    } else if is_key_down(Keycode::S) {
        temp_pos = Some(Vector2Int::up())
    }

    if is_key_down(Keycode::A) {
        temp_pos = Some(Vector2Int::left());
    } else if is_key_down(Keycode::D) {
        temp_pos = Some(Vector2Int::right());
    }

    if temp_pos.is_some() {
        let player_pos = get_entity_grid_position(world, player);
        input_state.path = vec![temp_pos.unwrap() + player_pos];
    }

    if is_key_down(Keycode::P) {
        let player_pos = get_entity_grid_position(world, player);
        input_state.control_type = ControlType::Keyboard(player_pos);
        input_state.start_grid_path = Some(player_pos);
    }

    if input_state.start_grid_path.is_some() {
        input_state.end_grid_path = Some(match input_state.control_type {
            ControlType::Mouse => screen_to_grid(get_mouse_pos()),
            ControlType::Keyboard(target_position) => target_position,
        });

        if is_mouse_down(MouseButton::Left) {
            let path = a_star(
                get_map().get_current_chunk().tiles.clone(),
                input_state.start_grid_path.unwrap(),
                input_state.end_grid_path.unwrap()
            );

            if path.is_some() {
                input_state.path = path.unwrap();
            }

            input_state.start_grid_path = None;
            input_state.end_grid_path = None;
        }

    } else if is_mouse_down(MouseButton::Left) {
        let player_pos = get_entity_grid_position(world, player);
        if player_pos == screen_to_grid(get_mouse_pos()) {
            input_state.control_type = ControlType::Mouse;
            input_state.start_grid_path = Some(player_pos);
        }
    }
}

pub fn query_ui_input(input_state: &mut InputState) {
    if is_mouse_down(MouseButton::Left) {
        input_state.ui_event = Some(UiEvent::MouseButtonDown(MouseButton::Left));
    } else if is_mouse_down(MouseButton::Right) {
        input_state.ui_event = Some(UiEvent::MouseButtonDown(MouseButton::Right));
    }
}

pub fn update(input_state: &mut InputState, world: &mut World, player: Entity) {



    if let Some(position) = input_state.path.pop() {
        {
            let mut actor = world.get_mut::<Actor>(player).unwrap();
            actor.action = Some(Action {
                cost: 1.,
                event: Events::MoveTo(position),
            });
        }
        
        run_actor_actions(world);
    }
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