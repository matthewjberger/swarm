use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn lifetime(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);

    let mut ticks: Vec<(Entity, f32)> = Vec::new();
    let mut expired: Vec<(Entity, Option<Entity>)> = Vec::new();
    for entity in game_world.query_entities(LIFETIME) {
        let Some(life) = game_world.get_lifetime(entity).copied() else {
            continue;
        };
        let remaining = life.remaining - delta;
        if remaining <= 0.0 {
            let engine = game_world.get_engine_entity(entity).map(|engine| engine.0);
            expired.push((entity, engine));
        } else {
            ticks.push((entity, remaining));
        }
    }

    for (entity, remaining) in ticks {
        if let Some(life) = game_world.get_lifetime_mut(entity) {
            life.remaining = remaining;
        }
    }

    for (entity, engine) in expired {
        if let Some(engine) = engine {
            despawn(world, engine);
        }
        game_world.despawn_entities(&[entity]);
    }
}
