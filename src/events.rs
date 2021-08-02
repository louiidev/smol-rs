use crate::{components::{Actor, Captured, Item, PlayerController, Relationship, Relationships}, logging::{log_new_message}, map::get_map, math::{Vector2, Vector2Int}, queries::get_entity_grid_position, render::{Color, WHITE}};
use hecs::{ Entity, World };
use crate::components::{Invulnerable, Physics, Transform};



#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DamageAction {
    pub amount: u16
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AttackAction {
    pub amount: u16,
    pub target: Entity
}


#[derive(Debug, Clone, Default)]
pub struct Action {
    pub cost: f32,
    pub event: Events
}

#[derive(Debug, Clone)]
pub struct ThrowAction {
    pub item: Box<dyn Item>,
    pub target: Entity,
}

#[derive(Debug, Clone)]
pub enum Events {
    TakeDamage(DamageAction),
    MoveTo(Vector2Int),
    MoveDirection(Vector2Int),
    Attack(AttackAction),
    ThrowItem(ThrowAction),
    Empty,
}

impl Default for Events {
    fn default() -> Self {
        Events::Empty
    }
}


fn event_take_damage(world: &mut World, ent: Entity, action: &mut DamageAction) {
    if let Ok(comp) = world.get_mut::<Invulnerable>(ent) { comp.take_damage(action); }
    if let Ok(mut comp) = world.get_mut::<Physics>(ent) { comp.take_damage(action); }
}

fn event_move_to(world: &mut World, ent: Entity, mut action: Vector2Int) {
    if let Ok(mut transform) = world.get_mut::<Transform>(ent) {
        if !get_map().is_tile_walkable(action) {
            action = transform.grid_position;
            if world.get::<PlayerController>(ent).is_ok() {
                log_new_message("That path is blocked");
            }
        }

        transform.move_pos(action);
    }
}

fn event_move_direction(world: &mut World, ent: Entity, action: &mut Vector2Int) {
    if let Ok(mut comp) = world.get_mut::<Transform>(ent) {
        comp.move_direction(*action);
    }
}

fn event_throw_item(world: &mut World, ent: Entity, action: &mut ThrowAction) {

    if let Ok(mut comp) = world.get_mut::<Actor>(action.target) {
        if let Some(relationship) = comp.relationships.0.get_mut(&action.target) {
            relationship.0 = relationship.0.min(-500);
        } else {
            comp.relationships.0.insert(action.target, Relationship(-500));
        }
    }

    log_new_message(&format!("You threw a [BLUE {} ] at [RED {:?}]", action.item.name(), action.target));
}


fn find_entities_in_distance(world: &mut World, ent: Entity, max_distance: f32) -> Vec<Entity> {
    let position: Vector2 = get_entity_grid_position(world, ent).into();
    world.query::<&Transform>().iter().filter(|(_, t)| {
        let pos: Vector2 = t.grid_position.into();
        pos.distance(position) <= max_distance
    }).map(|(e, _)| e).collect()
}


pub fn get_ai_action(world: &mut World, ent: Entity, relationships: Relationships) -> Action {

    let entities_nearby = find_entities_in_distance(world, ent, 10.);
    let target = entities_nearby.iter().find(|e| relationships.is_enemy(*e.to_owned()));

    if let Some(target) = target {
        // Attack
    }


    if let Ok(mut comp) = world.get_mut::<Captured>(ent) {
        // move towards target
    }

    let grid_pos = world.get::<Transform>(ent).unwrap().grid_position;

    world.get_mut::<Actor>(ent).unwrap().get_action(grid_pos)
}


pub fn run_event_system_hecs(world: &mut World, ent: Entity, event: &mut Events) {
    println!("EVENT FIRED: {:?}", event);
    match event {
        Events::TakeDamage(action) =>event_take_damage(world, ent, action),
        Events::MoveTo(action) => event_move_to(world, ent, *action),
        Events::MoveDirection(action) => event_move_direction(world, ent, action),
        Events::ThrowItem(action) => event_throw_item(world, ent, action),
        _ => {},
    }
}