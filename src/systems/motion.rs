use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn integrate_motion(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);

    let mut moves: Vec<(Entity, Vec3)> = Vec::new();
    for entity in game_world.query_entities(VELOCITY | ENGINE_ENTITY) {
        let (Some(velocity), Some(current)) = (
            game_world.get_velocity(entity).copied(),
            engine_position(world, game_world, entity),
        ) else {
            continue;
        };
        let next = current + vec3(velocity.0.x, 0.0, velocity.0.z) * delta;
        moves.push((entity, next));
    }

    for (entity, next) in moves {
        if let Some(engine) = game_world.get_engine_entity(entity).copied() {
            set_position(world, engine.0, next);
        }
    }
}
