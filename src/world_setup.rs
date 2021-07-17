use crate::components::*;
use crate::math::*;
use hecs::Entity;
use hecs::World;

pub fn setup_world() -> (World, Entity) {
    let mut world = World::new();

    let player = world.spawn((
        Transform {
            grid_position: Vector2Int::new(5, 5),
            screen_positon: Vector2::new(5. * 16., 5. * 16.),
            scale: Vector2::new(1., 1.),
        },
        Physics { health: 10, speed: 1., ..Default::default() },
        Actor::default(),
        SpriteRenderer {
            name: "player".to_string(),
            ..Default::default()
        }
    ));

    world.spawn((
        Transform {
            grid_position: Vector2Int::new(10, 10),
            screen_positon: Vector2::new(10. * 16., 10. * 16.),
            scale: Vector2::new(1., 1.),
        },
        Physics { health: 10, speed: 1., ..Default::default() },
        Actor::default(),
        SpriteRenderer {
            name: "creature".to_string(),
            ..Default::default()
        }
    ));
    
    
    (world, player)
}
