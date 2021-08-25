use crate::math::{Matrix, Vec2};

pub struct Camera {
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera { zoom: 1. }
    }
}

impl Camera {
    pub fn get_projection(&self, window_size: Vec2) -> Matrix {
        let mut proj = Matrix::ortho(0.0, window_size.x, window_size.y, 0.0, -100.0, 100.0);
        proj.scale(Vec2::new(1., 1.) * self.zoom);

        proj
    }
}
