//! Example of glyph_brush usage with raw OpenGL.
//!
//! Uses instanced rendering with 1 vertex per glyph referencing a 1 byte per pixel texture.
//!
//! Controls
//! * Scroll to size text.
//! * Type to modify text.
//! * Resize window.

use gl::types::*;
use glyph_brush::{ab_glyph::*, *};
use std::{ffi::CString, mem, ptr, str};

macro_rules! gl_assert_ok {
    () => {{
        let err = gl::GetError();
        assert_eq!(err, gl::NO_ERROR, "{}", gl_err_to_str(err));
    }};
}

pub type Vertex = [GLfloat; 13];

pub struct TextRenderer {
    pub text_pipe: GlTextPipe,
    texture: GlGlyphTexture,
    glyph_brush: GlyphBrush<Vertex>,
    max_image_dimension: u32,
}

pub type TextAlignment = HorizontalAlign;

#[derive(Debug)]
pub struct TextQueueConfig {
    pub horizontal_alginment: TextAlignment,
    pub font_size: f32,
    pub position: Vec2,
    pub color: Color,
}

impl Default for TextQueueConfig {
    fn default() -> Self {
        Self {
            horizontal_alginment: TextAlignment::Left,
            font_size: 16.,
            position: Vec2::default(),
            color: Color(0, 0, 0, 1.),
        }
    }
}

impl TextRenderer {
    pub fn new(x: i32, y: i32) -> TextRenderer {
        // let dejavu = FontRef::try_from_slice(include_bytes!("../assets/OpenSans-Light.ttf")).unwrap();
        let dejavu = FontArc::try_from_slice(include_bytes!("../assets/ThaleahFat.ttf")).unwrap();
        let glyph_brush: GlyphBrush<Vertex> = GlyphBrushBuilder::using_font(dejavu).build();
        let texture = GlGlyphTexture::new(glyph_brush.texture_dimensions());
        let text_pipe = GlTextPipe::new(Vec2Int { x, y }).unwrap();

        let max_image_dimension = {
            let mut value = 0;
            unsafe { gl::GetIntegerv(gl::MAX_TEXTURE_SIZE, &mut value) };
            value as u32
        };

        TextRenderer {
            text_pipe,
            texture,
            glyph_brush,
            max_image_dimension,
        }
    }

    pub fn queue_multiple(
        &mut self,
        text: Vec<(String, Color)>,
        position: Vec2,
        font_size: f32,
    ) -> Option<Rect> {
        let scale = (font_size * get_window_scale().x).round();
        let mut section = Section::default()
            .with_screen_position((position.x, position.y))
            .with_layout(
                Layout::default()
                    .h_align(HorizontalAlign::Left)
                    .v_align(VerticalAlign::Top),
            );
        let text_group: Vec<Text> = text
            .iter()
            .map(|(t, color)| {
                let (r, g, b, a) = color.into_gl();
                let base_text = Text::new(&t).with_scale(scale).with_color([r, g, b, a]);
                base_text
            })
            .collect();

        for base_text in text_group {
            section = section.add_text(base_text);
        }

        let bounds = self.glyph_brush.glyph_bounds(&section);

        self.glyph_brush.queue(section);

        bounds
    }

    pub fn get_text_bounds(&mut self, text: &str, text_config: TextQueueConfig) -> Option<Rect> {
        let scale = (text_config.font_size * get_window_scale().x).round();
        let base_text = Text::new(text).with_scale(scale);

        let section = Section::default()
            .add_text(base_text)
            .with_screen_position((text_config.position.x, text_config.position.y))
            .with_layout(
                Layout::default()
                    .h_align(text_config.horizontal_alginment)
                    .v_align(VerticalAlign::Top),
            );

        self.glyph_brush.glyph_bounds(&section)
    }

    pub fn queue_text_ex(&mut self, text: &str, text_config: TextQueueConfig) -> Option<Rect> {
        let scale = (text_config.font_size * get_window_scale().x).round();
        let base_text = Text::new(text).with_scale(scale);
        let (r, g, b, a) = text_config.color.into_gl();
        let section = Section::default()
            .add_text(base_text.with_color([r, g, b, a]))
            .with_screen_position((text_config.position.x, text_config.position.y))
            .with_layout(
                Layout::default()
                    .h_align(text_config.horizontal_alginment)
                    .v_align(VerticalAlign::Top),
            );

        let bounds = self.glyph_brush.glyph_bounds(&section);

        self.glyph_brush.queue(section);

        bounds
    }

    pub fn on_resize_window(&self, window_size: Vec2Int) {
        self.text_pipe.update_geometry(window_size);
    }

