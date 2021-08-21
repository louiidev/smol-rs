use std::collections::VecDeque;

use crate::components::{Actor, Physics, Relationship};
use crate::events::{get_ai_action, run_event_system_hecs, Action};
use hecs::{Entity, World};

pub fn recusiverly_fire_action(world: &mut World, entities: &mut VecDeque<Entity>) {
    if let Some(e) = entities.pop_front() {
        let action: Option<Action> = {
            let (actor_action, actor_relationships) = {
                let mut actor = world.get_mut::<Actor>(e).unwrap();
                (actor.action.take(), actor.relationships.clone())
            };

            let action = if let Some(actor_action) = actor_action {
                actor_action
            } else {
                get_ai_action(world, e, actor_relationships)
            };
            let mut physics = world.get_mut::<Physics>(e).unwrap();

            let can = action.cost <= physics.energy;

            let mut return_action: Option<Action> = None;
            if can {
                physics.energy -= action.cost;
                if physics.energy > 0. {
                    // push entity to back of vec to run action again if energy is > 0
                    entities.push_back(e);
                }
                return_action = Some(action);
            }

            return_action
        };

        if action.is_some() {
            run_event_system_hecs(world, e, &mut action.unwrap().event);
        }
        recusiverly_fire_action(world, entities);
    }
}

pub fn run_actor_actions(world: &mut World) {
    let mut entities: VecDeque<Entity> = {
        let mut ents = world
            .query_mut::<(&mut Physics, &mut Actor)>()
            .into_iter()
            .map(|(e, (p, a))| {
                p.generate_energy();

                (e, (p, a))
            })
            .collect::<Vec<(Entity, (&mut Physics, &mut Actor))>>();

        ents.sort_by(|(_, (p1, _)), (_, (p2, _))| p2.speed.partial_cmp(&p1.speed).unwrap());

        ents.iter().map(|(e, _)| *e).collect()
    };

    recusiverly_fire_action(world, &mut entities);
}

pub fn create_bad_relationship(world: &mut World, entity: Entity, target: Entity) {
    if let Ok(mut comp) = world.get_mut::<Actor>(target) {
        if let Some(relationship) = comp.relationships.0.get_mut(&entity) {
            relationship.0 = relationship.0.min(-500);
        } else {
            comp.relationships.0.insert(entity, Relationship(-500));
        }
    }
}

pub fn cleanup_dead_entities(world: &mut World) {
    let ents: Vec<Entity> = world
        .query::<&Physics>()
        .iter()
        .filter(|(_, p)| !p.is_alive())
        .map(|(e, _)| e)
        .collect();

    if !ents.is_empty() {
        println!("{:?}", ents);
    }

    for e in ents {
        let _ = world.despawn(e);
    }
}

pub enum Rarity {
    Uncommon,
}

#[derive(Clone, Copy)]
pub struct Item {
    damage: u32,
}

pub fn get_uncommon_items_with_damage_above(world: &mut World, dmg: u32) -> Vec<Item> {
    world
        .query::<&Item>()
        .iter()
        .filter(|(_, i)| i.damage >= dmg)
        .map(|(_, i)| *i)
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{
        components::Transform, events::Events, math::Vec2Int, queries::get_player_entity,
        world_setup::setup_world,
    };

    use super::*;
    #[test]
    fn test_cleanup() {
        let mut world = World::new();

        let a = world.spawn((10, Physics::new(0, 0.)));
        let b = world.spawn((10, Physics::new(0, 0.)));
        let c = world.spawn((10, Physics::new(0, 0.)));
        cleanup_dead_entities(&mut world);

        assert_eq!(world.contains(a), false);
        assert_eq!(world.contains(b), false);
        assert_eq!(world.contains(c), false);
    }

    #[test]
    fn test_run_actor_actions() {
        let mut world = setup_world();
        let player = get_player_entity(&world).unwrap();
        let mut player_pos = { world.get_mut::<Transform>(player).unwrap().grid_position };
        {
            let mut actor = world.get_mut::<Actor>(player).unwrap();
            actor.action = Some(Action {
                cost: 1.,
                event: Events::MoveTo(player_pos + Vec2Int::new(1, 0)),
            });
        }

        run_actor_actions(&mut world);
        player_pos += Vec2Int::new(1, 0);
        let new_player_pos = { world.get_mut::<Transform>(player).unwrap().grid_position };
        assert_eq!(player_pos, new_player_pos);
    }
}
