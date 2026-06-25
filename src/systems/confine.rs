use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn confine(world: &mut World, game_world: &mut GameWorld) {
    let mut clamps: Vec<(Entity, Vec3)> = Vec::new();
    for entity in game_world.query_entities(CONFINED | ENGINE_ENTITY) {
        let (Some(confined), Some(current)) = (
            game_world.get_confined(entity).copied(),
            engine_position(world, game_world, entity),
        ) else {
            continue;
        };
        let half = confined.half_extent;
        let outside =
            current.x < -half || current.x > half || current.z < -half || current.z > half;
        if outside {
            let clamped = vec3(
                current.x.clamp(-half, half),
                current.y,
                current.z.clamp(-half, half),
            );
            clamps.push((entity, clamped));
        }
    }

    for (entity, clamped) in clamps {
        if let Some(engine) = game_world.get_engine_entity(entity).copied() {
            set_position(world, engine.0, clamped);
        }
    }
}
