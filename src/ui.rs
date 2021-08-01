use std::{
    fmt::Display,
    sync::{Mutex, MutexGuard},
};

use crate::{components::{Inventory, Item}, core::get_text_bounds, text_render::TextAlignment};
use glyph_brush::ab_glyph::Rect;
use hecs::{Entity, World};

use crate::{
    collision::is_point_inside_rectangle,
    core::{
        clear, end_scissor, get_window_size, queue_multiple_text, queue_text, queue_text_ex,
        render_rect, render_text_queue, reset_offset, set_offset, start_scissor, Keycode,
        MouseButton,
    },
    events::Events,
    input::{self, get_mouse_pos, is_key_down, is_mouse_down, screen_to_grid, InputState},
    logging::get_log,
    map::get_map,
    math::{Rectangle, Vector2, Vector2Int},
    pathfinding::a_star,
    queries::{get_entity_at_grid_position, get_entity_grid_position},
    render::{Color, BLUE, RED},
    text_render::TextQueueConfig,
};

// lazy_static! {
//     static ref UI_CONTEXT: Mutex<UiState> = {
//         Mutex::new(UiState {
//             context_position: None,
//             focus_id: None
//         })
//     };
// }

#[derive(Debug, Clone, PartialEq)]
pub enum UiElements {
    ContextMenu,
}

// pub struct UiState {
//     pub context_position: Option<Vector2>,
//     pub focus_id: Option<String>
// }

// impl UiState {
//     pub fn is_focused(&self, id: &str) -> bool {
//         match &self.focus_id {
//             Some(focus_element) => {
//                 focus_element == id
//             },
//             None => false
//         }
//     }
// }

// pub fn get_ui_state() -> std::sync::MutexGuard<'static, UiState> {
//     UI_CONTEXT.lock().unwrap()
// }

#[derive(Default, Debug)]
pub struct ContextItem {
    pub text: String,
    pub action: ContextMenuAction,
    pub children: Vec<ContextItem>,
}

#[derive(Debug, Clone, Copy)]
pub enum ContextMenuAction {
    Move,
    NOOP,
    ThrowItem(Entity),
}

impl Default for ContextMenuAction {
    fn default() -> Self {
        Self::NOOP
    }
}

#[derive(Default, Debug)]
pub struct ContextMenu {
    focused_index: Option<usize>,
    items: Vec<ContextItem>,
    position: Option<Vector2>,
}

impl ContextMenu {
    pub fn update(&mut self, input_state: &mut InputState, world: &World, player: Entity) {
        if is_mouse_down(MouseButton::Left) {
            if let Some(pos) = self.position {
                if let Some(index) = self.focused_index {
                    if let Some(item) = self.items.get(index) {
                        match item.action {
                            ContextMenuAction::Move => {
                                let start = get_entity_grid_position(&world, player);
                                let grid_pos = screen_to_grid(pos.into());
                                input_state.path = a_star(
                                    get_map().get_current_chunk().tiles.clone(),
                                    start,
                                    grid_pos,
                                )
                                .unwrap_or(Vec::default());
                            }
                            ContextMenuAction::NOOP => todo!(),
                            ContextMenuAction::ThrowItem(e) => {
                                input_state.ui_action_type = Some(ContextMenuAction::ThrowItem(e))
                            }
                        }
                    }
                }
            }
        }

        let mut remake_items = false;

        if let Some(ui_event) = input_state.ui_event {
            match ui_event {
                UiEvent::MouseButtonDown(btn) => {
                    match btn {
                        MouseButton::Left => {
                            self.position = None;
                        },
                        MouseButton::Right => {
                            let pos = get_mouse_pos();
                            self.position = Some(pos.into());
                            remake_items = true;
                        },
                        _ => {},
                    }
                    input_state.context_menu_position = self.position;
                    input_state.ui_event = None;
                },
                _ => {}
            }
        }

        if remake_items {
            self.items = Vec::default();
            if let Some(pos) = self.position {
                let grid_pos = screen_to_grid(pos.into());
                let ent = get_entity_at_grid_position(world, grid_pos);
                if let Some(entity) = ent {
                    self.items.push(ContextItem {
                        text: "throw item".to_string(),
                        action: ContextMenuAction::ThrowItem(entity),
                        ..Default::default()
                    })
                } else {
                    let tile = get_map()
                        .get_tile_from_grid_position(grid_pos)
                        .unwrap()
                        .clone();
                    if tile.walkable {
                        self.items.push(ContextItem {
                            text: "Move to".to_string(),
                            action: ContextMenuAction::Move,
                            ..Default::default()
                        });
                    }
                }
            }
        }
    }

