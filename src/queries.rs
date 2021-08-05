use hecs::{Entity, World};

use crate::{components::Transform, math::Vec2Int};

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
