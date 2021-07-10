use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::c_void;

use crate::math::*;
use image::DynamicImage;


static VERTEX_DATA: [f32; 8] = [
    1.0, 1.0,  // top right
    1.0, 0.0, // bottom right
    0.0, 0.0, // bottom left
    0.0, 1.0 // top left 
];

static INDEX_DATA: [u32; 6] = [
    0, 1, 3, // first triangl::
    1, 2, 3  // second triangl::
];



// Shader sources
static VS_SRC: &'_ str = "
    #version 330 core
    layout (location = 0) in vec2 vertex;
    layout (location = 1) in vec2 tex_coords;

    out vec2 TexCoord;

    uniform mat4 projection;
    uniform mat4 model;

    void main()
    {
        gl_Position = projection * model * vec4(vertex.xy, 0.0, 1.0);
        TexCoord = tex_coords;
    }";

static FS_SRC: &'_ str = "
    #version 330 core
    out vec4 FragColor;  
    in vec2 TexCoord;

    uniform sampler2D ourTexture;
    uniform vec4 u_color;

    
    void main()
    {
        FragColor = texture(ourTexture, TexCoord) * u_color; 
    }";



#[derive(Debug, Default)]
pub struct Texture {
    id: u32,
    width: u32,
    height: u32
}

#[derive(Debug, Clone)]
pub struct PartialTexture {
    texture_id: u32,
    texture_width: u32,
    texture_height: u32,
    width: u32,
    height: u32,
    position: Vector2Int
}

impl Texture {
    pub fn create_partial(&self, width: u32, height: u32, position: Vector2Int) -> PartialTexture {
        PartialTexture {
            texture_id: self.id,
            texture_width: self.width,
            texture_height: self.height,
            width,
            height,
            position
        }
    }

    pub fn new_empty(width: i32, height: i32) -> Self {
        let mut texture_id = 0;
        let t = Texture {
            width: width as u32,
            height: height as u32,
            id: texture_id
        };
        
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGB as i32, width, height, 0, gl::RGB, gl::UNSIGNED_BYTE, ptr::null());
            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl_check_errors();
        };
        t
    }

    pub fn load_from_file(src: &str) -> Self {
        let mut texture_id = 0;

        let (width, height, img_data, internal_format, format) = match image::open(src).expect("Could not load image at src: {}") {
            DynamicImage::ImageRgba8(img) => {
                (img.width() as i32, img.height() as i32, img.into_raw(), gl::RGBA8, gl::RGBA)
            },
            DynamicImage::ImageRgb8(img) => {
                (img.width() as i32, img.height() as i32, img.into_raw(), gl::RGB8, gl::RGB)
            },
            img => {
                let image = img.to_rgba();
                (image.width() as i32, image.height() as i32, image.into_raw(), gl::RGBA8, gl::RGBA)
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

            gl::TexImage2D(gl::TEXTURE_2D, 0, internal_format as i32, width, height, 0, format, gl::UNSIGNED_BYTE, img_ptr);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        };

        Texture {
            width: width as u32,
            height: height as u32,
            id: texture_id
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, &self.id); };
    }
}



fn gl_check_errors()
{
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
                _ => println!("unknow error for code: {}", error_code)
            }

            error_code = gl::GetError();
        }
    }
}

pub struct FrameBuffer {
    pub texture: Texture,
    id: u32
}

impl FrameBuffer {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {

        let mut fbo_id: u32 = 0;
        let texture = unsafe {
            let mut default_draw_fbo = 0;
            gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut default_draw_fbo);
            println!("default_draw_fbo: {} ", default_draw_fbo);
            // Generate framebuffer
            gl::GenFramebuffers(1, &mut fbo_id);
            gl::Viewport(x, y, width, height);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, fbo_id);
            // Create the texture to render the data to, and attach it to our framebuffer
            let texture = Texture::new_empty(width, height);
            gl::FramebufferTexture2D(gl::DRAW_FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture.id, 0);
            gl_check_errors();
            // Create renderbuffer store the depth info
            let mut rbo_id: u32 = 0;
            gl::GenRenderbuffers(1, &mut rbo_id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, rbo_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT32, width, height);
            gl::FramebufferRenderbuffer(gl::DRAW_FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, rbo_id);


