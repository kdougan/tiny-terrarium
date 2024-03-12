mod components;
mod entities;
pub mod state;
mod systems;

use hecs::World;
use state::State;
use std::time::{Duration, Instant};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 450)
        .title("tiny-terrarium")
        .build();

    let mut world = World::new();
    let mut rng = rand::thread_rng();
    let state = State::new(16, 16);

    // fixed update
    let mut last_update = Instant::now();
    let mut accumulator = 0.0;
    let fixed_update_interval = Duration::from_secs_f32(1.0 / 60.0);

    while !rl.window_should_close() {
        let now = Instant::now();
        let dt = now.duration_since(last_update).as_secs_f32();
        last_update = now;

        systems::input(&mut world, &rl);
        accumulator += dt;
        while accumulator >= dt {
            systems::movement(&mut world, fixed_update_interval.as_secs_f32());
            systems::sporadic_movement(&mut world, &mut rng);
            systems::bounce(&mut world);
            systems::growth(&mut world, fixed_update_interval.as_secs_f32());
            accumulator -= fixed_update_interval.as_secs_f32();
        }
        systems::render(&state, &world, &mut rl, &thread);
    }
}
