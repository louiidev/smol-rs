use std::ffi::c_void;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use crate::math::*;
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
        let vs = compile_shader(include_str!("shader/2d.vs"), gl::VERTEX_SHADER);
        let fs = compile_shader(include_str!("shader/2d.fs"), gl::FRAGMENT_SHADER);
        let default_shader = link_program(vs, fs);

        let vbo_id = 0;
        let vao_id = 0;
        unsafe {
            gl_assert_ok!();
        }
        GfxContext {
            default_shader,
            vao_id,
            vbo_id,
        }
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
}

impl Drop for GfxContext {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.default_shader);
        }
    }
}

pub fn compile_shader(src: &str, ty: gl::types::GLenum) -> u32 {
    let id;
    unsafe {
        id = gl::CreateShader(ty);
        let c_str = CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(id, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(id);
        let mut status = gl::FALSE as gl::types::GLint;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);
        if status != (gl::TRUE as gl::types::GLint) {
            let mut len = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // subtract 1 to skip the trailing null character
            gl::GetShaderInfoLog(
                id,
                len,
                ptr::null_mut(),
                buf.as_mut_ptr() as *mut gl::types::GLchar,
            );
            panic!(
                "couldn't compile shader {}",
                str::from_utf8(&buf)
                    .ok()
                    .expect("ShaderInfoLog not valid utf8")
            );
        }
    }

    id
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
