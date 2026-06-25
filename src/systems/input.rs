use crate::ecs::*;
use crate::spawn::spawn_projectile;
use nightshade_api::prelude::*;

pub fn player_input(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);
    let Some(player) = game_world.resources.player_entity else {
        return;
    };
    let Some(player_pos) = player_position(game_world) else {
        return;
    };

    let direction = wasd(world);
    if let Some(velocity) = game_world.get_velocity_mut(player) {
        velocity.0 = direction * PLAYER_SPEED;
    }

    let ready = match game_world.get_player_mut(player) {
        Some(state) => {
            state.0 -= delta;
            if state.0 <= 0.0 {
                state.0 += PLAYER_FIRE_INTERVAL;
                true
            } else {
                false
            }
        }
        None => false,
    };
    if !ready {
        return;
    }

    let mut best_distance = f32::INFINITY;
    let mut target = None;
    game_world.for_each(SEEKER | POSITION, 0, |_, table, index| {
        let position = table.position[index].0;
        let distance = (position - player_pos).magnitude_squared();
        if distance < best_distance {
            best_distance = distance;
            target = Some(position);
        }
    });
    let Some(target) = target else {
        return;
    };

    let mut aim = target - player_pos;
    aim.y = 0.0;
    if aim.magnitude_squared() < 1.0e-6 {
        return;
    }
    let aim = aim.normalize();
    let spawn_at = vec3(player_pos.x, BULLET_Y, player_pos.z) + aim * PLAYER_RADIUS;
    spawn_projectile(world, game_world, spawn_at, aim * BULLET_SPEED);
}
