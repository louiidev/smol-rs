use crate::gfx::GfxContext;
use crate::{camera::Camera, transform::Transform};
use glyph_brush::GlyphBrush;
use hashbrown::HashMap;
use sdl2::video::Window;

use crate::color::Color;

use nalgebra::{Vector, Vector2, Vector4};

use crate::renderer::texture::{Font, Texture};

use super::shader::Shader;

pub(crate) struct Vertex {
    pub position: Vector4<f32>,
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
    pub tex_index: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Anchor {
    Center,
    TopLeft,
    Custom(Vector2<f32>),
}

impl Default for Anchor {
    fn default() -> Self {
        Anchor::Center
    }
}

#[derive(Default)]
pub struct Renderer {
    pub(crate) context: GfxContext,
    pub(crate) verticies: Vec<Vertex>,
    pub(crate) indicies: Vec<i32>,
    pub(crate) bound_texture_map: Vec<Texture>,
    pub(crate) default_texture: Texture,
    pub(crate) bound_shader: Option<Shader>,
    pub(crate) current_batch_id: Option<String>,
    pub camera: Camera,
    pub(crate) glyph_brushs: HashMap<Font, GlyphBrush<[f32; 13]>>,
    pub(crate) render_size: Vector2<i32>,
}

impl Renderer {
    pub fn new(render_size: Vector2<i32>) -> Self {
        let context = GfxContext::new(render_size);
        let camera = Camera::default();

        Renderer {
            camera,
            context,
            render_size,
            default_texture: Texture::default(),
            ..Default::default()
        }
    }

    pub fn clear(&self, color: Color) {
        self.context.clear_buffer(color);
    }

    pub fn set_shader(&mut self, shader: Shader) {
        if let Some(bound_shader) = self.bound_shader {
            if bound_shader != shader {
                self.flush_batch();
            }
        }

        self.bound_shader = Some(shader);
    }

    pub fn set_batch_id(&mut self, id: &str) {
        if let Some(batch_id) = &self.current_batch_id {
            if batch_id != id {
                self.flush_batch();
            }
        }

        self.current_batch_id = Some(id.into());
    }

    pub fn render(&mut self) {
        self.flush_batch();
    }

    pub fn texture(&mut self, transform: Transform, texture: &Texture) {
        self.push_rect(
            transform.position,
            texture.uv_size,
            transform.scale,
            transform.rotation,
            Color::WHITE.normalize(),
            transform.anchor,
            Some(texture),
        )
    }

    pub fn swap_buffer(&mut self, window: &Window) {
        self.context.swap_buffer(&window);
    }
}

pub(crate) fn get_anchor_point(anchor: Anchor, size: Vector2<f32>) -> Vector2<f32> {
    match anchor {
        Anchor::Center => size * 0.5,
        Anchor::TopLeft => Vector::from([0., 0.]),
        Anchor::Custom(pos) => pos,
    }
}
