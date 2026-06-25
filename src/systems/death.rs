use crate::ecs::*;
use crate::spawn::spawn_debris;
use nightshade_api::prelude::*;

pub fn death(world: &mut World, game_world: &mut GameWorld) {
    let mut casualties: Vec<(Entity, Vec3, Entity, bool)> = Vec::new();
    game_world.for_each(
        HEALTH | POSITION | ENGINE_ENTITY,
        0,
        |entity, table, index| {
            if table.health[index].current <= 0.0 {
                casualties.push((
                    entity,
                    table.position[index].0,
                    table.engine_entity[index].0,
                    table.mask & PLAYER != 0,
                ));
            }
        },
    );
    if casualties.is_empty() {
        return;
    }

    let mut player_died = false;
    for (entity, position, engine, is_player) in casualties {
        if is_player {
            player_died = true;
            continue;
        }
        game_world.resources.score += 1;
        despawn(world, engine);
        game_world.despawn_entities(&[entity]);
        spawn_debris(world, game_world, position);
    }

    if player_died {
        reset_game(world, game_world);
    }
}

fn reset_game(world: &mut World, game_world: &mut GameWorld) {
    game_world.resources.score = 0;
    game_world.resources.elapsed = 0.0;
    game_world.resources.spawn_timer = SPAWN_INTERVAL_BASE;

    let mut removed: Vec<(Entity, Entity)> = Vec::new();
    game_world.for_each(ENGINE_ENTITY, 0, |entity, table, index| {
        if table.mask & PLAYER == 0 {
            removed.push((entity, table.engine_entity[index].0));
        }
    });
    for (entity, engine) in removed {
        despawn(world, engine);
        game_world.despawn_entities(&[entity]);
    }

    let Some(player) = game_world.resources.player_entity else {
        return;
    };
    if let Some(health) = game_world.get_health_mut(player) {
        health.current = health.max;
    }
    if let Some(velocity) = game_world.get_velocity_mut(player) {
        velocity.0 = Vec3::zeros();
    }
    if let Some(position) = game_world.get_position_mut(player) {
        position.0 = vec3(0.0, PLAYER_Y, 0.0);
    }
}
