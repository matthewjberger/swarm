use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn seek(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);
    let Some(player) = game_world.resources.player_entity else {
        return;
    };
    let Some(player_position) = engine_position(world, game_world, player) else {
        return;
    };

    let mut updates: Vec<(Entity, Vec3)> = Vec::new();
    for enemy in game_world.query_entities(SEEKER | VELOCITY | ENGINE_ENTITY) {
        let (Some(seeker), Some(velocity), Some(enemy_position)) = (
            game_world.get_seeker(enemy).copied(),
            game_world.get_velocity(enemy).copied(),
            engine_position(world, game_world, enemy),
        ) else {
            continue;
        };

        let mut to_player = player_position - enemy_position;
        to_player.y = 0.0;
        if to_player.magnitude_squared() < 1.0e-6 {
            continue;
        }

        let desired = to_player.normalize() * seeker.max_speed;
        let steering = desired - velocity.0;
        let max_delta = seeker.acceleration * delta;
        let applied = if steering.magnitude() > max_delta {
            steering.normalize() * max_delta
        } else {
            steering
        };
        let mut next = velocity.0 + applied;
        if next.magnitude() > seeker.max_speed {
            next = next.normalize() * seeker.max_speed;
        }
        updates.push((enemy, next));
    }

    for (enemy, next) in updates {
        if let Some(velocity) = game_world.get_velocity_mut(enemy) {
            velocity.0 = next;
        }
    }
}