    pub fn render_queue(&mut self) {
        let texture_name = self.texture.name;

        let mut brush_action;
        loop {
            brush_action = self.glyph_brush.process_queued(
                |rect, tex_data| unsafe {
                    // Update part of gpu texture with new glyph alpha values
                    gl::BindTexture(gl::TEXTURE_2D, texture_name);

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
                    gl_assert_ok!();
                },
                to_vertex,
            );
            match brush_action {
                Ok(_) => break,
                Err(BrushError::TextureTooSmall { suggested, .. }) => {
                    let (new_width, new_height) = if (suggested.0 > self.max_image_dimension
                        || suggested.1 > self.max_image_dimension)
                        && (self.glyph_brush.texture_dimensions().0 < self.max_image_dimension
                            || self.glyph_brush.texture_dimensions().1 < self.max_image_dimension)
                    {
                        (self.max_image_dimension, self.max_image_dimension)
                    } else {
                        suggested
                    };
                    eprint!("\r                            \r");
                    eprintln!("Resizing glyph texture -> {}x{}", new_width, new_height);

                    // Recreate texture as a larger size to fit more
                    self.texture = GlGlyphTexture::new((new_width, new_height));

                    self.glyph_brush.resize_texture(new_width, new_height);
                }
            }
        }
        match brush_action.unwrap() {
            BrushAction::Draw(vertices) => self.text_pipe.upload_vertices(&vertices),
            BrushAction::ReDraw => {}
        }
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture_name);
        }
        self.text_pipe.draw();
    }
}

use crate::{
    core::get_window_scale,
    math::{Matrix, Vec2, Vec2Int, Vec3},
    render::{get_uniform_location, Color},
};

pub type Res<T> = Result<T, Box<dyn std::error::Error>>;
/// `[left_top * 3, right_bottom * 2, tex_left_top * 2, tex_right_bottom * 2, color * 4]`

pub fn gl_err_to_str(err: u32) -> &'static str {
    match err {
        gl::INVALID_ENUM => "INVALID_ENUM",
        gl::INVALID_VALUE => "INVALID_VALUE",
        gl::INVALID_OPERATION => "INVALID_OPERATION",
        gl::INVALID_FRAMEBUFFER_OPERATION => "INVALID_FRAMEBUFFER_OPERATION",
        gl::OUT_OF_MEMORY => "OUT_OF_MEMORY",
        gl::STACK_UNDERFLOW => "STACK_UNDERFLOW",
        gl::STACK_OVERFLOW => "STACK_OVERFLOW",
        _ => "Unknown error",
    }
}

pub fn compile_shader(src: &str, ty: GLenum) -> Res<GLuint> {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        // Attempt to compile the shader
        let c_str = CString::new(src.as_bytes())?;
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = GLint::from(gl::FALSE);
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        // Fail on error
        if status != GLint::from(gl::TRUE) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            return Err(str::from_utf8(&buf)?.into());
        }
    }
    Ok(shader)
}

pub fn link_program(vs: GLuint, fs: GLuint) -> Res<GLuint> {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = GLint::from(gl::FALSE);
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != GLint::from(gl::TRUE) {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
            return Err(str::from_utf8(&buf)?.into());
        }
        Ok(program)
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
) -> Vertex {
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

#[rustfmt::skip]
pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> [f32; 16] {
    let tx = -(right + left) / (right - left);
    let ty = -(top + bottom) / (top - bottom);
    let tz = -(far + near) / (far - near);
    [
        2.0 / (right - left), 0.0, 0.0, 0.0,
        0.0, 2.0 / (top - bottom), 0.0, 0.0,
        0.0, 0.0, -2.0 / (far - near), 0.0,
        tx, ty, tz, 1.0,
    ]
}

pub struct GlGlyphTexture {
    pub name: GLuint,
}

impl GlGlyphTexture {
    pub fn new((width, height): (u32, u32)) -> Self {
        let mut name = 0;
        unsafe {
            // Create a texture for the glyphs
            // The texture holds 1 byte per pixel as alpha data
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
            gl::GenTextures(1, &mut name);
            gl::BindTexture(gl::TEXTURE_2D, name);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as _);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as _);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RED as _,
                width as _,
                height as _,
                0,
                gl::RED,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );
            gl_assert_ok!();

            Self { name }
        }
    }
}

impl Drop for GlGlyphTexture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.name);
        }
    }
}

pub struct GlTextPipe {
    shaders: [GLuint; 2],
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    transform_uniform: GLint,
    vertex_count: usize,
    vertex_buffer_len: usize,
}

