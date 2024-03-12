use hecs::World;

use crate::{
    components::{Appearance, Fly, Growth, Plant, Position, Velocity, Weather, WeatherType},
    entities::spawn_plant,
    state::State,
};
use rand::Rng;
use raylib::{
    drawing::RaylibDraw,
    prelude::{color::Color, RaylibHandle, RaylibThread},
};

pub fn movement(world: &mut World, dt: f32) {
    for (_, (position, velocity)) in world.query_mut::<(&mut Position, &Velocity)>() {
        position.x += velocity.x * dt;
        position.y += velocity.y * dt;
    }
}

pub fn sporadic_movement(world: &mut World, rng: &mut impl Rng) {
    for (_, vel) in world.query_mut::<&mut Velocity>().with::<&Fly>() {
        vel.x += rng.gen_range(-1.0..=1.0) * 10.0;
        vel.y += rng.gen_range(-1.0..=1.0) * 10.0;
        vel.x = vel.x.clamp(-100.0, 100.0);
        vel.y = vel.y.clamp(-100.0, 100.0);
    }
}

pub fn bounce(world: &mut World) {
    for (_, (position, velocity)) in world.query_mut::<(&mut Position, &mut Velocity)>() {
        if position.x <= 0.0 || position.x >= 800.0 {
            velocity.x *= -1.0;
        }
        if position.y <= 0.0 || position.y >= 450.0 {
            velocity.y *= -1.0;
        }
    }
}

pub fn input(world: &mut World, rl: &RaylibHandle) {
    if rl.is_mouse_button_down(raylib::consts::MouseButton::MOUSE_LEFT_BUTTON) {
        let pos = rl.get_mouse_position();
        let x = pos.x;
        let y = pos.y;
        let x = x - x % 32.0 + 16.0;
        let y = y - y % 32.0 + 16.0;
        spawn_plant(world, x, y);
    }
}

pub fn render(state: &State, world: &World, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::new(30, 20, 30, 255));

    for row in &state.grid {
        for tile in row {
            d.draw_rectangle_lines(
                tile.x as i32 * tile.size as i32,
                tile.y as i32 * tile.size as i32,
                tile.size as i32,
                tile.size as i32,
                tile.color,
            );
        }
    }

    for (_, (position, appearance)) in world.query::<(&Position, &Appearance)>().iter() {
        d.draw_circle(
            position.x as i32,
            position.y as i32,
            appearance.size / 2.0,
            appearance.color,
        );
    }
}

pub fn growth(world: &mut World, dt: f32) {
    for (_, (appearance, growth)) in world.query_mut::<(&mut Appearance, &Growth)>() {
        if appearance.size < growth.max_size {
            appearance.size += growth.growth_rate * dt;
            if appearance.size > growth.max_size {
                appearance.size = growth.max_size;
            }
        }
    }
}

pub fn weather(world: &mut World, dt: f32) {
    for (_, weather) in world.query_mut::<&mut Weather>() {
        weather.duration -= dt;
        if weather.duration <= 0.0 {
            weather.type_ = match weather.type_ {
                WeatherType::Sunny => WeatherType::Rainy,
                WeatherType::Rainy => WeatherType::Sunny,
                _ => WeatherType::Sunny,
            };
            weather.duration = 5.0;
        }
    }
    // weather affects growth
    for (_, (growth, weather)) in world
        .query_mut::<(&mut Growth, &Weather)>()
        .with::<&Plant>()
    {
        match weather.type_ {
            WeatherType::Sunny => {
                growth.growth_rate = 0.1;
            }
            WeatherType::Rainy => {
                growth.growth_rate = 0.2;
            }
            _ => {}
        }
    }
}
