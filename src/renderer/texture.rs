use std::hash::{Hash, Hasher};

use nalgebra::Vector;

use crate::gfx::GfxContext;
use crate::math::Vector2;

#[derive(Clone, Copy, Default, Debug)]
pub struct Font {
    pub(crate) id: usize,
    pub(crate) texture: Texture,
}

impl PartialEq for Font {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Font {}

impl Hash for Font {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Texture {
    pub(crate) id: u32,
    pub uv_size: Vector2<f32>,
    uv_position: Vector2<f32>,
    texture_size: Vector2<f32>,
}

impl Texture {
    pub(crate) fn new(
        id: u32,
        uv_size: Vector2<f32>,
        uv_position: Vector2<f32>,
        texture_size: Vector2<f32>,
    ) -> Self {
        Self {
            id,
            uv_size,
            uv_position,
            texture_size,
        }
    }

    pub(crate) fn get_tex_coords(&self) -> [[f32; 2]; 4] {
        let max = Vector::from([
            (self.uv_position.x + self.uv_size.x) / self.texture_size.x,
            (self.uv_position.y + self.uv_size.y) / self.texture_size.y,
        ]);

        let min = Vector::from([
            self.uv_position.x / self.texture_size.x,
            self.uv_position.y / self.texture_size.y,
        ]);

        [
            [max.x, max.y], // top-right
            [max.x, min.y], //  bottom_right
            [min.x, min.y], //  bottom_left
            [min.x, max.y], // top_left
        ]
    }

    pub(crate) fn default() -> Self {
        let white_texture: u32 = 0xffffffff;
        let default_texture_id = GfxContext::generate_empty_texture(
            1,
            1,
            &white_texture as *const _ as *const std::ffi::c_void,
        );

        let size = Vector::from([1., 1.]);

        Self::new(default_texture_id, size, Vector2::default(), size)
    }
}
