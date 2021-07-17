use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::str;

use crate::components::Transform;
use crate::math::*;
use crate::render::{compile_shader, get_uniform_location, gl_check_errors, link_program, Color};

const MAX_BATCH_SIZE: u32 = 2;
const VERTEX_SIZE: u32 = 6;
const VERTICIES_LEN: usize = (MAX_BATCH_SIZE * 4 * VERTEX_SIZE) as usize;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Sprite {
    pub color: Color,
    pub transform: Transform,
}

// Shader sources
static VS_SRC: &'_ str = "
    #version 330 core
    layout (location = 0) in vec3 vertex;
    layout (location = 1) in vec4 color;

    out vec4 fColor;

     uniform mat4 projection;
     uniform mat4 model;
 
     void main()
     {
        fColor = color;
        gl_Position = vec4(vertex.x, 1-vertex.y, 0.0, 1.0);
    }";

static FS_SRC: &'_ str = "
    #version 330 core

    in vec4 fColor;

    out vec4 FragColor;  

    
    void main()
    {
        FragColor = fColor; 
    }";

pub struct RenderBatch {
    shader_2d: u32,
    verticies: [f32; VERTICIES_LEN],
    vao_id: u32,
    vertex_buffer_object_id: u32,
    sprites: Vec<Sprite>,
}

impl RenderBatch {
    pub fn default() -> Self {
        let pos_offset = 0;
        let pos_size = 2;
        let vertex_size = 6;

        let mut vao_id: u32 = 0;
        let mut vertex_buffer_object_id: u32 = 0;
        let mut index_buffer_object_id: u32 = 0;

        let mut verticies: [f32; VERTICIES_LEN] = [
            -1.5, -0.5,   0.18, 0.6, 0.96, 1.0,
            -0.5, -0.5,   0.18, 0.6, 0.96, 1.0,
            -0.5, 1.,    0.18, 0.6, 0.96, 1.0,
            -1.5, 1.,    0.18, 0.6, 0.96, 1.0,

            0.5, -0.5,    1., 0.93, 0.24, 1.0,
            1.5, -0.5,    1., 0.93, 0.24, 1.0,
            1.5, 0.5,     1., 0.93, 0.24, 1.0,
            0.5, 0.5,     1., 0.93, 0.24, 1.0,
        ];

        //RenderBatch::gen_vertex_props(&mut verticies, 0, Vector2::new(50., 50.), Color(0.18 * 25.5, 0.6 * 255., 0.96 * 255., 255.));
        // RenderBatch::gen_vertex_props(&mut verticies, 0, Vector2::new(150., 150.), Color(0.18 * 25.5, 0.6 * 255., 0.96 * 255., 255.));

        let indices: [u32; 12] = [0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4];

        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);

        let shader = link_program(vs, fs);
        let vertex_size_bytes = vertex_size * mem::size_of::<f32>();
        let colour_offset = pos_offset + pos_size * mem::size_of::<f32>();

