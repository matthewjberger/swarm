use crate::ecs::*;
use crate::spawn::spawn_enemy;
use nightshade_api::prelude::*;

pub fn enemy_spawning(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);
    game_world.resources.elapsed += delta;
    game_world.resources.spawn_timer -= delta;
    if game_world.resources.spawn_timer > 0.0 {
        return;
    }

    let elapsed = game_world.resources.elapsed;
    let interval = (SPAWN_INTERVAL_BASE - elapsed * 0.01).max(SPAWN_INTERVAL_MIN);
    game_world.resources.spawn_timer = interval;

    let angle = next_random(&mut game_world.resources.rng) * std::f32::consts::TAU;
    let spawn_at = vec3(
        angle.cos() * SPAWN_RADIUS,
        ENEMY_Y,
        angle.sin() * SPAWN_RADIUS,
    );
    let max_speed = (ENEMY_BASE_SPEED + elapsed * 0.03).min(ENEMY_MAX_SPEED_CAP);
    spawn_enemy(world, game_world, spawn_at, max_speed);
}
