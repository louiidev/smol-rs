use image::DynamicImage;
use std::ffi::c_void;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

#[cfg(feature = "opengl")]
use crate::opengl::GfxContext;

use crate::render::Color;
#[cfg(feature = "vulkan")]
use crate::vulkan::GfxContext;

use crate::math::*;

const MAX_BATCH_SIZE: i32 = 1000;

pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
    pub tex_coords: [f32; 2],
}

pub struct Renderer {
    context: GfxContext,
    verticies: Vec<Vertex>,
    indicies: Vec<i32>,
}

pub enum Anchor {
    Center,
    TopLeft,
}

pub struct Texture {
    pub size: Vec2,
    atlas_position: Vec2,
    atlas_size: Vec2,
}

impl Texture {
    pub fn get_tex_coords(&self) -> [[f32; 2]; 4] {
        [[1., 1.], [1., 1.], [1., 1.], [1., 1.]]
    }
}

pub struct DrawOptions {
    anchor: Anchor,
    color: Color,
    size: Vec2,
    position: Vec2,
}

impl Default for DrawOptions {
    fn default() -> Self {
        Self {
            anchor: Anchor::Center,
            color: Color::WHITE,
            size: Vec2::default(),
            position: Vec2::default(),
        }
    }
}

impl Renderer {
    pub fn new() -> Self {
        let context = GfxContext::new();

        Renderer {
            context,
            verticies: Vec::default(),
            indicies: Vec::default(),
        }
    }

    pub fn render(&mut self) {
        self.context.render(&self.verticies, &self.indicies);
        self.verticies.clear();
        self.indicies.clear();
    }

    pub fn rect(&mut self, options: DrawOptions) {
        self.push_rect(
            options.position,
            options.size,
            options.color.normalize(),
            options.anchor,
            None,
        );
    }

    pub fn texture(&mut self, options: DrawOptions, texture: Texture) {
        let size = if options.size != Vec2::default() {
            texture.size
        } else {
            options.size
        };

        self.push_rect(
            options.position,
            size,
            options.color.normalize(),
            options.anchor,
            Some(texture),
        )
    }

    fn push_rect(
        &mut self,
        position: Vec2,
        size: Vec2,
        color: [f32; 4],
        anchor: Anchor,
        texture: Option<Texture>,
    ) {
        let verticies_amount: i32 = self.verticies.len() as _;
        let new_verticies_size = verticies_amount / 4 + 1;

        if new_verticies_size > MAX_BATCH_SIZE {
            self.render();
        }

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
            let (x, y) = { (position.x, position.y) };
            let (width, height) = { (size.x, size.y) };

            match anchor {
                Anchor::Center => (
                    [x + (width / 2.), y + (height / 2.)],
                    [x + (width / 2.), y - (height / 2.)],
                    [x - (width / 2.), y - (height / 2.)],
                    [x - (width / 2.), y + (height / 2.)],
                ),
                Anchor::TopLeft => (
                    [x + width, y + height],
                    [x + width, y],
                    [x, y],
                    [x, y + height],
                ),
            }
        };

        let tex_coords = if let Some(texture) = texture {
            texture.get_tex_coords()
        } else {
            [[1., 1.], [1., 1.], [1., 1.], [1., 1.]]
        };

        let mut new_verticies = vec![
            Vertex {
                position: top_right,
                color,
                tex_coords: tex_coords[0],
            },
            Vertex {
                position: bottom_right,
                color,
                tex_coords: tex_coords[1],
            },
            Vertex {
                position: bottom_left,
                color,
                tex_coords: tex_coords[2],
            },
            Vertex {
                position: top_left,
                color,
                tex_coords: tex_coords[3],
            },
        ];

        self.verticies.append(&mut new_verticies);
    }
}