        unsafe {
            gl::CreateVertexArrays(1, &mut vao_id);
            gl::BindVertexArray(vao_id);

            gl::CreateBuffers(1, &mut vertex_buffer_object_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_object_id);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (verticies.len() * mem::size_of::<f32>()) as isize,
                &verticies[0] as *const f32 as *const c_void,
                gl::DYNAMIC_DRAW,
            );

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                6 * mem::size_of::<f32>() as i32,
                0 as *const c_void,
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                4,
                gl::FLOAT,
                gl::FALSE,
                6 * mem::size_of::<f32>() as i32,
                colour_offset as *const c_void,
            );

            gl::CreateBuffers(1, &mut index_buffer_object_id);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, index_buffer_object_id);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<f32>()) as isize,
                &indices[0] as *const u32 as *const c_void,
                gl::STATIC_DRAW,
            );
        }

        RenderBatch {
            shader_2d: shader,
            vao_id: vao_id,
            vertex_buffer_object_id: vertex_buffer_object_id,
            verticies,
            sprites: Vec::default(),
        }
    }

    pub fn add_sprite(&mut self, sprite: &Sprite) {
        self.sprites.push(sprite.clone());

        if self.sprites.len() >= MAX_BATCH_SIZE as usize {
            self.render();
        }
    }

    fn gen_vertex_props(verticies: &mut [f32; VERTICIES_LEN as usize], index: usize, position: Vector2, color: Color) {
        let mut offset: usize = index * 4 * VERTEX_SIZE as usize;
        let mut x_add = 1.0;
        let mut y_add = 1.0;
        for i in 0..4 {
            match i {
                1 => y_add = 0.0,
                2 => x_add = 0.0,
                3 => y_add = 1.0,
                _ => {}
            }
            // load position
            verticies[offset] = position.x + x_add;
            verticies[offset + 1] = position.y + y_add;

            let gl_color = color.into_gl();
            // load color
            verticies[offset + 2] = gl_color.0;
            verticies[offset + 3] = gl_color.1;
            verticies[offset + 4] = gl_color.2;
            verticies[offset + 5] = gl_color.3;
            println!("{:?}", offset);
            offset += VERTEX_SIZE as usize;
        }
    }

    fn load_vertex_properties(&mut self, index: usize) {
        let sprite = self.sprites[index];
        // Find offset within array (4 vertices per sprite)
        let mut offset: usize = index * 4 * VERTEX_SIZE as usize;

        let mut x_add = 1.0;
        let mut y_add = 1.0;
        for i in 0..4 {
            match i {
                1 => y_add = 0.0,
                2 => x_add = 0.0,
                3 => y_add = 1.0,
                _ => {}
            }
            // load position
            self.verticies[offset] =
                sprite.transform.screen_positon.x + (x_add * sprite.transform.scale.x);
            self.verticies[offset + 1] =
                sprite.transform.screen_positon.y + (y_add * sprite.transform.scale.y);
            let gl_color = sprite.color.into_gl();
            // load color
            self.verticies[offset + 2] = gl_color.0;
            self.verticies[offset + 3] = gl_color.1;
            self.verticies[offset + 4] = gl_color.2;
            self.verticies[offset + 5] = gl_color.3;
            println!("{:?}", offset);
            offset += VERTEX_SIZE as usize;
        }
    }

    pub fn render(&mut self) {
        // for i in 0..self.sprites.len() {
        //     self.load_vertex_properties(i);
        // }

        let model = Matrix::translate(Vector3 {
            x: 0.,
            y: 0.,
            z: 0.0,
        });
        let float_model: [f32; 16] = model.into();
        unsafe {

            gl::UseProgram(self.shader_2d);


            self.set_projection(640., 480.);
            

            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "model"),
                1,
                gl::FALSE,
                float_model.as_ptr(),
            );

            gl::BindVertexArray(self.vao_id);

            gl::DrawElements(
                gl::TRIANGLES,
                2 * 6,
                gl::UNSIGNED_INT,
                ptr::null(),
            );
            gl_check_errors();
            gl::BindVertexArray(0);

            gl::UseProgram(0);
        }
        self.clear_batch();
    }

    fn clear_batch(&mut self) {
        self.sprites = Vec::default();
        self.verticies = [0.; VERTICIES_LEN];
    }

    pub fn set_projection(&self, width: f32, height: f32) {
        unsafe {
            let proj: [f32; 16] = Matrix::ortho(0.0, width, height, 0.0, -10.0, 10.0).into();

            gl::UniformMatrix4fv(
                get_uniform_location(self.shader_2d, "projection"),
                1,
                gl::FALSE,
                proj.as_ptr(),
            );
        }
    }

    fn generate_indices() -> [i32; (6 * MAX_BATCH_SIZE) as usize] {
        let mut elements = [0; (6 * MAX_BATCH_SIZE) as usize];
        for i in 0..MAX_BATCH_SIZE {
            let offset_index: usize = 6 * i as usize;
            let offset: i32 = 4 * i as i32;
            // Triangle 1
            elements[offset_index] = offset + 3;
            elements[offset_index + 1] = offset + 2;
            elements[offset_index + 2] = offset + 0;
            // Triangle 2
            elements[offset_index + 3] = offset + 0;
            elements[offset_index + 4] = offset + 2;
            elements[offset_index + 5] = offset + 1;
        }

        elements
    }
}
