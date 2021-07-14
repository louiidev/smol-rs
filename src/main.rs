use smol_rs::core::*;
use smol_rs::input::get_player_direction;
use smol_rs::math::*;
use smol_rs::render::{ FrameBuffer, Color};
use smol_rs::components::{Physics, Weapon, Invulnerable, Transform};
use smol_rs::events::{Events,  run_event_system_hecs, DamageAction };
use smol_rs::render_batch::{ Sprite, RenderBatch};
use std::collections::HashMap;
use hecs::World;
use smol_rs::texture_packer::{ AsepritePackerConfig, AsespritePacker };
use rand::{self, Rng};


const TILE_SIZE: u32 = 16;


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

    let mut batch = RenderBatch::default();

    let texture = load_texture_from_bytes(include_bytes!("../assets/atlas.png"));
    let frame_buffer = FrameBuffer::new(640 * 2, 480 * 2);
    // println!("texture id:{:?}", texture);

    let dot_texture = &texture.create_partial(16, 16, Vector2Int::new(0, 0));
    let grass_texture = &texture.create_partial(16, 16, Vector2Int::new(0, 0));
    let mut rng = rand::thread_rng();

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
        let player_sprite_coords = Vector2Int { x: 32, y:0 };
        let partial_texture = &texture.create_partial(16, 16, player_sprite_coords);
        for x in 0..(640/16) {
            for y in 0..(480/16) {
                let value = rng.gen_range(0..=1);
                if value == 0 {
                    render_texture_partial(&dot_texture, Vector2::new(x as f32 * 16., y as f32 * 16.));
                } else {
                    render_texture_partial(&grass_texture, Vector2::new(x as f32 * 16., y as f32 * 16.));
                }
                    
            }
        }
        render_texture_partial(&partial_texture, world.get::<Transform>(player).unwrap().screen_positon.into());     
        // render_rect(world.get::<Transform>(player).unwrap().screen_positon.x as f32, world.get::<Transform>(player).unwrap().screen_positon.y as f32,16., 16., Color(255., 255., 255., 100.));
        frame_buffer.unbind();
        
        render_framebuffer_scale(&frame_buffer.texture, Vector2 { x: 0., y: 0. }, Vector2::new(5., 5.));
        // render_texture(&texture, Vector2 { x: 0., y: 0.});
        end_render();
    }
}
