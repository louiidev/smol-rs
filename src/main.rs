use smol_rs::core::*;
use smol_rs::input::get_player_direction;
use smol_rs::math::*;
use smol_rs::render::{ FrameBuffer, Color};
use smol_rs::components::{Physics, Weapon, Invulnerable, Transform};
use smol_rs::events::{Events,  run_event_system_hecs, DamageAction };
use std::collections::HashMap;
use hecs::World;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::DOWN
    }
}

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
    let mut player = world.reserve_entity();
    let start_point = get_screen_center() - 16;
    world.insert_one(player, Transform {
        grid_position: start_point / 16,
        screen_positon: start_point,
    });

    let frame_buffer = FrameBuffer::new(0, 0, 640 - 66, 180);

    let texture = load_texture("assets/tilemap_packed.png");
    while is_running() {
        run_event_system_hecs(&mut world, player, &mut Events::Move(get_player_direction()));
        clear();
        frame_buffer.bind();
        clear();
        

        let player_sprite_coords = Vector2Int { x: 384, y: 0 };

        let partial_texture = &texture.create_partial(16, 16, player_sprite_coords);
        
        render_texture_partial(&partial_texture, world.get::<Transform>(player).unwrap().screen_positon.into());
        render_rect(0., 0., 32., 32., Color(255., 255., 255., 100.));
        frame_buffer.unbind();

        render_texture(&frame_buffer.texture, Vector2 { x: 0., y: 64. });
        end_render();
    }
}