    pub fn render(&mut self) {
        let current_mouse_pos = get_mouse_pos();
        // shouldnt call render unless position has been set
        let position = self.position.unwrap();

        let padding: f32 = 10.;
        let mut min_width = 0.;
        let mut bounds: Vec<Rect> = Vec::default();

        for i in 0..self.items.len() {
            let item = &self.items[i];
            let tooltip_pos = {
                if i as i32 - 1 >= 0 && bounds.get(i - 1).is_some() {
                    let bounds = bounds.get(i - 1).unwrap();
                    Vector2::new(position.x + padding, bounds.max.y)
                } else {
                    Vector2::new(position.x + padding, position.y)
                }
            };

            let rect = queue_text(&item.text, tooltip_pos, 14., Color(255, 255, 255, 1.)).unwrap();

            if rect.width() > min_width {
                min_width = rect.width();
            }

            bounds.push(rect);
        }

        for i in 0..self.items.len() {
            let focused = self.focused_index == Some(i);
            let r = *bounds.get(i).unwrap();
            let rect = Rectangle {
                x: (r.min.x - padding),
                y: r.min.y,
                w: min_width + padding * 2.,
                h: r.height(),
            };

            let collision = is_point_inside_rectangle(current_mouse_pos, &rect);

            if collision && !focused {
                self.focused_index = Some(i);
            } else if !collision && focused {
                self.focused_index = None;
            }

            let color = if focused {
                Color(150, 60, 60, 1.)
            } else {
                Color(50, 50, 50, 1.)
            };
            render_rect(rect.x, rect.y, rect.w, rect.h, color);
        }
    }
}

pub fn rect_border(mut rect: Rectangle, thickness: f32, color: Color) {
    rect.add_padding(Vector2::new(thickness, thickness));

    render_rect(rect.x, rect.y, rect.w, rect.h, color);
}

pub fn line_border(rect: Rectangle, thickness: f32, color: Color) {
    // TOP
    render_rect(rect.x, rect.y, rect.w, thickness, color);
    // BOTTOM
    render_rect(
        rect.x,
        rect.y + rect.h - thickness,
        rect.w,
        thickness,
        color,
    );
    // LEFT
    render_rect(rect.x, rect.y, thickness, rect.h, color);
    //RIGHT
    render_rect(
        rect.x + rect.w - thickness,
        rect.y,
        thickness,
        rect.h,
        color,
    );
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ItemPressed(usize)
}

#[derive(Debug, Clone, Copy)]
pub enum UiEvent {
    MouseButtonDown(MouseButton),
    MouseMove,
    EventHandled,
}



pub struct ButtonState {
    is_pressed: bool
}

pub struct Button<'a> {
    on_press: Option<Message>,
    state: &'a mut ButtonState,
    text: String,
}

impl<'a> Button<'a> {
    pub fn new(state: &'a mut ButtonState, text: &str) -> Self {
        Self {
            state,
            text: text.to_owned(),
            on_press: None,
        }
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }
}

pub struct LogInfoBox {}

const MAX_ITEMS: usize = 8;
impl LogInfoBox {
    pub fn render(&self) {
        render_text_queue();

        let log_items = &get_log().items;
        let items = if log_items.len() > MAX_ITEMS {
            &log_items[log_items.len() - MAX_ITEMS..log_items.len()]
        } else {
            &log_items[0..log_items.len()]
        };

        let window_size: Vector2 = get_window_size().into();
        let height = 200.;
        let width = window_size.x - 310.;

        let start_position = Vector2::new(10., window_size.y - 40.);
        start_scissor(0, (0) as _, width as _, height as _);

        render_rect(
            0.,
            window_size.y - height,
            width,
            height,
            Color(28, 33, 43, 1.),
        );

        let rect = Rectangle {
            x: 0.,
            y: window_size.y - height,
            w: width,
            h: height,
        };

        let mut last_bounds: Option<Rect> = None;
        for item in items {
            let position = if let Some(bounds) = last_bounds {
                Vector2::new(start_position.x, bounds.min.y - bounds.height())
            } else {
                start_position
            };

            last_bounds = queue_multiple_text(item.clone(), position, 18.)
        }

        render_text_queue();

        line_border(rect, 5.0, Color(255, 255, 255, 1.));

        end_scissor();
    }
}

