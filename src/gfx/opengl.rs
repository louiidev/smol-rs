use std::ffi::c_void;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use gl::types::GLchar;
use gl::types::GLint;
use image::DynamicImage;
use sdl2::video::GLContext;
use sdl2::video::GLProfile;
use sdl2::video::SwapInterval;
use sdl2::video::Window;
use sdl2::Sdl;

use crate::core::AppSettings;
use crate::math::*;
use crate::render::Color;
use crate::renderer::Vertex;

macro_rules! gl_assert_ok {
    () => {{
        let err = gl::GetError();
        assert_eq!(err, gl::NO_ERROR, "{}", gl_err_to_str(err));
    }};
}

pub struct GfxContext {
    default_shader: u32,
    vao_id: u32,
    vbo_id: u32,
}

impl GfxContext {
    pub fn new() -> Self {
        let fs = compile_shader(include_str!("../shader/test.fs"), gl::FRAGMENT_SHADER);
        let vs = compile_shader(include_str!("../shader/test.vs"), gl::VERTEX_SHADER);
        let default_shader = link_program(vs, fs);

        let vbo_id = 0;
        let vao_id = 0;

        GfxContext {
            default_shader,
            vao_id,
            vbo_id,
        }
    }

    pub fn clear(&self, color: Color) {
        let [r, g, b, a] = color.normalize();
        unsafe {
            gl::ClearColor(r, g, b, a);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn generate_texture<'a>(bytes: &'a [u8]) -> (i32, i32, u32) {
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

    pub fn render(&self, verticies: &Vec<Vertex>, indicies: &Vec<i32>) {
        unsafe {
            gl::BindVertexArray(self.vao_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo_id);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (mem::size_of::<Vertex>() * verticies.len()) as _,
                verticies.as_ptr() as _,
            )
        }
    }

    pub fn swap_buffer(&self, window: &Window) {
        window.gl_swap_window();
    }
}

impl Drop for GfxContext {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.default_shader);
        }
    }
}

pub fn compile_shader(src: &str, ty: gl::types::GLenum) -> u32 {
    let shader;
    unsafe {
        gl_assert_ok!();
        shader = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        println!("{}", src);
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
    unsafe {
        gl_assert_ok!();
    }
    (window, _gl_context)
}
