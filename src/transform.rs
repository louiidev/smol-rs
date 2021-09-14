use nalgebra::{Vector, Vector2, Vector3};

use crate::renderer::Anchor;

#[derive(Debug, Clone, Copy)]
pub struct Transform {
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
    pub anchor: Anchor,
}

impl Transform {
    //  pub fn rect(x: f32, y: f32, width: f32, height: f32) -> Self {}
}

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            position: Vector3::default(),
            rotation: Vector3::default(),
            scale: Vector::from([1., 1., 1.]),
            anchor: Anchor::default(),
        }
    }
}

impl From<Vector2<f32>> for Transform {
    fn from(position: Vector2<f32>) -> Self {
        Transform {
            position: Vector::from([position.x, position.y, 0.]),
            ..Default::default()
        }
    }
}

impl From<Vector3<f32>> for Transform {
    fn from(position: Vector3<f32>) -> Self {
        Transform {
            position,
            ..Default::default()
        }
    }
}

impl From<Rect> for Transform {
    fn from(rect: Rect) -> Self {
        Transform {
            position: Vector::from([rect.x, rect.y, 0.]),
            scale: Vector::from([rect.width, rect.height, 0.]),
            ..Default::default()
        }
    }
}
