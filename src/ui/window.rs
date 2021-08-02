use crate::collision::is_point_inside_rectangle;
use crate::components::{Inventory, Item};
use crate::core::{
    clear, end_scissor, get_text_bounds, get_window_size, queue_text, queue_text_ex, render_rect,
    render_text_queue, reset_offset, set_offset, start_scissor, MouseButton,
};
use crate::input::{get_mouse_pos, InputState};
use crate::math::{Rectangle, Vector2};
use crate::render::Color;
use crate::text_render::{TextAlignment, TextQueueConfig};
use glyph_brush::ab_glyph::Rect;
use hecs::{Entity, World};

use super::{line_border, UiEvent};

pub fn start_window(dimensions: &Rectangle) {
    let Rectangle { x, y, w, h } = *dimensions;

    let height = get_window_size().y;
    // Scissor starts from bottom left
    let scissor_y = (height as f32) - h - y; // windowHeight - rect.Height - rect.Y
    render_text_queue();
    render_rect(x, y - 30., w, 30., Color(200, 200, 200, 1.));
    start_scissor(x as _, scissor_y as _, w as _, h as _);
    set_offset(Vector2 { x, y });
}

pub fn end_window(dimensions: &Rectangle) {
    render_text_queue();
    reset_offset();
    line_border(*dimensions, 1., Color(200, 200, 200, 1.));
    end_scissor()
}

pub struct WindowState {
     pub selected: bool,
}

pub struct ItemsWindow {
    state: WindowState,
    items: Vec<Box<dyn Item>>,
    focus_index: Option<usize>,
    dimensions: Rectangle,
}

impl ItemsWindow {
    pub fn new(
        items: Vec<Box<dyn Item>>,
    ) -> Self {
        Self {
            state: WindowState { selected: false },
            items,
            focus_index: None,
            dimensions: Rectangle { x: 240., y: 110., w: 500., h: 300. },
        }
    }

    pub fn set_items(&mut self, items: Vec<Box<dyn Item>>) {
        self.items = items;
    }

    pub fn update(&mut self, input_state: &mut InputState, world: &World, player: Entity) {

        if self.items.is_empty() || self.state.selected {
            let inventory = world.get::<Inventory>(player).unwrap();
            self.items = inventory.items.clone();
            self.state.selected = false;
        }

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

                if let Some(ui_event) = input_state.ui_event {
                    println!("ui event: {:?}", ui_event);
                    match ui_event {
                        UiEvent::MouseButtonDown(btn) => match btn {
                            MouseButton::Left => {
                                input_state.selected_item = Some(item.to_owned());
                                self.state.selected = true;
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }

            last_bounds = Some(bounds);
        }
    }

    pub fn render(&self) {
        let Rectangle { x, y, w, h } = self.dimensions;

        queue_text_ex(
            "Items",
            TextQueueConfig {
                position: Vector2::new(x + w / 2., y - 30.),
                font_size: 14.,
                color: Color(1, 1, 1, 1.),
                horizontal_alginment: TextAlignment::Center,
            },
        );

        start_window(&self.dimensions);
        clear(Color(28, 33, 43, 1.));

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
                    render_rect(
                        0.,
                        y_pos,
                        w,
                        last_bounds.unwrap().height(),
                        Color(200, 100, 100, 1.),
                    );
                }
            }
        }
        end_window(&self.dimensions);
    }
}
