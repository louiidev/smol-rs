use rand::Rng;

use crate::{events::{DamageAction, Events, Action }, math::{Vector2Int, Vector2}, render::Color};

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
   pub speed: f32,
   pub energy: f32
}

impl Physics {
    pub fn take_damage(&mut self, action: &mut DamageAction) {
        self.health-= action.amount;
    }

    pub fn generate_energy(&mut self) {
        self.energy+= self.speed;
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
        self.screen_positon+= Vector2::new(screen_pos.x * 16., screen_pos.y * 16.);
        // self.screen_positon+= screen_pos * GRID_SIZE_PIXELS as f32;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpriteRenderer {
    pub scale: Vector2,
    pub name: String,
    pub color: Color
}

impl Default for SpriteRenderer {
    fn default() -> Self {
        SpriteRenderer {
            scale: Vector2 { x: 1., y: 1.},
            color: Color(255, 255, 255, 1.),
            name: "".to_string(),
        }
    }
}


#[derive(Debug, Clone, Default, PartialEq)]
pub struct Actor {
    pub action: Option<Action>
}

impl Actor {
    pub fn get_action(&mut self) -> Action {
        if self.action.is_some() {
            return self.action.take().unwrap()
        }

        let x = rand::thread_rng().gen_range(-1..2);
        let y = rand::thread_rng().gen_range(-1..2);

        Action {
            cost: 1.,
            action: Events::Move(Vector2Int::new(x, y))
        }
    }
}