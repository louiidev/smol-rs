use std::ffi::c_void;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use gl::types;
use gl::types::GLchar;
use gl::types::GLint;
use gl::types::GLsizeiptr;
use gl::types::GLuint;
use image::DynamicImage;
use nalgebra::Vector2;
use sdl2::video::GLContext;
use sdl2::video::GLProfile;
use sdl2::video::SwapInterval;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::color::Color;
use crate::errors::SmolError;
use crate::renderer::core::Vertex;
use crate::renderer::Texture;
use crate::renderer::MAX_BATCH_SIZE;
use crate::AppSettings;

#[allow(unused_macros)]
macro_rules! gl_assert_ok {
    () => {{
        let err = gl::GetError();
        assert_eq!(err, gl::NO_ERROR, "{}", gl_err_to_str(err));
    }};
}

type TextureId = u32;

#[derive(Default)]
pub(crate) struct GfxContext {
    default_shader: u32,
    vao_id: u32,
    vertex_buffer_id: u32,
    index_buffer_id: u32,
    pub(crate) max_texture_units: i32,
    pub(crate) text_pipeline: GlTextPipeline,
}

impl GfxContext {
    pub fn new(render_size: Vector2<i32>) -> Self {
        let text_pipeline = GlTextPipeline::new();

        let fs = compile_shader(include_str!("../shaders/opengl/2d.fs"), gl::FRAGMENT_SHADER);
        let vs = compile_shader(include_str!("../shaders/opengl/2d.vs"), gl::VERTEX_SHADER);
        let default_shader = link_program(vs, fs);

        let mut vao_id = 0;
        let mut vertex_buffer_id: u32 = 0;
        let mut index_buffer_id: u32 = 0;
        let mut max_texture_units = 0;
        unsafe {
            gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut max_texture_units);
            gl::UseProgram(default_shader);

            gl::GenVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);

            gl::GenBuffers(1, &mut vertex_buffer_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_id);

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (4 * MAX_BATCH_SIZE as usize * mem::size_of::<Vertex>()) as GLsizeiptr,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            gl::GenBuffers(1, &mut index_buffer_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (6 * MAX_BATCH_SIZE as usize * mem::size_of::<i32>()) as isize,
                ptr::null(),
                gl::DYNAMIC_DRAW,
            );

            set_shader_attributes(
                default_shader,
                &[
                    Attribute::new_float("vertex", 4),
                    Attribute::new_float("color", 4),
                    Attribute::new_float("tex_coords", 2),
                    Attribute::new("tex_index", 1, gl::INT),
                ],
            )
            .unwrap();

