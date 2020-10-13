use smol_rs::core::*;
use smol_rs::math::*;
use std::collections::HashMap;

struct GameState {
    current_chunks: Vec<Vector2Int>,
}

struct MapChunk {}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::DOWN
    }
}

#[derive(Default)]
struct Player {
    position: Vector2,
    facing_direction: Direction,
    moving: bool,
    animation_frame: usize,
}


struct Animation<T> {
    frames: Vec<T>,
    frame_time: f32
}


struct AnimationSystem<T> {
    animations: HashMap<T, Vec<Vector2Int>>,
    current_state: T,
    current_time: f32,
}


impl <T>AnimationSystem<T> where
T: std::cmp::Eq
    + std::hash::Hash
    + std::clone::Clone
    + std::fmt::Debug, {
    fn register(&mut self, animation: T, frames: Vec<Vector2Int>) {
        self.animations.insert(animation, frames);
    }
}

fn main() {
    init();
    let mut player = Player::default();

    let player_animations = AnimationSystem {
        animations: HashMap::new(),
        current_state: Direction::DOWN,
        current_time: 0.0
    };

    let texture = load_texture("assets/tilemap_packed.png");
    while is_running() {
        let mut temp_pos = Vector2::default();

        if is_key_down(Keycode::W) {
            temp_pos.y -= 0.25 * delta_time();
            player.facing_direction = Direction::UP;
        } else if is_key_down(Keycode::S) {
            temp_pos.y += 0.25 * delta_time();
            player.facing_direction = Direction::DOWN;
        }

        if is_key_down(Keycode::A) {
            temp_pos.x -= 0.25 * delta_time();
            player.facing_direction = Direction::LEFT;
        } else if is_key_down(Keycode::D) {
            temp_pos.x += 0.25 * delta_time();
            player.facing_direction = Direction::RIGHT;
        }

        if temp_pos != Vector2::default() {
            temp_pos.normalize();
            player.moving = true;
        } else {
            player.moving = false;
        }

        player.position += temp_pos;
        clear();

        let player_sprite_coords = match player.facing_direction {
            Direction::DOWN => {
                if player.moving {
                    let frames = [Vector2Int { x: 24, y: 1 }, Vector2Int { x: 24, y: 2 }];
                    let next_frame = *frames.get(player.animation_frame).unwrap();
                    player.animation_frame = if frames.len() - 1 == player.animation_frame {
                        0
                    } else {
                        player.animation_frame + 1
                    };
                    println!("{}", player.animation_frame);
                    next_frame
                } else {
                    Vector2Int { x: 24, y: 0 }
                }
            }
            Direction::UP => {
                if player.moving {
                    let frames = [Vector2Int { x: 25, y: 1 },  Vector2Int { x: 25, y: 2 }];
                    let next_frame = *frames.get(player.animation_frame).unwrap();
                    player.animation_frame = if frames.len() - 1 == player.animation_frame {
                        0
                    } else {
                        player.animation_frame + 1
                    };
                    next_frame
                } else {
                    Vector2Int { x: 25, y: 0 }
                }
            }
            Direction::LEFT => {
                if player.moving {
                    let frames =[Vector2Int { x: 23, y: 1 },  Vector2Int { x: 23, y: 2 }];
                    let next_frame = *frames.get(player.animation_frame).unwrap();
                    player.animation_frame = if frames.len() - 1 == player.animation_frame {
                        0
                    } else {
                        player.animation_frame + 1
                    };
                    next_frame
                } else {
                    Vector2Int { x: 23, y: 0 }
                }
            },
            Direction::RIGHT => {
                if player.moving {
                    let frames = [Vector2Int { x: 26, y: 1 },  Vector2Int { x: 26, y: 2 }];
                    let next_frame = *frames.get(player.animation_frame).unwrap();
                    player.animation_frame = if frames.len() - 1 == player.animation_frame {
                        0
                    } else {
                        player.animation_frame + 1
                    };
                    next_frame
                } else {
                    Vector2Int { x: 26, y: 0 }
                }
            },
        };

        draw_sprite_from_atlas(&texture, player.position, player_sprite_coords, 16);
        end_render();
    }
}
