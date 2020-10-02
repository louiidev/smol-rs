use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::c_void;

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

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

struct Matrix
{
	m0: f32, m4: f32, m8: f32, m12: f32,
	m1: f32, m5: f32, m9: f32, m13: f32,
	m2: f32, m6: f32, m10: f32, m14: f32,
	m3: f32, m7: f32, m11: f32, m15: f32
}

impl Matrix {
    fn f32_array(self) -> [f32; 16]
	{
		[
            self.m0, self.m1, self.m2, self.m3,
            self.m4, self.m5, self.m6, self.m7,
            self.m8, self.m9, self.m10, self.m11,
            self.m12, self.m13, self.m14, self.m15
        ]
    }

    fn translate(base: Vector3) -> Self {
        Matrix {
            m0: 1.0, m4: 0.0, m8: 0.0, m12: base.x,
            m1: 0.0, m5: 1.0, m9: 0.0, m13: base.y,
            m2: 0.0, m6: 0.0, m10: 1.0, m14: base.z,
            m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0
        }
    }

    fn scale(&mut self, scale: Vector2) {
        self.m0 *= scale.x;
		self.m5 *= scale.y;
    }
}

pub struct Texture {
    id: u32,
    width: u32,
    height: u32
}

impl Texture {
    pub fn load_from_file(src: &str) -> Self {
        let mut texture_id = 0;
        let img = image::open(src).expect("Could not load image at src: {}");
        

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

pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32
}

pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

pub struct Vector3 {
    x: f32,
    y: f32,
    z: f32
}

impl From<Vector2> for Vector3 {
    fn from(item: Vector2) -> Self {
        Vector3 {
            x: item.x,
            y: item.y,
            z: 0.0
        }
    }
}

pub struct Color (pub f32, pub f32, pub f32, pub f32);

fn orthographic_projection(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix
{

    let rl = right - left;
    let tb = top - bottom;
    let f_n = far - near;

    Matrix {
        m0: 2.0 / rl,
        m1: 0.0,
        m2: 0.0,
        m3: 0.0,
        m4: 0.0,
        m5: 2.0 / tb,
        m6: 0.0,
        m7: 0.0,
        m8: 0.0,
        m9: 0.0,
        m10: -2.0 / f_n,
        m11: 0.0,
        m12: -(left + right) / rl,
        m13: -(top + bottom) / tb,
        m14: -(far + near) / f_n,
        m15: 1.0
    }
}


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
	default_texture: Texture
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

            let proj = orthographic_projection(0.0, SCREEN_WIDTH as f32, SCREEN_HEIGHT as f32, 0.0, -1.0, 1.0).f32_array();
            gl::UniformMatrix4fv(get_uniform_location(shader, "projection"), 1, gl::FALSE, mem::transmute(&proj[0]));

            gl::GenTextures(1, &mut white_tex_id);
            gl::BindTexture(gl::TEXTURE_2D, white_tex_id);
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, 1, 1, 0, gl::RGBA, gl::UNSIGNED_BYTE, &white_texture as *const _ as *const std::ffi::c_void);


