use std::ffi::c_void;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;

use crate::math::*;
use image::DynamicImage;

static VERTEX_DATA: [f32; 8] = [
    1.0, 1.0, // top right
    1.0, 0.0, // bottom right
    0.0, 0.0, // bottom left
    0.0, 1.0, // top left
];

static INDEX_DATA: [u32; 6] = [
    0, 1, 3, // first triangl::
    1, 2, 3, // second triangl::
];

#[derive(Debug, Default)]
pub struct Texture {
    id: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct PartialTexture {
    texture_id: u32,
    texture_width: u32,
    texture_height: u32,
    width: u32,
    height: u32,
    position: Vector2Int,
}

impl Texture {
    pub fn create_partial(&self, width: u32, height: u32, position: Vector2Int) -> PartialTexture {
        PartialTexture {
            texture_id: self.id,
            texture_width: self.width,
            texture_height: self.height,
            width,
            height,
            position,
        }
    }

    pub fn new_empty(width: i32, height: i32) -> Self {
        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                width,
                height,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
            // gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
            gl_check_errors();
        };
        let t = Texture {
            width: width as u32,
            height: height as u32,
            id: texture_id,
        };
        t
    }

    pub fn load_from_bytes(bytes: &[u8]) -> Self {
        let mut texture_id = 0;

        let (width, height, img_data, internal_format, format) =
            match image::load_from_memory(bytes).expect("Could not load image at src: {}") {
                DynamicImage::ImageRgba8(_image) => {
                    (
                        _image.width() as i32,
                        _image.height() as i32,
                        _image.into_raw(),
                        gl::RGBA8,
                        gl::RGBA,
                    )
                }
                DynamicImage::ImageRgb8(_image) => {
                    
                    (
                        _image.width() as i32,
                        _image.height() as i32,
                        _image.into_raw(),
                        gl::RGB8,
                        gl::RGB,
                    )
                }
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

        Texture {
            width: width as u32,
            height: height as u32,
            id: texture_id,
        }
    }

    pub fn load_from_file(src: &str) -> Self {
        let mut texture_id = 0;

        let (width, height, img_data, internal_format, format) =
            match image::open(src).expect("Could not load image at src: {}") {
                DynamicImage::ImageRgba8(img) => (
                    img.width() as i32,
                    img.height() as i32,
                    img.into_raw(),
                    gl::RGBA8,
                    gl::RGBA,
                ),
                DynamicImage::ImageRgb8(img) => (
                    img.width() as i32,
                    img.height() as i32,
                    img.into_raw(),
                    gl::RGB8,
                    gl::RGB,
                ),
                img => {
                    let image = img.to_rgba8();
                    (
                        image.width() as i32,
                        image.height() as i32,
                        image.into_raw(),
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
        };

        Texture {
            width: width as u32,
            height: height as u32,
            id: texture_id,
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        };
    }
}

pub fn gl_check_errors() {
    unsafe {
        let mut error_code = gl::GetError();
        while error_code != gl::NO_ERROR {
            match error_code {
                gl::INVALID_ENUM => println!("INVALID_ENUM"),
                gl::INVALID_VALUE => println!("INVALID_VALUE"),
                gl::INVALID_OPERATION => println!("INVALID_OPERATION"),
                gl::STACK_OVERFLOW => println!("INVALID_ENUM"),
                gl::STACK_UNDERFLOW => println!("INVALID_VALUE"),
                gl::OUT_OF_MEMORY => println!("INVALID_OPERATION"),
                gl::INVALID_FRAMEBUFFER_OPERATION => println!("INVALID_FRAMEBUFFER_OPERATION"),
                _ => println!("unknow error for code: {}", error_code),
            }

            error_code = gl::GetError();
        }
    }
}

#[derive(Debug, Default)]
pub struct FrameBuffer {
    pub texture: Texture,
    pub id: u32,
}

impl FrameBuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let mut fbo_id: u32 = 0;
        let texture = unsafe {
            let mut default_draw_fbo = 0;
            gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut default_draw_fbo);
            // Create the texture to render the data to, and attach it to our framebuffer
            let texture = Texture::new_empty(width, height);
            println!("texture id: {}", texture.id);
            // Create renderbuffer store the depth info
            let mut depth_texture: u32 = 0;
            gl::GenTextures(1, &mut depth_texture);
            gl::BindTexture(gl::TEXTURE_2D, depth_texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::DEPTH_COMPONENT as i32,
                width,
                height,
                0,
                gl::DEPTH_COMPONENT,
                gl::UNSIGNED_BYTE,
                ptr::null(),
            );
            gl::BindTexture(gl::TEXTURE_2D, 0);

            // Generate framebuffer
            gl::GenFramebuffers(1, &mut fbo_id);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);

            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture.id,
                0,
            );
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                depth_texture,
                0,
            );

            gl_check_errors();
            if gl::CheckFramebufferStatus(gl::DRAW_FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("ERROR IN OPENGL FRAME BUFFER");
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, default_draw_fbo as u32);

            texture
        };
        FrameBuffer {
            texture,
            id: fbo_id,
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
        };
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Color(pub u8, pub u8, pub u8, pub f32);