impl GlTextPipe {
    pub fn new(window_size: Vec2Int) -> Res<Self> {
        let (w, h) = (window_size.x as f32, window_size.y as f32);

        let vs = compile_shader(include_str!("shader/text.vs"), gl::VERTEX_SHADER)?;
        let fs = compile_shader(include_str!("shader/text.fs"), gl::FRAGMENT_SHADER)?;
        let program = link_program(vs, fs)?;

        let mut vao = 0;
        let mut vbo = 0;

        let transform_uniform = unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Use shader program
            gl::UseProgram(program);
            gl::BindFragDataLocation(program, 0, CString::new("out_color")?.as_ptr());

            // Specify the layout of the vertex data
            let uniform = gl::GetUniformLocation(program, CString::new("projection")?.as_ptr());
            if uniform < 0 {
                return Err(format!("GetUniformLocation(\"projection\") -> {}", uniform).into());
            }
            let projection: [f32; 16] = Matrix::ortho(0.0, w, 0.0, h, -100.0, 100.0).into();
            let offset_location = get_uniform_location(program, "offset");

            gl::Uniform2f(offset_location, 0., 0.);
            gl::UniformMatrix4fv(uniform, 1, 0, projection.as_ptr());

            let mut offset = 0;
            for (v_field, float_count) in &[
                ("left_top", 3),
                ("right_bottom", 2),
                ("tex_left_top", 2),
                ("tex_right_bottom", 2),
                ("color", 4),
            ] {
                let attr = gl::GetAttribLocation(program, CString::new(*v_field)?.as_ptr());
                if attr < 0 {
                    return Err(format!("{} GetAttribLocation -> {}", v_field, attr).into());
                }
                gl::VertexAttribPointer(
                    attr as _,
                    *float_count,
                    gl::FLOAT,
                    gl::FALSE as _,
                    mem::size_of::<Vertex>() as _,
                    offset as _,
                );
                gl::EnableVertexAttribArray(attr as _);
                gl::VertexAttribDivisor(attr as _, 1);

                offset += float_count * 4;
            }

            // Enabled alpha blending
            //gl::Enable(gl::BLEND);
            //gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            // Use srgb for consistency with other examples
            // gl::Enable(gl::FRAMEBUFFER_SRGB);
            gl::UseProgram(0);
            gl_assert_ok!();

            uniform
        };

        Ok(Self {
            shaders: [vs, fs],
            program,
            vao,
            vbo,
            transform_uniform,
            vertex_count: 0,
            vertex_buffer_len: 0,
        })
    }

    pub fn set_offset(&self, offset: Vec2) {
        unsafe {
            gl::UseProgram(self.program);
            let view = Matrix::translate(Vec3 {
                x: offset.x,
                y: offset.y,
                z: 0.0,
            });

            let float_view: [f32; 16] = view.into();
            gl::UniformMatrix4fv(
                get_uniform_location(self.program, "view"),
                1,
                gl::FALSE,
                float_view.as_ptr(),
            );
            gl::UseProgram(0);
        }
    }

    pub fn reset_offset(&self) {
        unsafe {
            gl::UseProgram(self.program);
            let view = Matrix::translate(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            });

            let float_view: [f32; 16] = view.into();
            gl::UniformMatrix4fv(
                get_uniform_location(self.program, "view"),
                1,
                gl::FALSE,
                float_view.as_ptr(),
            );
            gl::UseProgram(0);
        }
    }

    pub fn upload_vertices(&mut self, vertices: &[Vertex]) {
        // Draw new vertices
        self.vertex_count = vertices.len();

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            if self.vertex_buffer_len < self.vertex_count {
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (self.vertex_count * mem::size_of::<Vertex>()) as GLsizeiptr,
                    vertices.as_ptr() as _,
                    gl::DYNAMIC_DRAW,
                );
                self.vertex_buffer_len = self.vertex_count;
            } else {
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    0,
                    (self.vertex_count * mem::size_of::<Vertex>()) as GLsizeiptr,
                    vertices.as_ptr() as _,
                );
            }
            gl_assert_ok!();
        }
    }

    pub fn update_geometry(&self, window_size: Vec2Int) {
        let (w, h) = (window_size.x as f32, window_size.y as f32);
        let projection: [f32; 16] = Matrix::ortho(0.0, w, 0.0, h, -100.0, 100.0).into();

        unsafe {
            gl::UseProgram(self.program);
            gl::UniformMatrix4fv(self.transform_uniform, 1, 0, projection.as_ptr());
            gl::UseProgram(0);
            gl_assert_ok!();
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::UseProgram(self.program);

            gl::BindVertexArray(self.vao);
            gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, self.vertex_count as _);
            gl::UseProgram(0);
            gl_assert_ok!();
        }
    }
}

impl Drop for GlTextPipe {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            self.shaders.iter().for_each(|s| gl::DeleteShader(*s));
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
