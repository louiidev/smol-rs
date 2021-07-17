use crate::{components::Transform, math::*};
use std::{collections::HashMap, sync::Mutex};
use hecs::{Entity, World};
use lazy_static::lazy_static;



lazy_static! {
    static ref MAP: Mutex<Map> = {
        Mutex::new(Map::new())
    };
}

pub fn get_map() -> std::sync::MutexGuard<'static, Map> {
    MAP.lock().unwrap()
}

// Tiles per axis basically so atm 40x40 tiles in a chunk
pub static MAX_CHUNK_SIZE: i32 = 40;


#[derive(Default, Debug)]
pub struct MapChunk {
    position: Vector2Int,
}

impl MapChunk {
    pub fn generate(position: Vector2Int) -> Self {

        MapChunk {
            position
        }
    }
}





#[derive(Default, Debug)]
pub struct Map {
    chunks: HashMap<Vector2Int, MapChunk>
}

impl Map {
    pub fn new() -> Self {
        let mut chunks = HashMap::new();
        chunks.insert(Vector2Int::default(), MapChunk::generate(Vector2Int::default()));
        Map {
            chunks
        }
    }
}

impl Map {
    pub fn try_get_chunk(&mut self, position: &Vector2Int) -> Option<&mut MapChunk> {
        self.chunks.get_mut(position)
    }

    pub fn add_new_chunk(&mut self, position: Vector2Int) {
        self.chunks.insert(position, MapChunk::generate(position));
    }

    pub fn get_chunk_from_position(&mut self, position: Vector2Int) -> &mut MapChunk {

        let Vector2Int { x, y } = position;

        let chunk_pos = {
            let mut pos = Vector2Int::default();
            if x != 0 {
                pos.x = x / MAX_CHUNK_SIZE;
            }
            if y != 0 {
                pos.y = y / MAX_CHUNK_SIZE;
            }
            pos
        };

        self.try_get_chunk(&chunk_pos).unwrap()
    }

    pub fn get_all_other_entities_in_chunk(&mut self, world: &mut World, ent: Entity) -> Vec<Entity> {
        let query_ent_position = world.get::<Transform>(ent).unwrap().grid_position;
        let target_chunk_pos = self.get_chunk_from_position(query_ent_position).position;

        world.query::<&Transform>().iter().filter(|(e, t)| {
            *e != ent && self.get_chunk_from_position(t.grid_position).position == target_chunk_pos
        }).map(|(e, _)| e).collect()
    }

    pub fn get_all_entities_in_chunk(&mut self, world: &mut World, chunk_pos: Vector2Int) -> Vec<Entity> {
        let target_chunk_pos = self.get_chunk_from_position(chunk_pos).position;

        world.query::<&Transform>().iter().filter(|(_, t)| {
            self.get_chunk_from_position(t.grid_position).position == target_chunk_pos
        }).map(|(e, _)| e).collect()
    }
}



fn _example() {
    let mut current_chunk_pos = Vector2Int::default();
    let mut map = Map::default();
    map.add_new_chunk(current_chunk_pos.clone());

    current_chunk_pos.x+= 1;

    let _new_chunk = if let Some(chunk) = map.try_get_chunk(&current_chunk_pos) {
        chunk
    } else {
        map.add_new_chunk(current_chunk_pos.clone());
        map.try_get_chunk(&current_chunk_pos).unwrap()
    };
} 



#[cfg(test)]
mod test {
    use crate::{math::Vector2Int};
    use hecs::World;
    use super::*;

    #[test]
    fn test_get_chunk_pos() {
        let mut map = get_map();
        let try_pos = Vector2Int::new(30, 30);
        let chunk_pos = map.get_chunk_from_position(try_pos).position;
        assert_eq!(Vector2Int::new(0, 0), chunk_pos);        
    }

    #[test]
    fn large_chunk_get() {
        let mut map = get_map();
        map.add_new_chunk(Vector2Int::new(101, 404));
        let try_pos = Vector2Int::new(101 * MAX_CHUNK_SIZE, 404 * MAX_CHUNK_SIZE);
        let chunk_pos = map.get_chunk_from_position(try_pos).position;
        assert_eq!(Vector2Int::new(101, 404), chunk_pos);
    }

    #[test]
    fn div_by_zero() {
        let mut map = get_map();

        let try_pos = Vector2Int::new(0, 30);
        let chunk_pos = map.get_chunk_from_position(try_pos).position;
        assert_eq!(Vector2Int::new(0, 0), chunk_pos);
    }

    #[test]
    fn test_entitie_chunk_query() {
        let mut world = World::new();
        let mut map = get_map();
        map.add_new_chunk(Vector2Int::new(1, 1));
        world.spawn((Transform {
            grid_position: Vector2Int {
                x: MAX_CHUNK_SIZE + 1,
                y: MAX_CHUNK_SIZE + 1
            },
            ..Default::default()
        }, true));

        let a = world.spawn((Transform {
            grid_position: Vector2Int {
                x: MAX_CHUNK_SIZE,
                y: MAX_CHUNK_SIZE
            },
            ..Default::default()
        }, true));

        world.spawn((Transform {
            grid_position: Vector2Int {
                x: MAX_CHUNK_SIZE - 1,
                y: MAX_CHUNK_SIZE - 1
            },
            ..Default::default()
        }, true));

        assert_eq!(map.get_all_other_entities_in_chunk(&mut world, a).len(), 1);


        assert_eq!(map.get_all_entities_in_chunk(&mut world, Vector2Int {
            x: MAX_CHUNK_SIZE - 1,
            y: MAX_CHUNK_SIZE - 1
        }).len(), 1);
    }
}