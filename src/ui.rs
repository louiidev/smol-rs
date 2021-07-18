use glyph_brush::ab_glyph::Rect;

use crate::{collision::is_point_inside_rect, core::{queue_text, render_rect}, input::get_mouse_pos, math::Vector2, render::Color};





pub fn context_menu(position: Vector2) {
    let mut last_bounds: Option<Rect> = None;
    let current_mouse_pos = get_mouse_pos();
    for i in 1..4 {
        let padding: f32 = 10.;
        let tooltip_pos = {
            if let Some(bounds) = last_bounds  {
                Vector2::new(position.x + padding, bounds.max.y)
            } else {
                Vector2::new(position.x + padding, position.y)
            }
        };
        let bounds = queue_text(&format!("item {}", i), tooltip_pos, 14., Color(255, 255, 255, 1.)).unwrap();
        let collision = is_point_inside_rect(
            current_mouse_pos,   
            (bounds.min.x - padding) as _,
            bounds.min.y as _,
            (bounds.width() + padding * 2.) as _,
            bounds.height() as _,
        );
        let color = if collision {
            Color(150, 60, 60, 1.)
        } else {
            Color(50, 50, 50, 1.)
        };
        render_rect(bounds.min.x - padding, bounds.min.y, bounds.width() + padding * 2., bounds.height(), color);
        last_bounds = Some(bounds);
    }
}