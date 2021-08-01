use hecs::{Entity, World};

use crate::{components::Transform, math::{Vector2Int}};



pub fn get_entity_grid_position(world: &World, entity: Entity) -> Vector2Int  {
    world.get::<Transform>(entity).unwrap().grid_position
}

pub fn get_entity_at_grid_position(world: &World, grid_position: Vector2Int) -> Option<Entity> {
    world.query::<&Transform>().iter().find(|(_, t)| grid_position == t.grid_position.clone()).map(|(e, _)| e)
}