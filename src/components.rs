use std::fmt::Debug;

use hashbrown::HashMap;
use hecs::{Entity, EntityRef, World};
use rand::Rng;

use crate::{
    events::{Action, DamageAction, Events},
    math::{Vec2, Vec2Int},
    render::Color,
};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Invulnerable;

impl Invulnerable {
    pub fn take_damage(&self, action: &mut DamageAction) {
        action.amount = 0;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Physics {
    pub health: u16,
    pub max_health: u16,
    pub speed: f32,
    pub energy: f32,
}

impl Physics {
    pub fn new(starting_health: u16, speed: f32) -> Self {
        Self {
            health: starting_health,
            max_health: starting_health,
            speed,
            energy: 0.,
        }
    }

    pub fn take_damage(&mut self, action: &mut DamageAction) {
        self.health -= action.amount;
    }

    pub fn generate_energy(&mut self) {
        self.energy += self.speed;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Weapon {
    pub attack: u16,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PlayerController;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Transform {
    pub screen_positon: Vec2,
    pub grid_position: Vec2Int,
    pub scale: Vec2,
}

impl Transform {
    pub fn new_center() {}

    pub fn move_pos(&mut self, grid_position: Vec2Int) {
        self.grid_position = grid_position;
        let grid_pos: Vec2 = grid_position.into();
        self.screen_positon = Vec2::new(grid_pos.x * 16., grid_pos.y * 16.);
    }

    pub fn move_direction(&mut self, grid_direction: Vec2Int) {
        self.grid_position += grid_direction;
        let grid_pos: Vec2 = grid_direction.into();
        self.screen_positon += Vec2::new(grid_pos.x * 16., grid_pos.y * 16.);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteRenderer {
    pub scale: Vec2,
    pub name: String,
    pub color: Color,
}

impl Default for SpriteRenderer {
    fn default() -> Self {
        SpriteRenderer {
            scale: Vec2 { x: 1., y: 1. },
            color: Color(255, 255, 255, 1.),
            name: "".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Relationship(pub i32);

impl Relationship {
    pub fn is_friendly(&self) -> bool {
        self.0 >= 0
    }

    pub fn is_enemy(&self) -> bool {
        self.0 < 0
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Relationships(pub HashMap<Entity, Relationship>);

impl Relationships {
    pub fn is_enemy(&self, e: Entity) -> bool {
        if let Some(relationship) = self.0.get(&e) {
            relationship.is_enemy()
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Actor {
    pub action: Option<Action>,
    pub relationships: Relationships,
}

impl Actor {
    pub fn get_action(&mut self, grid_pos: Vec2Int) -> Action {
        let x = rand::thread_rng().gen_range(-1..2);
        let y = rand::thread_rng().gen_range(-1..2);

        let mut event = Events::Empty;

        if x != 0 || y != 0 {
            event = Events::MoveTo(Vec2Int::new(x, y) + grid_pos);
        }

        Action { cost: 1., event }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Captured {
    pub owner: Entity,
}

// pub fn follow_target() -> Action {

// }

// pub fn kill_target_goal(target: Entity) -> Action
// {
//     let range = calc_range();

//     if range > 1 {
//         return follow_target()
//     } else {

//     }
// }

// Find goal
// push goal kill player
// do action
//

pub fn find_action() {

    // Check current goals
    // if goals.len() > 0
    // try goal until valid
    // else search map
}

fn try_attack(world: &mut World, user: Entity, target: Entity) -> bool {
    false
}

pub trait Goal {
    fn finished(&self) -> bool;
    fn get_action(&self) -> Action;
    fn can_get_action_from_goal(&self) -> bool;
}

pub struct GoalHandler {
    pub goals: Vec<Box<dyn Goal>>,
}

fn get_child_goal<T: Goal>() -> Option<T> {
    None
}

impl GoalHandler {
    pub fn get_action(&mut self) -> Action {
        while self.goals.last().is_some() && self.goals.last().unwrap().finished() {
            self.goals.pop();
        }

        // let mut next_goal: Option<T> = None;

        if let Some(goal) = self.goals.last() {
            if goal.can_get_action_from_goal() {
                return goal.get_action();
            }
            // next_goal = Some(goal.get_child_goal());
        }

        // if let Some(next_goal) = next_goal {
        //     self.goals.push(next_goal);
        // }

        self.get_action()
    }
}

#[derive(Default)]
pub struct Inventory {
    pub items: Vec<Box<dyn Item>>,
}

// impl Inventory {
//     pub fn get_action(&self) {

//     }
// }

// #[derive(Default, Debug)]
// pub struct Item {
//     name: String,
//     description: String,
//     action: Action,
// }

pub trait Item: Send + Sync + ItemClone + Debug {
    fn get_action(&self, world: &mut World, parent: Entity) -> Option<Action>;
    fn name(&self) -> &str {
        "no name"
    }
    fn description(&self) -> &str {
        "no description"
    }
}

pub trait ItemClone {
    fn clone_box(&self) -> Box<dyn Item>;
}

impl<T> ItemClone for T
where
    T: 'static + Item + Clone,
{
    fn clone_box(&self) -> Box<dyn Item> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Item> {
    fn clone(&self) -> Box<dyn Item> {
        self.clone_box()
    }
}

#[derive(Clone, Default, Debug)]
pub struct HealthPotion {}

impl Item for HealthPotion {
    fn get_action(&self, world: &mut World, parent: Entity) -> Option<Action> {
        Some(Action {
            cost: 1.,
            event: Events::Empty,
        })
    }

    fn name(&self) -> &str {
        "Health Potion"
    }

    fn description(&self) -> &str {
        "no description"
    }
}

struct KillTarget {
    target: u32,
}

impl Goal for KillTarget {
    fn finished(&self) -> bool {
        true
    }

    fn get_action(&self) -> Action {
        Action::default()
    }
    fn can_get_action_from_goal(&self) -> bool {
        true
    }
}

struct B {}

impl Goal for B {
    fn finished(&self) -> bool {
        true
    }

    fn get_action(&self) -> Action {
        Action::default()
    }
    fn can_get_action_from_goal(&self) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::{components::Transform, events::Events, math::Vec2Int, world_setup::setup_world};

    use super::*;

    #[test]
    fn test_goal_handler() {
        let goal_handler = GoalHandler {
            goals: vec![Box::new(B {}), Box::new(KillTarget { target: 5 })],
        };
    }
}
