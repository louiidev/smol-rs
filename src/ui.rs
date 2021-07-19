use std::sync::Mutex;

use glyph_brush::ab_glyph::Rect;
use lazy_static::lazy_static;

use crate::{
    collision::{is_point_inside_rect, is_point_inside_rectangle},
    core::{queue_text, render_rect},
    input::get_mouse_pos,
    math::{Rectangle, Vector2},
    render::Color,
};

lazy_static! {
    static ref UI_CONTEXT: Mutex<UiState> = {
        Mutex::new(UiState {
            focus_index: None,
            focus_element: None,
        })
    };
}


#[derive(Debug, Clone, PartialEq)]
pub enum UiElements {
    ContextMenu,
}

pub struct UiState {
    pub focus_index: Option<usize>,
    pub focus_element: Option<UiElements>
}

impl UiState {
    pub fn is_focused(&self, element: UiElements, index: usize) -> bool {
        match &self.focus_element {
            Some(focus_element) => {
                match self.focus_index {
                    Some(focus_index) => {
                        focus_element == &element && focus_index == index
                    },
                    None => false
                }
            },
            None => false
        }
    }
}

pub fn get_ui_state() -> std::sync::MutexGuard<'static, UiState> {
    UI_CONTEXT.lock().unwrap()
}

pub struct ContextItem {
    pub text: String
}

pub fn render_context_menu(position: Vector2, items: Vec<ContextItem>) {
    let mut last_bounds: Option<Rect> = None;
    let mut ui_state = get_ui_state();
    let current_mouse_pos = get_mouse_pos();
    for i in 0..items.len() {
        let item = &items[i];
        let focused = ui_state.is_focused(UiElements::ContextMenu, i);
        let padding: f32 = 10.;
        let tooltip_pos = {
            if let Some(bounds) = last_bounds {
                Vector2::new(position.x + padding, bounds.max.y)
            } else {
                Vector2::new(position.x + padding, position.y)
            }
        };
        let bounds = queue_text(
            &item.text,
            tooltip_pos,
            14.,
            Color(255, 255, 255, 1.),
        )
        .unwrap();
        let collision = is_point_inside_rect(
            current_mouse_pos,
            (bounds.min.x - padding) as _,
            bounds.min.y as _,
            (bounds.width() + padding * 2.) as _,
            bounds.height() as _,
        );

        if collision && !focused {
            ui_state.focus_index = Some(i);
            ui_state.focus_element = Some(UiElements::ContextMenu);
        } else if !collision && focused {
            ui_state.focus_index = None;
            ui_state.focus_element = None;
        }

        let color = if focused {
            Color(150, 60, 60, 1.)
        } else {
            Color(50, 50, 50, 1.)
        };
        render_rect(
            bounds.min.x - padding,
            bounds.min.y,
            bounds.width() + padding * 2.,
            bounds.height(),
            color,
        );
        last_bounds = Some(bounds);
    }
}

pub fn rect_border(mut rect: Rectangle, thickness: f32, color: Color) {
    rect.add_padding(Vector2::new(thickness, thickness));

    render_rect(rect.x, rect.y, rect.w, rect.h, color);
}

pub fn button(position: Vector2, text: &str) {
    let current_mouse_pos = get_mouse_pos();
    let bounds = queue_text(text, position, 14., Color(255, 255, 255, 1.)).unwrap();
    let padding: f32 = 10.;
    let border = 2.;
    let mut rect: Rectangle = bounds.into();
    rect.add_padding(Vector2::new(padding, 0.));
    let collision = is_point_inside_rectangle(current_mouse_pos, rect);
    let color = if collision {
        Color(150, 60, 60, 1.)
    } else {
        Color(50, 50, 50, 1.)
    };

    if collision {
        rect_border(rect, border, Color(150, 160, 160, 1.));
    }

    render_rect(
        bounds.min.x - padding,
        bounds.min.y,
        bounds.width() + padding * 2.,
        bounds.height(),
        color,
    );
}
