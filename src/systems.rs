use std::collections::VecDeque;

use hecs::{Entity, World};
use crate::events::{Action, run_event_system_hecs};
use crate::components::{Physics, Actor};


pub fn recusiverly_fire_action(world: &mut World, entities: &mut VecDeque<Entity>) {
    if let Some(e) = entities.pop_front() {
        let action: Option<Action> = {
            let mut actor = world.get_mut::<Actor>(e).unwrap();
            let mut physics = world.get_mut::<Physics>(e).unwrap();
            let action = actor.get_action();
            let can = action.cost <= physics.energy;

            let mut return_action: Option<Action> = None;
            println!("speed: {}",physics.speed);
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
            run_event_system_hecs(world, e, &mut action.unwrap().action);
        }
        recusiverly_fire_action(world, entities); 
    }
}



pub fn run_actor_actions(world: &mut World) {

    let mut entities: VecDeque<Entity> = {
        let mut ents =
        world.query_mut::<(&mut Physics, &mut Actor)>()
        .into_iter().map(|(e, (p, a))| {
            p.generate_energy();

            (e, (p, a))
        }).collect::<Vec<(Entity, (&mut Physics, &mut Actor))>>();

        ents.sort_by(
            |(_, (p1, _ )),
            (_, (p2, _ ))| {
                p2.speed.partial_cmp(&p1.speed).unwrap()
        });

        ents.iter().map(|(e,_)| *e).collect()
    };
       
    
    
    recusiverly_fire_action(world, &mut entities);
}
    


#[cfg(test)]
mod test {
    use crate::{components::Transform, events::Events, math::Vector2Int, world_setup::setup_world};

    use super::*;

    #[test]
    fn test_run_actor_actions() {
        let (mut world, player) = setup_world();
        let mut player_pos = {
            world.get_mut::<Transform>(player).unwrap().grid_position
        };
        {
            let mut actor = world.get_mut::<Actor>(player).unwrap();
            actor.action = Some(Action {
                cost: 1.,
                action: Events::Move(Vector2Int::new(1, 0))
            });
        }
       
        run_actor_actions(&mut world);
        player_pos+= Vector2Int::new(1, 0);
        let new_player_pos = {
            world.get_mut::<Transform>(player).unwrap().grid_position
        };
        assert_eq!(player_pos, new_player_pos);
    }

}