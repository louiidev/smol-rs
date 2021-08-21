use crate::components::{Actor, Inventory, Item};
use crate::core::{get_context, get_window_scale, MouseButton};
use crate::events::{Action, Events, ThrowAction};
use crate::map::get_map;
use crate::math::{Vec2, Vec2Int};
use crate::pathfinding::a_star;
use crate::queries::{get_entity_grid_position, get_player_entity};
use crate::systems::run_actor_actions;
use crate::ui::ContextMenuAction;
use hecs::World;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseState;
use sdl2::EventPump;
use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq)]
pub enum InputTransition {
    Released,
    Down,
    Pressed,
}

pub type KeypressCapture = (InputTransition, Keycode);
pub type MouseButtonCapture = (InputTransition, MouseButton);

pub struct Input {
    pub input_previous_keyboard_state: HashSet<Keycode>,
    pub input_current_keyboard_state: HashSet<Keycode>,
    pub previous_mouse_state: HashSet<MouseButton>,
    pub current_mouse_state: HashSet<MouseButton>,
    mouse_state: MouseState,
    captured_keyboard_input: HashSet<(InputTransition, Keycode)>,
    captured_mouse_input: HashSet<(InputTransition, MouseButton)>,
}

impl Input {
    pub fn new() -> Self {
        Input {
            input_previous_keyboard_state: HashSet::new(),
            input_current_keyboard_state: HashSet::new(),
            previous_mouse_state: HashSet::new(),
            current_mouse_state: HashSet::new(),
            mouse_state: MouseState::from_sdl_state(0),
            captured_keyboard_input: HashSet::new(),
            captured_mouse_input: HashSet::new(),
        }
    }
}

impl Input {
    pub fn is_key_down(&self, key: Keycode) -> bool {
        !self.input_previous_keyboard_state.contains(&key)
            && self.input_current_keyboard_state.contains(&key)
            && !self
                .captured_keyboard_input
                .contains(&(InputTransition::Down, key))
    }

    pub fn is_key_pressed(&self, key: Keycode) -> bool {
        self.input_previous_keyboard_state.contains(&key)
            && self.input_current_keyboard_state.contains(&key)
            && !self
                .captured_keyboard_input
                .contains(&(InputTransition::Pressed, key))
    }

    pub fn is_key_released(&self, key: Keycode) -> bool {
        self.input_previous_keyboard_state.contains(&key)
            && !self.input_current_keyboard_state.contains(&key)
            && !self
                .captured_keyboard_input
                .contains(&(InputTransition::Released, key))
    }

    pub fn is_mouse_down(&self, key: MouseButton) -> bool {
        !self.previous_mouse_state.contains(&key)
            && self.current_mouse_state.contains(&key)
            && !self
                .captured_mouse_input
                .contains(&(InputTransition::Down, key))
    }

    pub fn is_mouse_pressed(&self, key: MouseButton) -> bool {
        self.previous_mouse_state.contains(&key)
            && self.current_mouse_state.contains(&key)
            && !self
                .captured_mouse_input
                .contains(&(InputTransition::Pressed, key))
    }

    pub fn is_mouse_released(&self, key: MouseButton) -> bool {
        self.previous_mouse_state.contains(&key)
            && !self.current_mouse_state.contains(&key)
            && !self
                .captured_mouse_input
                .contains(&(InputTransition::Released, key))
    }

    pub fn set_mouse_state(&mut self, events: &EventPump) {
        let state = events.mouse_state();
        let keys = events.mouse_state().pressed_mouse_buttons().collect();

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
        self.captured_mouse_input = HashSet::default();
        self.captured_keyboard_input = HashSet::default();
    }

    pub fn get_mouse_pos(&mut self) -> Vec2Int {
        Vec2Int::new(self.mouse_state.x(), self.mouse_state.y())
    }

    pub fn set_captured_keypress(&mut self, captured_keypress: KeypressCapture) {
        // Shouldnt be called more than once per key
        assert_eq!(
            self.captured_keyboard_input.contains(&captured_keypress),
            false
        );
        self.captured_keyboard_input.insert(captured_keypress);
    }

