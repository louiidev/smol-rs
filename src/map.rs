use crate::{components::Transform, math::*, queries::get_entity_grid_position};
use std::{sync::Mutex};
use hashbrown::HashMap;
use hecs::{Entity, World};
use lazy_static::lazy_static;
use rand::Rng;



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

#[derive(Debug, Clone)]
pub enum TileType {
    Empty,
    Tree,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Empty
    }
}


#[derive(Default, Debug, Clone)]
pub struct Tile {
    pub walkable: bool,
    pub texture_name: String,
    pub tile_type: TileType,

    // ONLY USED FOR PATH FINDING
    pub g: i32,
    pub f: i32,
    pub previous: Option<Vector2Int>
}


#[derive(Default, Debug)]
pub struct MapChunk {
    position: Vector2Int,
    pub tiles: HashMap<Vector2Int, Tile>
}

impl MapChunk {
    pub fn generate(position: Vector2Int) -> Self {
        let mut tiles = HashMap::default();
        let mut rng = rand::thread_rng();
        for x in 0..MAX_CHUNK_SIZE {
            for y in 0..MAX_CHUNK_SIZE {
                let value = rng.gen_range(0..10);
                let texture_name = if value == 0 || value == 1 {
                    "grass"
                } else if value == 2 {
                    "tree"
                } else {
                    "dot"
                };
                tiles.insert(Vector2Int::new(x, y) + position, Tile {
                    walkable: texture_name != "tree",
                    texture_name: texture_name.into(),
                    tile_type: match texture_name {
                        "tree" => TileType::Tree,
                        _ => TileType::Empty
                    },
                    ..Default::default()
                });
            }
        }

        MapChunk {
            position,
            tiles
        }
    }

    pub fn get_tiles_range_of(&self, start_tile_position: Vector2Int, range: i32) -> HashMap<Vector2Int, Tile> {
        let mut tiles_to_return = HashMap::new();

        for x in -range..range+1 {
            for y in -range..range+1 {
                let pos = Vector2Int::new(x, y) + start_tile_position;
                let pot_tile = self.tiles.get(&pos);
                if let Some(tile) = pot_tile {
                    tiles_to_return.insert(pos, tile.clone());
                }
            }
        }


        tiles_to_return
    }
}





#[derive(Default, Debug)]
pub struct Map {
    chunks: HashMap<Vector2Int, MapChunk>,
    current_chunk: Vector2Int,
}

impl Map {
    pub fn new() -> Self {
        let mut chunks = HashMap::new();
        chunks.insert(Vector2Int::default(), MapChunk::generate(Vector2Int::default()));
        Map {
            chunks,
            current_chunk: Vector2Int::default()
        }
    }

    pub fn get_current_chunk(&mut self) -> &MapChunk {
        let pos = self.current_chunk;
        self.try_get_chunk(&pos).unwrap()
    }

    pub fn try_get_chunk(&self, position: &Vector2Int) -> Option<&MapChunk> {
        self.chunks.get(position)
    }

    pub fn try_get_mut_chunk(&mut self, position: &Vector2Int) -> Option<&mut MapChunk> {
        self.chunks.get_mut(position)
    }

    pub fn add_new_chunk(&mut self, position: Vector2Int) {
        self.chunks.insert(position, MapChunk::generate(position));
    }

    pub fn get_mut_chunk_from_position(&mut self, position: Vector2Int) -> &mut MapChunk {

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

        self.try_get_mut_chunk(&chunk_pos).unwrap()
    }

    pub fn get_chunk_from_position(&self, position: Vector2Int) -> &MapChunk {

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

    /// Gets a tile from the world map regardless on which chunk
    /// ```
    pub fn get_tile_from_grid_position(&self, position: Vector2Int) -> Option<&Tile> {
        let chunk = self.get_chunk_from_position(position);

        chunk.tiles.get(&position)
    }


    pub fn is_tile_walkable(&self, position: Vector2Int) -> bool {
        if let Some(tile) = self.get_tile_from_grid_position(position) {
            return tile.walkable
        }

        false
    }

    pub fn get_all_other_entities_in_chunk(&mut self, world: &mut World, ent: Entity) -> Vec<Entity> {
        let query_ent_position = get_entity_grid_position(world, ent);
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

    #[test]
    fn test_get_tiles_range_of() {
        let chunk = MapChunk::generate(Vector2Int::default());

        let range_tiles = chunk.get_tiles_range_of(Vector2Int::new(5, 5), 2);

        assert_eq!(range_tiles.len(), 25);
    }
}