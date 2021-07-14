use crate::{events::DamageAction, math::{Vector2Int, Vector2}};

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


#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Transform {
    pub screen_positon: Vector2,
    pub grid_position: Vector2Int,
    pub scale: Vector2,
}

impl Transform {
    pub fn new_center() {

    }
    pub fn move_pos(&mut self, direction: Vector2Int) {
        self.grid_position+= direction;
        let screen_pos: Vector2 = direction.into();
        self.screen_positon+= screen_pos * GRID_SIZE_PIXELS as f32;
    }
}