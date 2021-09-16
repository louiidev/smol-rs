use nalgebra::{Vector, Vector2};

use crate::{renderer::Vertex, Color, Transform};

use super::Renderer;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Renderer {
    pub fn rectangle_transform(&mut self, transform: Transform, color: Color) {
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

    pub fn rectangle(&mut self, rect: Rectangle, color: Color) {
        let verticies_amount: i32 = self.verticies.len() as _;

        let mut new_indicies: Vec<i32> = vec![
            // first tri
            verticies_amount + 0,
            verticies_amount + 1,
            verticies_amount + 3,
            // secound tri
            verticies_amount + 1,
            verticies_amount + 2,
            verticies_amount + 3,
        ];

        let color = color.normalize();

        self.indicies.append(&mut new_indicies);

        let tex_coords = [[1., 1.], [1., 0.], [0., 0.], [0., 1.]];
        let tex_index = self.get_texture_index(None);

        let mut new_verticies = vec![
            Vertex {
                position: Vector::from([rect.x + rect.width, rect.y + rect.height, 0., 1.]),
                color,
                tex_coords: tex_coords[0],
                tex_index,
            },
            Vertex {
                position: Vector::from([rect.x + rect.width, rect.y, 0., 1.]),
                color,
                tex_coords: tex_coords[1],
                tex_index,
            },
            Vertex {
                position: Vector::from([rect.x, rect.y, 0., 1.]),
                color,
                tex_coords: tex_coords[2],
                tex_index,
            },
            Vertex {
                position: Vector::from([rect.x, rect.y + rect.height, 0., 1.]),
                color,
                tex_coords: tex_coords[3],
                tex_index,
            },
        ];

        self.verticies.append(&mut new_verticies);
    }

    pub fn line_rect(&mut self, rect: Rectangle, thickness: f32, color: Color) {
        let top_left = Vector::from([rect.x, rect.y]);
        let bottom_left = Vector::from([rect.x, rect.y + rect.height]);
        let bottom_right = Vector::from([rect.x + rect.width, rect.y + rect.height]);
        let top_right = Vector::from([rect.x + rect.width, rect.y]);

        self.line(top_left, bottom_left, thickness, color);

        self.line(bottom_left, bottom_right, thickness, color);

        self.line(bottom_right, top_right, thickness, color);

        self.line(top_right, top_left, thickness, color);
    }

    pub fn circle(&mut self, center: Vector2<f32>, radius: f32, steps: i32, color: Color) {
        let mut last = Vector::from([center.x + radius, center.y]);
        use std::f32::consts::TAU;
        for i in 1..steps + 1 {
            let radians = (i as f32 / steps as f32) * TAU;
            let next = Vector::from([
                center.x + radians.cos() * radius,
                center.y + radians.sin() * radius,
            ]);

            self.triangle(last, next, center, color);

            last = next;
        }
    }

    pub fn triangle(
        &mut self,
        pos0: Vector2<f32>,
        pos1: Vector2<f32>,
        pos2: Vector2<f32>,
        color: Color,
    ) {
        let verticies_amount: i32 = self.verticies.len() as _;

        let mut new_indicies: Vec<i32> = vec![
            // first tri
            verticies_amount + 0,
            verticies_amount + 1,
            verticies_amount + 2,
        ];

        let color = color.normalize();

        self.indicies.append(&mut new_indicies);

        let tex_coords = [[1., 1.], [1., 0.], [0., 0.], [0., 1.]];
        let tex_index = self.get_texture_index(None);

        let mut new_verticies = vec![
            Vertex {
                position: Vector::from([pos0.x, pos0.y, 0., 1.]),
                color,
                tex_coords: tex_coords[0],
                tex_index,
            },
            Vertex {
                position: Vector::from([pos1.x, pos1.y, 0., 1.]),
                color,
                tex_coords: tex_coords[1],
                tex_index,
            },
            Vertex {
                position: Vector::from([pos2.x, pos2.y, 0., 1.]),
                color,
                tex_coords: tex_coords[2],
                tex_index,
            },
        ];

        self.verticies.append(&mut new_verticies);
    }

    pub fn line(&mut self, from: Vector2<f32>, to: Vector2<f32>, thickness: f32, color: Color) {
        let normal = (to - from).normalize();
        let perp = Vector::from([normal.y, -normal.x]);
        let pos0 = from + perp * thickness * 0.5;
        let pos1 = to + perp * thickness * 0.5;
        let pos2 = to - perp * thickness * 0.5;
        let pos3 = from - perp * thickness * 0.5;

        let verticies_amount: i32 = self.verticies.len() as _;

        let mut new_indicies: Vec<i32> = vec![
            // first tri
            verticies_amount + 0,
            verticies_amount + 1,
            verticies_amount + 3,
            // secound tri
            verticies_amount + 1,
            verticies_amount + 2,
            verticies_amount + 3,
        ];

        let color = color.normalize();

        self.indicies.append(&mut new_indicies);

        let tex_coords = [[1., 1.], [1., 0.], [0., 0.], [0., 1.]];
        let tex_index = self.get_texture_index(None);

        let mut new_verticies = vec![
            Vertex {
                position: Vector::from([pos0.x, pos0.y, 0., 1.]),
                color,
                tex_coords: tex_coords[0],
                tex_index,
            },
            Vertex {
                position: Vector::from([pos1.x, pos1.y, 0., 1.]),
                color,
                tex_coords: tex_coords[1],
                tex_index,
            },
            Vertex {
                position: Vector::from([pos2.x, pos2.y, 0., 1.]),
                color,
                tex_coords: tex_coords[2],
                tex_index,
            },
            Vertex {
                position: Vector::from([pos3.x, pos3.y, 0., 1.]),
                color,
                tex_coords: tex_coords[3],
                tex_index,
            },
        ];

        self.verticies.append(&mut new_verticies);
    }
}
