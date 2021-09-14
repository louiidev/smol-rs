use nalgebra::Vector;

use crate::{Color, Transform};

use super::Renderer;

impl Renderer {
    pub fn rect(&mut self, transform: Transform, color: Color) {
        self.push_rect(
            transform.position,
            Vector::from([transform.scale.x, transform.scale.y]),
            Vector::from([1., 1., 0.]),
            transform.rotation,
            color.normalize(),
            transform.anchor,
            None,
        );
    }
}
