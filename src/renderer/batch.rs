use nalgebra::{Matrix4, Point3, Vector, Vector2, Vector3};

use crate::renderer::{get_anchor_point, Vertex};

use super::{Anchor, Renderer, Texture, MAX_BATCH_SIZE};

impl Renderer {
    pub fn flush_batch(&mut self) {
        if !self.verticies.is_empty() {
            self.context.render(
                &self.verticies,
                &self.indicies,
                &self.bound_texture_map,
                &self.camera.get_projection_view_matrix(self.render_size),
            );
        }

        self.render_all_text_queue().unwrap();

        self.verticies.clear();
        self.indicies.clear();
        self.bound_texture_map.clear();
        self.bound_shader = None;
    }

    fn check_batch_overflow(&mut self) {
        let max_vertex_amount = 4 * MAX_BATCH_SIZE as usize;
        let max_indicies_amount = 6 * MAX_BATCH_SIZE as usize;

        let new_verticies_len = self.verticies.len() + 4;
        let new_indicies_len = self.indicies.len() + 6;

        if new_verticies_len > max_vertex_amount
            || new_indicies_len > max_indicies_amount
            || self.bound_texture_map.len() > self.context.max_texture_units as usize
        {
            self.flush_batch();
        }
    }

    pub(crate) fn push_rect(
        &mut self,
        position: Vector3<f32>,
        size: Vector2<f32>,
        scale: Vector3<f32>,
        rotation: Vector3<f32>,
        color: [f32; 4],
        anchor: Anchor,
        texture: Option<&Texture>,
    ) {
        self.check_batch_overflow();
        self.set_batch_id("rect");
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

        self.indicies.append(&mut new_indicies);

        let (top_right, bottom_right, bottom_left, top_left) = {
            let anchor_point = get_anchor_point(anchor, size);
            let anchor = Point3::from([anchor_point.x * scale.x, anchor_point.y * scale.y, 0.0]);

            let model = Matrix4::new_translation(&Vector::from([
                position.x - anchor_point.x,
                position.y - anchor_point.y,
                1.,
            ])) * Matrix4::new_rotation_wrt_point(rotation, anchor)
                * Matrix4::new_nonuniform_scaling(&Vector::from([
                    size.x * scale.x,
                    size.y * scale.y,
                    1.0,
                ]));

            (
                model * Vector::from([1., 1., 0., 1.]),
                model * Vector::from([1., 0., 0., 1.]),
                model * Vector::from([0., 0., 0., 1.]),
                model * Vector::from([0., 1., 0., 1.]),
            )
        };

        let tex_coords = if let Some(texture) = texture {
            texture.get_tex_coords()
        } else {
            [[1., 1.], [1., 0.], [0., 0.], [0., 1.]]
        };

        let tex_index: i32 = {
            let texture = if let Some(texture) = texture {
                *texture
            } else {
                self.default_texture
            };

            let match_index = self
                .bound_texture_map
                .iter()
                .position(|t| t.id == texture.id);

            if let Some(index) = match_index {
                index as _
            } else {
                let index = self.bound_texture_map.len();
                self.bound_texture_map.push(texture);
                index as _
            }
        };

        let mut new_verticies = vec![
            Vertex {
                position: top_right,
                color,
                tex_coords: tex_coords[0],
                tex_index,
            },
            Vertex {
                position: bottom_right,
                color,
                tex_coords: tex_coords[1],
                tex_index,
            },
            Vertex {
                position: bottom_left,
                color,
                tex_coords: tex_coords[2],
                tex_index,
            },
            Vertex {
                position: top_left,
                color,
                tex_coords: tex_coords[3],
                tex_index,
            },
        ];

        self.verticies.append(&mut new_verticies);
    }
}
