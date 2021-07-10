use std::ffi::CString;
use std::mem;
use std::ptr;
use std::str;
use std::ffi::c_void;

use crate::math::*;
use image::DynamicImage;



const MAX_BATCH_SIZE: u32 = 1000;
const VERTEX_SIZE: u32 = 6;
const VERTICIES_LEN: usize = (MAX_BATCH_SIZE * 4 * VERTEX_SIZE) as usize;


pub struct RenderBatch {
    pos_size: usize,
    colour_size: u8,
    pos_offset: usize,
    colour_offset: usize,
    vertex_size: u8,
    vertex_size_bytes: usize,
    verticies: [f32; VERTICIES_LEN],
    vao_id: u32,
    vbo_id: u32,
    ebo_id: u32,
}

impl RenderBatch {
    pub fn default() -> Self {

        let pos_offset = 0;
        let pos_size = 2;
        let vertex_size = 6;

        let VAO: u32 = 0;
        let VBO: u32 = 0;
        let EBO: u32 = 0;

        let verticies =  [0.; VERTICIES_LEN];

        

        let vertex_size_bytes = vertex_size * mem::size_of::<f32>();

        let colour_offset = pos_offset + pos_size * mem::size_of::<f32>();

        let mut render_batch = RenderBatch {
            pos_size,
            pos_offset,
            colour_size: 4,
            colour_offset,
            vertex_size: vertex_size as u8,
            vertex_size_bytes,
            vao_id: VAO,
            vbo_id: VBO,
            ebo_id: EBO,
            verticies,
        };

        let indices = render_batch.generate_indices();

        unsafe {
            gl::GenVertexArrays(1, &mut render_batch.vao_id);
            gl::BindVertexArray(render_batch.vao_id);

            gl::GenBuffers(1, &mut render_batch.vbo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, render_batch.vbo_id);
            gl::BufferData(gl::ARRAY_BUFFER, (render_batch.verticies.len() * mem::size_of::<f32>()) as isize, &render_batch.verticies[0] as *const f32 as *const c_void, gl::DYNAMIC_DRAW);

            gl::GenBuffers(1, &mut render_batch.ebo_id);
            gl::BindBuffer(gl::ARRAY_BUFFER, render_batch.ebo_id);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, (indices.len() * mem::size_of::<f32>()) as isize, &indices[0] as *const i32 as *const c_void, gl::STATIC_DRAW);


            gl::VertexAttribPointer(0, pos_size as i32,  gl::FLOAT, gl::FALSE, vertex_size_bytes as i32, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, vertex_size_bytes as i32, colour_offset as *const c_void);
            gl::EnableVertexAttribArray(1);


        }

        render_batch
    }

    fn generate_indices(&self) -> [i32; (6 * MAX_BATCH_SIZE) as usize] {
        let elements = [0; (6 * MAX_BATCH_SIZE) as usize];
        for i in 0..MAX_BATCH_SIZE {
            let offset_index = 6 * i;
            let offset = 4 * i;
        }

        elements
    }
}