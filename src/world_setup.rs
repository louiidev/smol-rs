use crate::components::*;
use crate::math::*;
use hecs::Entity;
use hecs::World;

pub type WorldPlayer = (World, Entity);

pub fn setup_world() -> WorldPlayer {
    let mut world = World::new();

    let player = world.spawn((
        Transform {
            grid_position: Vec2Int::new(5, 5),
            screen_positon: Vec2::new(5. * 16., 5. * 16.),
            scale: Vec2::new(1., 1.),
        },
        Physics::new(10, 1.),
        Actor::default(),
        SpriteRenderer {
            name: "player".to_string(),
            ..Default::default()
        },
        PlayerController,
        Inventory {
            items: vec![
                Box::new(HealthPotion::default()),
                Box::new(HealthPotion::default()),
                Box::new(HealthPotion::default()),
            ],
        },
    ));

    world.spawn((
        Transform {
            grid_position: Vec2Int::new(10, 10),
            screen_positon: Vec2::new(10. * 16., 10. * 16.),
            scale: Vec2::new(1., 1.),
        },
        Physics::new(10, 2.),
        Actor::default(),
        SpriteRenderer {
            name: "minicoo".to_string(),
            ..Default::default()
        },
    ));

    (world, player)
}
