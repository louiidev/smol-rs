use ab_glyph::{point, Rect};
use glyph_brush::{BrushAction, HorizontalAlign, Layout, Section, Text, VerticalAlign};
use nalgebra::Vector2;

use crate::errors::SmolError;
use crate::Color;

use super::{Font, Renderer};

pub type TextAlignment = HorizontalAlign;

pub struct TextSettings {
    pub color: Color,
    pub alignment: TextAlignment,
    pub position: Vector2<f32>,
}

impl Default for TextSettings {
    fn default() -> Self {
        TextSettings {
            color: Color::BLACK,
            alignment: TextAlignment::Left,
            position: Vector2::default(),
        }
    }
}

impl Renderer {
    pub fn text(&mut self, font: &Font, text: &str, position: Vector2<f32>) {
        self.text_ex(
            font,
            text,
            &TextSettings {
                color: Color::BLACK,
                alignment: TextAlignment::Left,
                position,
            },
        )
    }

    pub fn text_ex(&mut self, font: &Font, text: &str, settings: &TextSettings) {
        let brush = self.glyph_brushs.get_mut(font).unwrap();
        let base_text = Text::new(&text);
        let section = Section::default()
            .add_text(
                base_text
                    .with_scale(40.)
                    .with_color(settings.color.normalize()),
            )
            .with_screen_position((settings.position.x, settings.position.y))
            .with_layout(
                Layout::default()
                    .h_align(settings.alignment)
                    .v_align(VerticalAlign::Top),
            );
        brush.queue(section);
        self.set_batch_id(&format!("font_{}", font.id));
    }

    pub(crate) fn render_all_text_queue(&mut self) -> Result<(), SmolError> {
        // let brush_map: HashMap<Font, GlyphBrush<[f32; 13]>> = .iter().collect();
        for (font, brush) in &mut self.glyph_brushs {
            self.context.bind_texture(&font.texture);
            let brush_action = brush.process_queued(
                |rect, tex_data| unsafe {
                    gl::TexSubImage2D(
                        gl::TEXTURE_2D,
                        0,
                        rect.min[0] as _,
                        rect.min[1] as _,
                        rect.width() as _,
                        rect.height() as _,
                        gl::RED,
                        gl::UNSIGNED_BYTE,
                        tex_data.as_ptr() as _,
                    );
                },
                to_vertex,
            )?;

            match brush_action {
                BrushAction::Draw(vertices) => self.context.text_pipeline.upload_vertices(vertices),
                BrushAction::ReDraw => {}
            }
            self.context
                .text_pipeline
                .flush(&self.camera.get_projection_matrix(self.render_size));
        }

        Ok(())
    }
}

#[inline]
pub fn to_vertex(
    glyph_brush::GlyphVertex {
        mut tex_coords,
        pixel_coords,
        bounds,
        extra,
    }: glyph_brush::GlyphVertex,
) -> [f32; 13] {
    let gl_bounds = bounds;

    let mut gl_rect = Rect {
        min: point(pixel_coords.min.x as f32, pixel_coords.min.y as f32),
        max: point(pixel_coords.max.x as f32, pixel_coords.max.y as f32),
    };

    // handle overlapping bounds, modify uv_rect to preserve texture aspect
    if gl_rect.max.x > gl_bounds.max.x {
        let old_width = gl_rect.width();
        gl_rect.max.x = gl_bounds.max.x;
        tex_coords.max.x = tex_coords.min.x + tex_coords.width() * gl_rect.width() / old_width;
    }
    if gl_rect.min.x < gl_bounds.min.x {
        let old_width = gl_rect.width();
        gl_rect.min.x = gl_bounds.min.x;
        tex_coords.min.x = tex_coords.max.x - tex_coords.width() * gl_rect.width() / old_width;
    }
    if gl_rect.max.y > gl_bounds.max.y {
        let old_height = gl_rect.height();
        gl_rect.max.y = gl_bounds.max.y;
        tex_coords.max.y = tex_coords.min.y + tex_coords.height() * gl_rect.height() / old_height;
    }
    if gl_rect.min.y < gl_bounds.min.y {
        let old_height = gl_rect.height();
        gl_rect.min.y = gl_bounds.min.y;
        tex_coords.min.y = tex_coords.max.y - tex_coords.height() * gl_rect.height() / old_height;
    }

    [
        gl_rect.min.x,
        gl_rect.max.y,
        extra.z,
        gl_rect.max.x,
        gl_rect.min.y,
        tex_coords.min.x,
        tex_coords.max.y,
        tex_coords.max.x,
        tex_coords.min.y,
        extra.color[0],
        extra.color[1],
        extra.color[2],
        extra.color[3],
    ]
}