pub struct SideBar {}

impl SideBar {
    pub fn render(&self) {
        let window_size: Vector2 = get_window_size().into();
        let height = window_size.y;
        let width = 300.;

        render_rect(
            window_size.x - width,
            0.,
            width,
            height,
            Color(28, 33, 43, 1.),
        );

        let rect = Rectangle {
            x: window_size.x - width,
            y: 0.,
            w: width,
            h: height,
        };

        line_border(rect, 5.0, Color(255, 255, 255, 1.));
    }
}

#[derive(Default)]
pub struct ItemsWindow {
    dimensions: Rectangle,
    should_render: bool,
    items: Vec<Box<dyn Item>>,
    focus_index: Option<usize>,
}

impl ItemsWindow {
    pub fn new(dimensions: Rectangle) -> Self {
        Self {
            dimensions,
            ..Default::default()
        }
    }

    pub fn update(&mut self, input_state: &mut InputState, world: &World, player: Entity) {
        self.should_render = if let Some(ui_action) = input_state.ui_action_type {
            match ui_action {
                ContextMenuAction::ThrowItem(_) => true,
                _ => false,
            }
        } else {
            false
        };

        if self.should_render {
            if self.items.is_empty() {
                let inventory = world.get::<Inventory>(player).unwrap();
                self.items = inventory.items.clone();
            }
        } else if !self.items.is_empty() {
            self.items = Vec::default();
        }


        // Focus item logic
        if self.should_render && !self.items.is_empty() {
            let mouse_pos = get_mouse_pos();
            let mut last_bounds: Option<Rect> = None;
            for index in 0..self.items.len() {
                let item = self.items.get(index).unwrap();
                let y_pos = if let Some(bounds) = last_bounds {
                    bounds.max.y
                } else {
                    10.
                };
                let bounds = get_text_bounds(item.name(), Vector2::new(0., y_pos), 14.).unwrap();
                let Rectangle { x, y, .. } = self.dimensions;
                let rect = Rectangle {
                    x: bounds.min.x + x,
                    y: bounds.min.y + y,
                    w: self.dimensions.w,
                    h: bounds.height(),
                };

                if is_point_inside_rectangle(mouse_pos, &rect) {
                    self.focus_index = Some(index);

                    if is_mouse_down(MouseButton::Left) {

                    }
                }

                last_bounds = Some(bounds);
            }

        }

    }

    pub fn render(&self) {
        let Rectangle { x, y, w, h } = self.dimensions;

        if !self.should_render {
            return;
        }

        let height = get_window_size().y;
        // Scissor starts from bottom left
        let scissor_y = (height as f32) - h - y; // windowHeight - rect.Height - rect.Y

        queue_text_ex(
            "Items",
            TextQueueConfig {
                position: Vector2::new(x + w / 2., y - 30.),
                font_size: 14.,
                color: Color(1, 1, 1, 1.),
                horizontal_alginment: TextAlignment::Center,
            },
        );

        render_rect(x, y - 30., w, 30., Color(200, 200, 200, 1.));
        render_text_queue();
        start_scissor(x as _, scissor_y as _, w as _, h as _);
        clear(Color(28, 33, 43, 1.));

        set_offset(Vector2 { x, y });

        let mut last_bounds: Option<Rect> = None;
        let mut inventory_height = 0.;
        for index in 0..self.items.len() {
            let item = self.items.get(index).unwrap();
            let y_pos = if let Some(bounds) = last_bounds {
                bounds.max.y
            } else {
                10.
            };

            last_bounds = queue_text(
                item.name(),
                Vector2::new(10., y_pos),
                14.,
                Color(255, 255, 255, 1.),
            );

            if let Some(bounds) = last_bounds {
                inventory_height += bounds.height();
            }

            if let Some(focus_index) = self.focus_index {
                if focus_index == index {
                    render_rect(0., y_pos, w, last_bounds.unwrap().height(), Color(200, 100, 100, 1.));
                }
            }

        }

        render_text_queue();
        reset_offset();
        line_border(self.dimensions, 1., Color(200, 200, 200, 1.));
        end_scissor()
    }
}