            gl::Viewport(0, 0, render_size.x, render_size.y);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DeleteShader(fs);
            gl::DeleteShader(vs);
        }

        GfxContext {
            default_shader,
            vao_id,
            vertex_buffer_id,
            index_buffer_id,
            max_texture_units,
            text_pipeline,
        }
    }

    pub fn clear_buffer(&self, color: Color) {
        let [r, g, b, a] = color.normalize();
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn generate_empty_texture(width: i32, height: i32, img_ptr: *const c_void) -> TextureId {
        let mut texture_id = 0;
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                width,
                height,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img_ptr,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        };

        texture_id
    }

    pub fn bind_texture(&self, texture: &Texture) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
        }
    }

    pub fn generate_font_texture((width, height): (u32, u32)) -> TextureId {
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

            name
        }
    }

    pub fn generate_texture<'a>(bytes: &'a [u8]) -> (i32, i32, TextureId) {
        let mut texture_id = 0;

        let (width, height, img_data, internal_format, format) =
            match image::load_from_memory(bytes).expect("Could not load image at src: {}") {
                DynamicImage::ImageRgba8(_image) => (
                    _image.width() as i32,
                    _image.height() as i32,
                    _image.into_raw(),
                    gl::RGBA8,
                    gl::RGBA,
                ),
                DynamicImage::ImageRgb8(_image) => (
                    _image.width() as i32,
                    _image.height() as i32,
                    _image.into_raw(),
                    gl::RGB8,
                    gl::RGB,
                ),
                img => {
                    let _image = img.to_rgba8();
                    (
                        _image.width() as i32,
                        _image.height() as i32,
                        _image.into_raw(),
                        gl::RGBA8,
                        gl::RGBA,
                    )
                }
            };

        let img_ptr = img_data.as_ptr() as *const c_void;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internal_format as i32,
                width,
                height,
                0,
                format,
                gl::UNSIGNED_BYTE,
                img_ptr,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        };

        (width, height, texture_id)
    }

    pub(crate) fn render(
        &self,
        verticies: &Vec<Vertex>,
        indicies: &Vec<i32>,
        bound_texture_map: &Vec<Texture>,
        projection_view_matrix: &crate::glm::Mat4x4,
    ) {
        unsafe {
            gl::UseProgram(self.default_shader);

            for (index, texture) in bound_texture_map.iter().enumerate() {
                gl::ActiveTexture(gl::TEXTURE0 + index as gl::types::GLenum);
                self.bind_texture(&texture);
            }

            let loc = get_uniform_location(self.default_shader, "u_textures");
            let texture_index_arr: Vec<u32> =
                (0..bound_texture_map.len()).map(|v| v as u32).collect();

            gl::Uniform1iv(
                loc,
                texture_index_arr.len() as _,
                texture_index_arr.as_ptr() as _,
            );

            let projection_location = get_uniform_location(self.default_shader, "projection_view");

            gl::UniformMatrix4fv(
                projection_location,
                1,
                gl::FALSE,
                projection_view_matrix.as_ptr(),
            );

            gl::BindVertexArray(self.vao_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vertex_buffer_id);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (mem::size_of::<Vertex>() * verticies.len()) as _,
                verticies.as_ptr() as _,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.index_buffer_id);
            gl::BufferSubData(
                gl::ELEMENT_ARRAY_BUFFER,
                0,
                (mem::size_of::<i32>() * indicies.len()) as _,
                indicies.as_ptr() as _,
            );

            gl::DrawElements(
                gl::TRIANGLES,
                indicies.len() as _,
                gl::UNSIGNED_INT,
                ptr::null(),
            );

            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }

    pub fn swap_buffer(&self, window: &Window) {
        window.gl_swap_window();
    }

    pub fn resize_window(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }
}

impl Drop for GfxContext {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.default_shader);
        }
    }
}

pub fn get_uniform_location(shader: u32, name: &str) -> i32 {
    let c_str_name = CString::new(name).unwrap();
    unsafe { gl::GetUniformLocation(shader, c_str_name.as_ptr()) }
}

pub fn compile_shader(src: &str, ty: gl::types::GLenum) -> u32 {
    let shader;
    unsafe {
        shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(shader);
        let mut status = GLint::from(gl::FALSE);
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);

        if status != GLint::from(gl::TRUE) {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len(len as usize); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                shader,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );

            let mut status = GLint::from(gl::FALSE);
            gl::GetShaderiv(shader, gl::LINK_STATUS, &mut status);

            if status != GLint::from(gl::TRUE) {
                println!("Link error");
            }

            panic!(
                "couldn't compile shader {}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }

    shader
}

pub fn link_program(vs: u32, fs: u32) -> u32 {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
        // Get the link status
        let mut status = gl::FALSE as gl::types::GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);

        // Fail on error
        if status != (gl::TRUE as gl::types::GLint) {
            let mut len = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetProgramInfoLog(
                program,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut gl::types::GLchar,
            );
            panic!(
                "{}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ProgramInfoLog not valid utf8")
            );
        }
        program
    }
}

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

pub fn build_window(sdl_context: &Sdl, settings: &AppSettings) -> (Window, GLContext) {
    let video_subsystem = sdl_context.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 1);

    let window = video_subsystem
        .window("Window", settings.size.x as _, settings.size.y as _)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    // let window = window_builder.build().unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    let _ = video_subsystem.gl_set_swap_interval(SwapInterval::VSync);
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (4, 1));
    (window, _gl_context)
}

#[derive(Debug, Default)]
pub(crate) struct GlTextPipeline {
    program: GLuint,
    vao: GLuint,
    vbo: GLuint,
    vertex_count: usize,
    vertex_buffer_len: usize,
}

