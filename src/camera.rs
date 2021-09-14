use nalgebra::{Matrix4, Vector2, Vector3};

use crate::glm::{ortho_lh, translate};

#[derive(Default)]
pub struct Camera {
    pub zoom: f32,
    pub projection_type: Projection,
    pub position: Vector3<f32>,
}

pub enum Projection {
    Orthographic,
    Perspective,
}

impl Default for Projection {
    fn default() -> Self {
        Self::Orthographic
    }
}

impl Camera {}

impl Camera {
    pub fn set_projection(&mut self, projection_type: Projection) {
        self.projection_type = projection_type;
    }

    pub(crate) fn get_projection_matrix(&self, window_size: Vector2<i32>) -> Matrix4<f32> {
        ortho_lh(
            -(window_size.x as f32 / 2.),
            window_size.x as f32 / 2.,
            window_size.y as f32 / 2.,
            -(window_size.y as f32 / 2.),
            -1.0,
            1.0,
        )
    }

    pub(crate) fn get_projection_view_matrix(&self, window_size: Vector2<i32>) -> Matrix4<f32> {
        let proj = self.get_projection_matrix(window_size);

        let view = translate(&Matrix4::identity(), &self.position)
            .try_inverse()
            .unwrap();

        proj * view
    }
}
