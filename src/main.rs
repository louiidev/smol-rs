use smol_rs::core::*;
use smol_rs::input::get_player_direction;
use smol_rs::math::*;
use smol_rs::render::{Color, FrameBuffer, PartialTexture};
use smol_rs::components::{Physics, Weapon, Invulnerable, Transform};
use smol_rs::events::{Events,  run_event_system_hecs, DamageAction };
use smol_rs::render_batch::{ Sprite, RenderBatch};
use std::collections::HashMap;
use hecs::World;
use smol_rs::texture_packer::{AseTextureData, TexturePacker};
use rand::{self, Rng};


const TILE_SIZE_X: u32 = 16;
const TILE_SIZE_Y: u32 = 24;


#[derive(Default)]
struct MapChunk {
    
}

#[derive(Default)]
struct Map {
    chunks: HashMap<Vector2Int, MapChunk>
}

impl Map {
    pub fn try_get_chunk(&mut self, position: &Vector2Int) -> Option<&mut MapChunk> {
        self.chunks.get_mut(position)
    }

    pub fn add_new_chunk(&mut self, position: Vector2Int) {
        self.chunks.insert(position, MapChunk{});
    }
}


fn example() {
    let mut current_chunk_pos = Vector2Int::default();
    let mut map = Map::default();
    map.add_new_chunk(current_chunk_pos.clone());

    current_chunk_pos.x+= 1;

    let new_chunk = if let Some(chunk) = map.try_get_chunk(&current_chunk_pos) {
        chunk
    } else {
        map.add_new_chunk(current_chunk_pos.clone());
        map.try_get_chunk(&current_chunk_pos).unwrap()
    };
}


fn main() {
    init();
    let mut world = World::new();
    let player = world.reserve_entity();
    world.insert_one(player, Transform {
        grid_position: Vector2Int::new(0, 0),
        screen_positon: Vector2::new(0., 0.),
        scale: Vector2::new(1., 1.)
    }).expect("Couldnt insert entity for player");

    // let mut batch = RenderBatch::default();

    let texture_packer = TexturePacker::new();
    let frame_buffer = FrameBuffer::new(640 * 2, 480 * 2);

    let dot_texture = texture_packer.get_texture("dot");
    let grass_texture = texture_packer.get_texture("grass");
    let player_tex = texture_packer.get_texture("player");
    let mut rng = rand::thread_rng();


    let tiles = {
        let mut v: HashMap<Vector2, PartialTexture> = HashMap::new();
        for x in 0..(640/TILE_SIZE_X) {
            for y in 0..(480/TILE_SIZE_Y) {
                let value = rng.gen_range(0..5);
                if value == 0 {
                    v.insert(Vector2::new(x as f32, y as f32), grass_texture.clone());
                } else {
                    v.insert(Vector2::new(x as f32, y as f32), dot_texture.clone());
                }
                    
            }
        }
        v
    };

    while is_running() {
        run_event_system_hecs(&mut world, player, &mut Events::Move(get_player_direction()));
        clear(Color (3., 31., 30., 255.));

        // batch.add_sprite(&Sprite {
        //     color: Color(100., 100., 100., 255.),
        //     transform: Transform {
        //         grid_position: Vector2Int::new(0, 0),
        //         screen_positon: Vector2::new(0., 0.),
        //         scale: Vector2::new(1., 1.)
        //     }
        // });

        //batch.render();
        frame_buffer.bind(640 * 2, 480 * 2);
        clear(Color (3., 31., 30., 255.));
        tiles.iter().for_each(|t| {
            render_texture_partial(t.1, Vector2::new(t.0.x * 16., t.0.y * 24.));    
        });
        let pos: Vector2 = world.get::<Transform>(player).unwrap().screen_positon.into();
        render_rect(pos.x, pos.y, 16., 16., Color (3., 31., 30., 255.));
        render_texture_partial(&player_tex, pos);     
        frame_buffer.unbind();
        
        render_framebuffer_scale(&frame_buffer.texture, Vector2 { x: 0., y: 0. }, Vector2::new(3., 3.));
        // render_texture(&texture, Vector2 { x: 0., y: 0.});
        end_render();
    }
}
