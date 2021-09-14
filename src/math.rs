use crate::math::Vector2;
use glyph_brush::ab_glyph::Rect;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub fn add_padding(&mut self, expand_by: Vector2<f32>) {
        self.x -= expand_by.x;
        self.y -= expand_by.y;
        self.w += expand_by.x * 2.;
        self.h += expand_by.y * 2.;
    }
}

impl From<Rect> for Rectangle {
    fn from(item: Rect) -> Self {
        Rectangle {
            x: item.min.x,
            y: item.min.y,
            width: item.width(),
            height: item.height(),
        }
    }
}

pub fn clamp(min: f32, max: f32, value: f32) -> f32 {
    f32::min(f32::max(min, value), max)
}
