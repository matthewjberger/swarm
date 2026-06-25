use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn spin(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);

    let mut rotations: Vec<(Entity, Vec3, f32)> = Vec::new();
    for entity in game_world.query_entities(SPIN | ENGINE_ENTITY) {
        if let (Some(spin), Some(engine)) = (
            game_world.get_spin(entity).copied(),
            game_world.get_engine_entity(entity).copied(),
        ) {
            rotations.push((engine.0, spin.axis, spin.speed * delta));
        }
    }

    for (engine, axis, radians) in rotations {
        rotate(world, engine, axis, radians);
    }
}