            gl::GenVertexArrays(1, &mut VAO);
            gl::BindVertexArray(VAO);
            
            
            gl::GenBuffers(1, &mut VBO);
            gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
            gl::BufferData(gl::ARRAY_BUFFER, (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize, mem::transmute(&VERTEX_DATA[0]), gl::STATIC_DRAW);
    
            gl::GenBuffers(1, &mut IBO);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, IBO);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (INDEX_DATA.len() * mem::size_of::<f32>()) as isize, mem::transmute(&INDEX_DATA[0]), gl::STATIC_DRAW);
    
            
            gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 2 * mem::size_of::<f32>() as i32, ptr::null());
            gl::EnableVertexAttribArray(0);
    
    
            gl::GenBuffers(1, &mut TBO);
            gl::BindBuffer(gl::ARRAY_BUFFER, TBO);
            gl::BufferData(gl::ARRAY_BUFFER, (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize, mem::transmute(&VERTEX_DATA[0]), gl::DYNAMIC_DRAW);
    
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
        }
    }

    pub fn clear(color: Color) {
        unsafe {
            gl::ClearColor(color.0 / 255., color.1 / 255., color.2 / 255., color.3 /255.);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn set_viewport(width: u32, height: u32) {
        unsafe { gl::Viewport(0, 0, width as i32, height as i32); };
    }

    pub fn rect(&self, rect: Rectangle, color: Color) {
        
		let mut model = Matrix::translate(Vector3 { x: rect.x, y: rect.y, z: 0.0 });
		model.scale(Vector2{ x: rect.width, y: rect.height });
		let float_model = model.f32_array();
        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(get_uniform_location(self.shader_2d, "model"), 1, gl::FALSE, mem::transmute(&float_model[0]));
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
		
        let float_model = model.f32_array();
        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(get_uniform_location(self.shader_2d, "model"), 1, gl::FALSE, mem::transmute(&float_model[0]));
            gl::Uniform4f(get_uniform_location(self.shader_2d, "u_color"), 1.0, 1.0, 1.0, 1.0);
            
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            gl::BindVertexArray(0);
            //Unbind program
            gl::UseProgram(0);
        }
       
    }


    pub fn texture_rect(self, texture: Texture, rect: Rectangle, position: Vector2) {
        Renderer::texture_rect_scale(self, texture, rect, position, 1.0);
    }

    pub fn texture_rect_scale(self, texture: Texture, rect: Rectangle, position: Vector2, scale: f32) {
       
        let mut model = Matrix::translate(Vector3 { x: position.x, y: position.y, z: 0.0 });
		model.scale(Vector2{ x: rect.width * scale, y: rect.height * scale });
		let float_model = model.f32_array();

        unsafe {
            gl::UseProgram(self.shader_2d);
            gl::UniformMatrix4fv(get_uniform_location(self.shader_2d, "model"), 1, gl::FALSE, mem::transmute(&float_model[0]));
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
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (tex_coords.len() * mem::size_of::<f32>()) as isize, mem::transmute(&tex_coords[0]));
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::BindVertexArray(self.vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
		
            // RESET BUFFER TO DEFAULT
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize, mem::transmute(&VERTEX_DATA[0]));
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::UseProgram(0);
        };
    }

    pub fn atlas_sub(&self, texture: Texture, x_pos: u32, y_pos: u32, texture_size: u32, position: Vector2) {
        Renderer::atlas_sub_s(self, texture, x_pos, y_pos, texture_size, position, 1.0);
    }

    pub fn atlas_sub_s(&self, texture: Texture, x_pos: u32, y_pos: u32, texture_size: u32, position: Vector2, scale: f32) {

        let source = Rectangle {
            x: x_pos as f32,
            y: y_pos as f32,
            width: texture_size as f32,
            height: texture_size as f32
        };

        let dest = Rectangle {
            x: position.x,
            y: position.y,
            width: texture_size as f32 * scale,
            height: texture_size as f32 * scale
        };

        Renderer::atlas_sub_rect(self, texture, source, dest);
    }

    pub fn atlas_sub_rect(&self, texture: Texture, sub_texture_data: Rectangle, dest: Rectangle) {
        unsafe {
            gl::UseProgram(self.shader_2d);
        }
		let mut model = Matrix::translate(Vector3 { x: dest.x, y: dest.y, z: 0.0 });
		model.scale(Vector2 { x: dest.width as f32, y: dest.height as f32 });
        let float_model = model.f32_array();
        unsafe {
            gl::UniformMatrix4fv(get_uniform_location(self.shader_2d, "model"), 1, 0, mem::transmute(&float_model[0]));
            gl::Uniform4f(get_uniform_location(self.shader_2d, "u_color"), 1.0, 1.0, 1.0, 1.0);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
        }

        let min = Vector2  {
            x: (sub_texture_data.x * sub_texture_data.width) / texture.width as f32,
            y: (sub_texture_data.y * sub_texture_data.height) / texture.height as f32
        };

        let max = Vector2 {
            x: (sub_texture_data.x + 1.) * sub_texture_data.width / texture.width as f32,
            y: (sub_texture_data.y + 1.0) * sub_texture_data.height / texture.height as f32
        };

		let tex_coords: [f32; 8] = [
			max.x, max.y,
			max.x, min.y,
			min.x, min.y,
			min.x, max.y
        ];
        unsafe {
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (tex_coords.len() * mem::size_of::<f32>()) as isize, mem::transmute(&tex_coords[0]));
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture.id);
            gl::BindVertexArray(self.vao);

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
		
            // RESET BUFFER TO DEFAULT
            gl::BindBuffer(gl::ARRAY_BUFFER, self.tbo);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, (VERTEX_DATA.len() * mem::size_of::<f32>()) as isize, mem::transmute(&VERTEX_DATA[0]));
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