impl GlTextPipeline {
    pub fn new() -> Self {
        let vs = compile_shader(include_str!("../shaders/opengl/text.vs"), gl::VERTEX_SHADER);
        let fs = compile_shader(
            include_str!("../shaders/opengl/text.fs"),
            gl::FRAGMENT_SHADER,
        );
        let program = link_program(vs, fs);

        let mut vao = 0;
        let mut vbo = 0;

        unsafe {
            // Create Vertex Array Object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            // Create a Vertex Buffer Object
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Use shader program
            gl::UseProgram(program);

            let location = CString::new("out_color").unwrap();
            gl::BindFragDataLocation(program, 0, location.as_ptr());

            let mut offset = 0;
            for (v_field, float_count) in &[
                ("left_top", 3),
                ("right_bottom", 2),
                ("tex_left_top", 2),
                ("tex_right_bottom", 2),
                ("color", 4),
            ] {
                let location = CString::new(*v_field).unwrap();
                let attr = gl::GetAttribLocation(program, location.as_ptr());
                if attr < 0 {
                    println!("{} GetAttribLocation -> {}", v_field, attr);
                }
                gl::VertexAttribPointer(
                    attr as _,
                    *float_count,
                    gl::FLOAT,
                    gl::FALSE as _,
                    mem::size_of::<[f32; 13]>() as _,
                    offset as _,
                );
                gl::EnableVertexAttribArray(attr as _);
                gl::VertexAttribDivisor(attr as _, 1);

                offset += float_count * 4;
            }

            gl::UseProgram(0);
            gl_assert_ok!();
        }

        GlTextPipeline {
            program,
            vao,
            vbo,
            vertex_count: 0,
            vertex_buffer_len: 0,
        }
    }

    pub fn upload_vertices(&mut self, vertices: Vec<[f32; 13]>) {
        // Draw new vertices
        self.vertex_count = vertices.len();

        if self.vertex_count > 0 {
            unsafe {
                gl::BindVertexArray(self.vao);
                gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
                if self.vertex_buffer_len < self.vertex_count {
                    gl::BufferData(
                        gl::ARRAY_BUFFER,
                        (self.vertex_count * mem::size_of::<[f32; 13]>()) as GLsizeiptr,
                        vertices.as_ptr() as _,
                        gl::DYNAMIC_DRAW,
                    );
                    self.vertex_buffer_len = self.vertex_count;
                } else {
                    gl::BufferSubData(
                        gl::ARRAY_BUFFER,
                        0,
                        (self.vertex_count * mem::size_of::<[f32; 13]>()) as GLsizeiptr,
                        vertices.as_ptr() as _,
                    );
                }
                gl_assert_ok!();
            }
        }
    }

    pub fn flush(&self, projection_matrix: &crate::glm::Mat4x4) {
        if self.vertex_count > 0 {
            unsafe {
                gl::UseProgram(self.program);

                let projection_location = get_uniform_location(self.program, "projection");

                gl::UniformMatrix4fv(
                    projection_location,
                    1,
                    gl::FALSE,
                    projection_matrix.as_ptr(),
                );

                gl::BindVertexArray(self.vao);
                gl::DrawArraysInstanced(gl::TRIANGLE_STRIP, 0, 4, self.vertex_count as _);
                gl::UseProgram(0);
                gl_assert_ok!();
            }
        }
    }
}

impl Drop for GlTextPipeline {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}

struct Attribute<'a> {
    field_name: &'a str,
    float_count: i32,
    type_: types::GLenum,
}

impl<'a> Attribute<'a> {
    fn new(field_name: &'a str, float_count: i32, type_: types::GLenum) -> Self {
        Self {
            field_name,
            float_count,
            type_,
        }
    }
    fn new_float(field_name: &'a str, float_count: i32) -> Self {
        Self {
            field_name,
            float_count,
            type_: gl::FLOAT,
        }
    }
}

unsafe fn set_shader_attributes<'a>(
    program: u32,
    attributes: &[Attribute<'a>],
) -> Result<(), SmolError> {
    let mut offset = 0;
    for attribute in attributes {
        let attr = gl::GetAttribLocation(program, CString::new(attribute.field_name)?.as_ptr());
        if attr < 0 {
            return Err(SmolError::new(format!(
                "{} GetAttribLocation -> {}",
                attribute.field_name, attr
            )));
        }
        gl::VertexAttribPointer(
            attr as _,
            attribute.float_count,
            attribute.type_,
            gl::FALSE as _,
            mem::size_of::<Vertex>() as _,
            offset as _,
        );
        gl::EnableVertexAttribArray(attr as _);
        // gl::VertexAttribDivisor(attr as _, 1);

        offset += attribute.float_count * std::mem::size_of::<f32>() as i32
    }
    Ok(())
}