            if gl::CheckFramebufferStatus(gl::DRAW_FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("ERROR IN OPENGL FRAME BUFFER");   
            }
            
           
            println!("fbo_id: {} ", fbo_id);
            
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, default_draw_fbo as u32);
            
            texture
        };
        FrameBuffer {
            texture,
            id: fbo_id,
        }
    }

    pub fn bind(&self) {
        unsafe { gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, self.id); }
    }

    pub fn unbind(&self) {
        unsafe { gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, 0); }
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteFramebuffers(1, &self.id); };
    }
}



pub struct Color (pub f32, pub f32, pub f32, pub f32);

fn get_uniform_location(shader: u32, name: &str) -> i32 {
    let c_str_name = CString::new(name).unwrap();
    unsafe { gl::GetUniformLocation(shader, c_str_name.as_ptr()) }
}


fn compile_shader(src: &str, ty: gl::types::GLenum) -> u32 {
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
            println!("{}", src);
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

fn link_program(vs: u32, fs: u32) -> u32 {
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
}

impl Renderer {
    pub fn default() -> Self {
        let white_texture: u32 = 0xffffffff;
        let mut white_tex_id: u32 = 0;

        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);

        let shader = link_program(vs, fs);

        let mut VAO: u32 = 0;
        let mut VBO: u32 = 0;
        let mut IBO: u32 = 0;
        let mut TBO: u32 = 0;

        unsafe {
      
            gl::UseProgram(shader);
            gl::GenTextures(1, &mut white_tex_id);
            gl::BindTexture(gl::TEXTURE_2D, white_tex_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, 1, 1, 0, gl::RGBA, gl::UNSIGNED_BYTE, &white_texture as *const _ as *const std::ffi::c_void);


            gl::GenVertexArrays(1, &mut VAO);
            gl::BindVertexArray(VAO);
            
            
            gl::GenBuffers(1, &mut VBO);
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER, (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize, &VERTEX_DATA[0] as *const f32 as *const c_void, gl::STATIC_DRAW);
    
            gl::GenBuffers(1, &mut IBO);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, IBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (INDEX_DATA.len() * mem::size_of::<f32>()) as isize, &INDEX_DATA[0] as *const u32 as *const c_void, gl::STATIC_DRAW);
    
            
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 2 * mem::size_of::<f32>() as i32, ptr::null());
            gl::EnableVertexAttribArray(0);
    
    
            gl::GenBuffers(1, &mut TBO);
            gl::BindBuffer(gl::ARRAY_BUFFER, TBO);
            gl::BufferData(gl::ARRAY_BUFFER, (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize, &VERTEX_DATA[0] as *const f32 as *const c_void, gl::DYNAMIC_DRAW);
    
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 2 * mem::size_of::<f32>() as i32, ptr::null());
            gl::EnableVertexAttribArray(1);
    
    
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::DeleteShader(fs);
            gl::DeleteShader(vs);
        }
        
        
        Renderer {
            vao: VAO,
            vbo: VBO,
            ibo: IBO,
            tbo: TBO,
            shader_2d: shader,
            default_texture: Texture {
                width: 1,
                height: 1,
                id: white_tex_id
            },
            screen_scale: Vector2::new(1.,1.),
        }
    }


    pub fn clear(color: Color) {
        unsafe {
            gl::ClearColor(color.0 / 255., color.1 / 255., color.2 / 255., color.3 /255.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn set_viewport(x: f32, y: f32, width: u32, height: u32) {
        unsafe { gl::Viewport(x as i32, y as i32, width as i32, height as i32); };
    }

    pub fn set_projection(&self, width: u32, height: u32) {
        unsafe {
            gl::UseProgram(self.shader_2d);
            let proj: [f32; 16] = Matrix::ortho(0.0, width as f32, height as f32, 0.0, -1.0, 1.0).into();
            gl::UniformMatrix4fv(get_uniform_location(self.shader_2d, "projection"), 1, gl::FALSE, proj.as_ptr());
            gl::UseProgram(0);
        }
    }

    pub fn set_scale(&mut self, scale: Vector2) {
        self.screen_scale = scale;
    }

    pub fn rect(&self, width: f32, height: f32, x: f32, y: f32, color: Color) {
        
		let mut model = Matrix::translate(Vector3 { x, y, z: 0.0 });
		model.scale(Vector2{ x: width, y: height });
		let float_model: [f32; 16] = model.into();
        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(get_uniform_location(self.shader_2d, "model"), 1, gl::FALSE, float_model.as_ptr());
            gl::Uniform4f(get_uniform_location(self.shader_2d, "u_color"), color.0 / 255., color.1 / 255., color.2 / 255.0, color.3 / 255.0);

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
        model.scale(Vector2 { x: texture.width as f32 * scale, y: texture.height as f32 * scale });
		
        let float_model: [f32; 16] = model.into();
        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(get_uniform_location(self.shader_2d, "model"), 1, gl::FALSE, float_model.as_ptr());
            gl::Uniform4f(get_uniform_location(self.shader_2d, "u_color"), 1.0, 1.0, 1.0, 1.0);
            
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

    pub fn texture_rect_scale(self, texture: &Texture, rect: Rectangle, position: Vector2, scale: f32) {
       
        let mut model = Matrix::translate(Vector3 { x: position.x, y: position.y, z: 0.0 });
		model.scale(Vector2{ x: rect.width * scale, y: rect.height * scale });
		let float_model: [f32; 16] = model.into();

        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(get_uniform_location(self.shader_2d, "model"), 1, gl::FALSE, float_model.as_ptr());
            gl::Uniform4f(get_uniform_location(self.shader_2d, "u_color"), 1.0, 1.0, 1.0, 1.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
        }
        
		let min = Vector2 {
            x: (rect.x * rect.width) / texture.width as f32,
            y: (rect.y * rect.height) / texture.height as f32,
        };

        let max = Vector2 {
            x: (rect.x + 1.0) * rect.width / texture.width as f32,
		    y: (rect.y + 1.0) * rect.height / texture.height as f32
        };

        let tex_coords: [f32; 8] = [
			max.x, max.y,
			max.x, min.y,
			min.x, min.y,
			min.x, max.y
        ];

        unsafe {
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (tex_coords.len() * mem::size_of::<f32>()) as isize, &tex_coords[0] as *const f32 as *const c_void);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::BindVertexArray(self.vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
		
            // RESET BUFFER TO DEFAULT
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize, &VERTEX_DATA[0] as *const f32 as *const c_void);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        };
    }

    pub fn render_texture_partial(&self, texture: &PartialTexture, position: Vector2) {

        let source = Rectangle {
            x: texture.position.x as f32,
            y: texture.position.y as f32,
            width: texture.width as f32,
            height: texture.height as f32
        };

        let dest = Rectangle {
            x: position.x,
            y: position.y,
            width: texture.width as f32 * 1.,
            height: texture.width as f32 * 1.
        };

        Renderer::atlas_sub_rect(self, texture.texture_id, texture.texture_width, texture.texture_height, source, dest);
    }

    pub fn atlas_sub_rect(&self, texture_id: u32, texture_width: u32, texture_height: u32, sub_texture_data: Rectangle, dest: Rectangle) {
        unsafe {
            gl::UseProgram(self.shader_2d);
        }
		let mut model = Matrix::translate(Vector3 { x: dest.x, y: dest.y, z: 0.0 });
		model.scale(Vector2 { x: dest.width as f32, y: dest.height as f32 });
        let float_model: [f32; 16] = model.into();
        unsafe {
            gl::UniformMatrix4fv(get_uniform_location(self.shader_2d, "model"), 1, 0, float_model.as_ptr());
            gl::Uniform4f(get_uniform_location(self.shader_2d, "u_color"), 1.0, 1.0, 1.0, 1.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
        }

        let min = Vector2  {
            x: sub_texture_data.x / texture_width as f32,
            y: sub_texture_data.y / texture_height as f32
        };

        let max = Vector2 {
            x: (sub_texture_data.x + sub_texture_data.width) / texture_width as f32,
            y: (sub_texture_data.y + sub_texture_data.height) /texture_height as f32
        };

		let tex_coords: [f32; 8] = [
			max.x, max.y,
			max.x, min.y,
			min.x, min.y,
			min.x, max.y
        ];
        unsafe {
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (tex_coords.len() * mem::size_of::<f32>()) as isize, &tex_coords[0] as *const f32 as *const c_void);
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::BindVertexArray(self.vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
		
            // RESET BUFFER TO DEFAULT
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize, &VERTEX_DATA[0] as *const f32 as *const c_void);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        }
		
    }

}

impl Drop for Renderer {
    fn drop(&mut self) {
        println!("Renderer dropped");
        unsafe {
            gl::DeleteProgram(self.shader_2d);
            gl::DeleteBuffers(1, &self.tbo);
            gl::DeleteBuffers(1, &self.ibo);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}