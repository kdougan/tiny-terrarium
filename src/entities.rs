use hecs::World;

use crate::components::{Appearance, Growth, Plant, PlantType, Position};

pub fn spawn_plant(world: &mut World, x: f32, y: f32) {
    world.spawn((
        Plant {
            type_: PlantType::Grass,
        },
        Position { x, y },
        Appearance {
            size: 1.0,
            color: raylib::prelude::Color::new(0, 200, 0, 255),
        },
        Growth {
            growth_rate: 1.0,
            max_size: 32.0,
        },
    ));
}
