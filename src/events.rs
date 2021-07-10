use std::{any::Any, collections::HashMap};
use crate::math::Vector2Int;
use hecs::{ Entity, World };
use crate::components::{Invulnerable, Physics, Transform};
macro_rules! noop { () => (); }

struct Event {

}

#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct DamageAction {
    pub amount: u16
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Events {
    TakeDamage(DamageAction),
    Move(Vector2Int)
}


fn event_take_damage(world: &mut World, ent: Entity, action: &mut DamageAction) {
    if let Ok(comp) = world.get_mut::<Invulnerable>(ent) { comp.take_damage(action); }
    if let Ok(mut comp) = world.get_mut::<Physics>(ent) { comp.take_damage(action); }
    println!("check damage {:?}", world.get::<Physics>(ent).unwrap().health);
}

fn event_move(world: &mut World, ent: Entity, action: &mut Vector2Int) {
    if let Ok(mut comp) = world.get_mut::<Transform>(ent) {
        comp.move_pos(*action);
    }
}


pub fn run_event_system_hecs(world: &mut World, ent: Entity, event: &mut Events) {
    match event {
        Events::TakeDamage(action) =>event_take_damage(world, ent, action),
        Events::Move(action) => event_move(world, ent, action),
        _ => println!("EVENT NOT HANDLED {:?}", event)
    }
}