    pub fn set_captured_mousepress(&mut self, captured_keypress: MouseButtonCapture) {
        // Shouldnt be called more than once per key
        assert_eq!(
            self.captured_mouse_input.contains(&captured_keypress),
            false
        );
        self.captured_mouse_input.insert(captured_keypress);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlType {
    Mouse,
    Keyboard(Vec2Int),
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
    pub path: Vec<Vec2Int>,
    pub start_grid_path: Option<Vec2Int>,
    pub end_grid_path: Option<Vec2Int>,
    pub control_type: ControlType,
    pub ui_action_type: Option<ContextMenuAction>,
    pub world_input_block: bool,
    pub context_menu_position: Option<Vec2>,
    pub selected_item: Option<Box<dyn Item>>,
}

pub fn screen_to_grid(screen_pos: Vec2Int) -> Vec2Int {
    let scale = get_window_scale().x;
    let grid_scale = 16 * scale as i32;
    let grid_pos = screen_pos / grid_scale;

    grid_pos
}

pub fn query_world_input(input_state: &mut InputState, world: &World) {
    if input_state.world_input_block {
        return;
    }

    let mut temp_pos = None;

    if is_key_down(Keycode::W) {
        temp_pos = Some(Vec2Int::DOWN);
    } else if is_key_down(Keycode::S) {
        temp_pos = Some(Vec2Int::UP);
    }

    if is_key_down(Keycode::A) {
        temp_pos = Some(Vec2Int::LEFT);
    } else if is_key_down(Keycode::D) {
        temp_pos = Some(Vec2Int::RIGHT);
    }

    if temp_pos.is_some() {
        if let Some(player) = get_player_entity(world) {
            let player_pos = get_entity_grid_position(world, player);
            input_state.path = vec![temp_pos.unwrap() + player_pos];
        }
    }

    if is_key_down(Keycode::P) {
        if let Some(player) = get_player_entity(world) {
            let player_pos = get_entity_grid_position(world, player);
            input_state.control_type = ControlType::Keyboard(player_pos);
            input_state.start_grid_path = Some(player_pos);
        }
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
                input_state.end_grid_path.unwrap(),
            );

            if path.is_some() {
                input_state.path = path.unwrap();
            }

            input_state.start_grid_path = None;
            input_state.end_grid_path = None;
        }
    } else if is_mouse_down(MouseButton::Left) {
        if let Some(player) = get_player_entity(world) {
            let player_pos = get_entity_grid_position(world, player);
            if player_pos == screen_to_grid(get_mouse_pos()) {
                input_state.control_type = ControlType::Mouse;
                input_state.start_grid_path = Some(player_pos);
            }
        }
    }
}

pub fn update(input_state: &mut InputState, world: &mut World) {
    let mut run_actions = false;

    if let Some(position) = input_state.path.pop() {
        if let Some(player) = get_player_entity(world) {
            {
                let mut actor = world.get_mut::<Actor>(player).unwrap();
                actor.action = Some(Action {
                    cost: 1.,
                    event: Events::MoveTo(position),
                });
            }
            run_actions = true;
        }
    } else if let Some(ui_action_type) = input_state.ui_action_type {
        match ui_action_type {
            ContextMenuAction::ThrowItem(target) => {
                if let Some(player) = get_player_entity(world) {
                    if let Some(item) = input_state.selected_item.take() {
                        run_actions = true;
                        let mut inventory = world.get_mut::<Inventory>(player).unwrap();
                        let to_remove_index = inventory
                            .items
                            .iter()
                            .position(|i| &i.name() == &item.name())
                            .unwrap();
                        inventory.items.remove(to_remove_index);
                        let mut actor = world.get_mut::<Actor>(player).unwrap();
                        actor.action = Some(Action {
                            cost: 1.,
                            event: Events::ThrowItem(ThrowAction { item, target }),
                        });
                        input_state.ui_action_type = None;
                    }
                }
            }
            _ => {}
        }
    }

    if run_actions {
        run_actor_actions(world);
    }
}

pub fn set_captured_mousepress(captured_keypress: MouseButtonCapture) {
    get_context()
        .input
        .set_captured_mousepress(captured_keypress)
}

pub fn set_captured_keypress(captured_keypress: KeypressCapture) {
    get_context().input.set_captured_keypress(captured_keypress)
}

pub fn get_mouse_pos() -> Vec2Int {
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
