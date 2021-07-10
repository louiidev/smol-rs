use crate::{events::DamageAction, math::{Vector2Int}};

const GRID_SIZE_PIXELS: i32 = 16;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Invulnerable;

impl Invulnerable {
    pub fn take_damage(&self, action: &mut DamageAction) {
        action.amount = 0;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Physics {
   pub health: u16
}

impl Physics {
    pub fn take_damage(&mut self, action: &mut DamageAction) {
        self.health-= action.amount;
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Weapon {
    pub attack: u16
}


#[derive(Debug, Clone, Default, PartialEq)]
pub struct Transform {
    pub screen_positon: Vector2Int,
    pub grid_position: Vector2Int,
}

impl Transform {
    pub fn new_center() {

    }
    pub fn move_pos(&mut self, direction: Vector2Int) {
        self.grid_position+= direction;
        self.screen_positon+= direction * GRID_SIZE_PIXELS;
    }
}