impl Color {
    pub fn into_gl(&self) -> (f32, f32, f32, f32) {
        (self.0 as f32 / 255., self.1 as f32 / 255., self.2 as f32 / 255., self.3)
    }
}

pub(crate) const WHITE: Color = Color(255, 255, 255, 1.);
pub(crate) const BLUE: Color = Color(10, 10, 255, 1.);
pub(crate) const RED: Color = Color(255, 10, 10, 1.);
pub(crate) const GREEN: Color = Color(10, 255, 10, 1.);
pub(crate) const BLACK: Color = Color(1, 1, 1, 1.);

pub fn get_uniform_location(shader: u32, name: &str) -> i32 {
    let c_str_name = CString::new(name).unwrap();
    unsafe { gl::GetUniformLocation(shader, c_str_name.as_ptr()) }
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

pub struct Renderer {
    shader_2d: u32,
    vao: u32,
    vbo: u32,
    ibo: u32,
    tbo: u32,
    default_texture: Texture,
    screen_scale: Vector2,
    pub frame_buffer: FrameBuffer,
}

impl Renderer {
    pub fn default(w: i32, h: i32) -> Self {
        let white_texture: u32 = 0xffffffff;
        let mut white_tex_id: u32 = 0;

        let vs = compile_shader(include_str!("shader/2d.vs"), gl::VERTEX_SHADER);
        let fs = compile_shader(include_str!("shader/2d.fs"), gl::FRAGMENT_SHADER);

        let shader = link_program(vs, fs);

        let mut vao: u32 = 0;
        let mut vertex_buffer_object: u32 = 0;
        let mut index_buffer_object: u32 = 0;
        let mut texture_buffer_object: u32 = 0;

        unsafe {
            gl::UseProgram(shader);
            gl::GenTextures(1, &mut white_tex_id);
            gl::BindTexture(gl::TEXTURE_2D, white_tex_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA8 as i32,
                1,
                1,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                &white_texture as *const _ as *const std::ffi::c_void,
            );

            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vertex_buffer_object);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize,
                &VERTEX_DATA[0] as *const f32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut index_buffer_object);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_object);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (INDEX_DATA.len() * mem::size_of::<f32>()) as isize,
                &INDEX_DATA[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * mem::size_of::<f32>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::GenBuffers(1, &mut texture_buffer_object);
            gl::BindBuffer(gl::ARRAY_BUFFER, texture_buffer_object);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize,
                &VERTEX_DATA[0] as *const f32 as *const c_void,
                gl::DYNAMIC_DRAW,
            );

            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                2 * mem::size_of::<f32>() as i32,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DeleteShader(fs);
            gl::DeleteShader(vs);
        }

        Renderer {
            vao,
            vbo: vertex_buffer_object,
            ibo: index_buffer_object,
            tbo: texture_buffer_object,
            shader_2d: shader,
            default_texture: Texture {
                width: 1,
                height: 1,
                id: white_tex_id,
            },
            screen_scale: Vector2::new(1., 1.),
            frame_buffer: FrameBuffer::new(w, h)
        }
    }

    pub fn clear(color: Color) {
        let (r, g, b, a) = color.into_gl();
        unsafe {
            gl::ClearColor(
               r, g, b, a 
            );
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn set_viewport(&mut self, x: f32, y: f32, width: u32, height: u32) {
        unsafe {
            gl::Viewport(x as i32, y as i32, width as i32, height as i32);
        };
        self.frame_buffer = FrameBuffer::new(width as i32, height as i32);
    }

    pub fn set_projection(&self, width: f32, height: f32) {
        
        unsafe {
            gl::UseProgram(self.shader_2d);
            let proj: [f32; 16] = Matrix::ortho(0.0, width, height, 0.0, -100.0, 100.0).into();
            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "projection"),
                1,
                gl::FALSE,
                proj.as_ptr(),
            );
            gl::UseProgram(0);
        }
    }

    pub fn set_offset(&self, offset: Vector2) {
        unsafe {
            gl::UseProgram(self.shader_2d);
            let view = Matrix::translate(Vector3 { x: offset.x, y: offset.y, z: 0.0 });

            let float_view: [f32; 16] = view.into();
            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "view"),
                1,
                gl::FALSE,
                float_view.as_ptr(),
            );
            gl::UseProgram(0);
        }
    }

    pub fn reset_offset(&self) {
        unsafe {
            gl::UseProgram(self.shader_2d);
            let view = Matrix::translate(Vector3 { x: 0.0, y: 0.0, z: 0.0 });

            let float_view: [f32; 16] = view.into();
            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "view"),
                1,
                gl::FALSE,
                float_view.as_ptr(),
            );
            gl::UseProgram(0);
        }
    }

    pub fn set_scale(&mut self, scale: Vector2) {
        self.screen_scale = scale;
    }

    pub fn rect(&self, width: f32, height: f32, x: f32, y: f32, color: Color) {
        let mut model = Matrix::translate(Vector3 { x, y, z: 0.0 });
        model.scale(Vector2 {
            x: width,
            y: height,
        });
        let float_model: [f32; 16] = model.into();
        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "model"),
                1,
                gl::FALSE,
                float_model.as_ptr(),
            );
            let (r, g, b, a) = color.into_gl();
            gl::Uniform4f(
                get_uniform_location(self.shader_2d, "u_color"),
                r, g, b, a
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.default_texture.id);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }

    pub fn texture(&self, texture: &Texture, position: Vector2) {
        Renderer::texture_scale(self, texture, position, 1.0);
    }

    pub fn texture_scale(&self, texture: &Texture, position: Vector2, scale: f32) {
        let mut model = Matrix::translate(position.into());
        model.scale(Vector2 {
            x: texture.width as f32 * scale,
            y: texture.height as f32 * scale,
        });

        let float_model: [f32; 16] = model.into();
        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "model"),
                1,
                gl::FALSE,
                float_model.as_ptr(),
            );
            gl::Uniform4f(
                get_uniform_location(self.shader_2d, "u_color"),
                1.0,
                1.0,
                1.0,
                1.0,
            );

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl_check_errors();
            gl::BindVertexArray(0);
            //Unbind program
            gl::UseProgram(0);
        }
    }

    pub fn texture_rect(self, texture: &Texture, rect: Rectangle, position: Vector2) {
        Renderer::texture_rect_scale(self, texture, rect, position, 1.0);
    }

    pub fn texture_rect1(&self, texture: &Texture, position: Vector2) {
        Renderer::texture_scale(self, texture, position, 1.0);
    }


    pub fn framebuffer_texture_scale(&self, texture: &Texture, position: Vector2, scale: Vector2) {
        let mut model = Matrix::translate(position.into());
        model.scale(Vector2 {
            x: texture.width as f32 * scale.x,
            y: texture.height as f32 * scale.y,
        });

        let float_model: [f32; 16] = model.into();

        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "model"),
                1,
                gl::FALSE,
                float_model.as_ptr(),
            );
            gl::Uniform4f(
                get_uniform_location(self.shader_2d, "u_color"),
                1.0,
                1.0,
                1.0,
                1.0,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
        }

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::BindVertexArray(self.vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        };
    }

    pub fn texture_rect_scale(
        self,
        texture: &Texture,
        rect: Rectangle,
        position: Vector2,
        scale: f32,
    ) {
        let mut model = Matrix::translate(Vector3 {
            x: position.x,
            y: position.y,
            z: 0.0,
        });
        model.scale(Vector2 {
            x: rect.w * scale,
            y: rect.h * scale,
        });
        let float_model: [f32; 16] = model.into();

        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "model"),
                1,
                gl::FALSE,
                float_model.as_ptr(),
            );
            gl::Uniform4f(
                get_uniform_location(self.shader_2d, "u_color"),
                1.0,
                1.0,
                1.0,
                1.0,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
        }

        let min = Vector2 {
            x: (rect.x * rect.w) / texture.width as f32,
            y: (rect.y * rect.h) / texture.height as f32,
        };

        let max = Vector2 {
            x: (rect.x + 1.0) * rect.w / texture.width as f32,
            y: (rect.y + 1.0) * rect.h / texture.height as f32,
        };

        let tex_coords: [f32; 8] = [max.x, max.y, max.x, min.y, min.x, min.y, min.x, max.y];

        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (tex_coords.len() * mem::size_of::<f32>()) as isize,
                &tex_coords[0] as *const f32 as *const c_void,
            );
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::BindVertexArray(self.vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            // RESET BUFFER TO DEFAULT
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize,
                &VERTEX_DATA[0] as *const f32 as *const c_void,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        };
    }

    pub fn render_texture_partial(&self, texture: &PartialTexture, position: Vector2) {
        let source = Rectangle {
            x: texture.position.x as f32,
            y: texture.position.y as f32,
            w: texture.width as f32,
            h: texture.height as f32,
        };

        let dest = Rectangle {
            x: position.x,
            y: position.y,
            w: texture.width as f32,
            h: texture.width as f32,
        };

        Renderer::atlas_sub_rect(
            self,
            texture.texture_id,
            texture.texture_width,
            texture.texture_height,
            source,
            dest,
        );
    }

    pub fn atlas_sub_rect(
        &self,
        texture_id: u32,
        texture_width: u32,
        texture_height: u32,
        sub_texture_data: Rectangle,
        dest: Rectangle,
    ) {
        unsafe {
            gl::UseProgram(self.shader_2d);
        }
        let mut model = Matrix::translate(Vector3 {
            x: dest.x,
            y: dest.y,
            z: 0.0,
        });
        model.scale(Vector2 {
            x: dest.w as f32,
            y: dest.h as f32,
        });
        let float_model: [f32; 16] = model.into();
        unsafe {
            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "model"),
                1,
                0,
                float_model.as_ptr(),
            );
            gl::Uniform4f(
                get_uniform_location(self.shader_2d, "u_color"),
                1.0,
                1.0,
                1.0,
                1.0,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
        }

        let min = Vector2 {
            x: sub_texture_data.x / texture_width as f32,
            y: sub_texture_data.y / texture_height as f32,
        };

        let max = Vector2 {
            x: (sub_texture_data.x + sub_texture_data.w) / texture_width as f32,
            y: (sub_texture_data.y + sub_texture_data.h) / texture_height as f32,
        };

        let tex_coords: [f32; 8] = [max.x, max.y, max.x, min.y, min.x, min.y, min.x, max.y];
        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (tex_coords.len() * mem::size_of::<f32>()) as isize,
                &tex_coords[0] as *const f32 as *const c_void,
            );
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::BindVertexArray(self.vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            // RESET BUFFER TO DEFAULT
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize,
                &VERTEX_DATA[0] as *const f32 as *const c_void,
            );
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
    }

    pub fn start_scissor(x: i32, y: i32, width: i32, height: i32) {
        unsafe {
            gl::Enable(gl::SCISSOR_TEST);
            gl::Scissor(x, y, width, height);
        }
    }

    pub fn end_scissor() {
        unsafe {
            gl::Disable(gl::SCISSOR_TEST);
        }
    }
    

}


impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.shader_2d);
            gl::DeleteBuffers(1, &self.tbo);
            gl::DeleteBuffers(1, &self.ibo);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
