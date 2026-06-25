use crate::ecs::*;
use crate::spawn::spawn_debris;
use nightshade_api::prelude::*;

pub fn death(world: &mut World, game_world: &mut GameWorld) {
    let mut casualties: Vec<(Entity, Vec3, bool)> = Vec::new();
    for entity in game_world.query_entities(HEALTH | ENGINE_ENTITY) {
        let Some(health) = game_world.get_health(entity).copied() else {
            continue;
        };
        if health.current > 0.0 {
            continue;
        }
        let player = game_world.get_player(entity).is_some();
        let position = engine_position(world, game_world, entity).unwrap_or_else(Vec3::zeros);
        casualties.push((entity, position, player));
    }
    if casualties.is_empty() {
        return;
    }

    let mut player_died = false;
    for (entity, position, player) in casualties {
        if player {
            player_died = true;
            continue;
        }
        game_world.resources.score += 1;
        if let Some(engine) = game_world.get_engine_entity(entity).copied() {
            despawn(world, engine.0);
        }
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

    let mut removed: Vec<(Entity, Option<Entity>)> = Vec::new();
    for entity in game_world.query_entities(ENGINE_ENTITY) {
        if game_world.get_player(entity).is_some() {
            continue;
        }
        let engine = game_world.get_engine_entity(entity).map(|engine| engine.0);
        removed.push((entity, engine));
    }
    for (entity, engine) in removed {
        if let Some(engine) = engine {
            despawn(world, engine);
        }
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
    if let Some(engine) = game_world.get_engine_entity(player).copied() {
        set_position(world, engine.0, vec3(0.0, PLAYER_Y, 0.0));
    }
}
