use hecs::{Entity, World};

use crate::{
    components::{PlayerController, Transform},
    math::Vec2Int,
};

pub fn get_entity_grid_position(world: &World, entity: Entity) -> Vec2Int {
    world.get::<Transform>(entity).unwrap().grid_position
}

pub fn get_entity_at_grid_position(world: &World, grid_position: Vec2Int) -> Option<Entity> {
    world
        .query::<&Transform>()
        .iter()
        .find(|(_, t)| grid_position == t.grid_position.clone())
        .map(|(e, _)| e)
}

// Should be good enough for now, we only expect one entity max
pub fn get_player_entity(world: &World) -> Option<Entity> {
    let mut players: Vec<Entity> = world
        .query::<&PlayerController>()
        .iter()
        .map(|(e, _)| e)
        .collect();

    players.pop()
}
