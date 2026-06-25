use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn lifetime(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);

    let mut expired: Vec<(Entity, Entity)> = Vec::new();
    game_world.for_each_mut(LIFETIME | ENGINE_ENTITY, 0, |entity, table, index| {
        let remaining = table.lifetime[index].0 - delta;
        table.lifetime[index].0 = remaining;
        if remaining <= 0.0 {
            expired.push((entity, table.engine_entity[index].0));
        }
    });

    for (entity, engine) in expired {
        despawn(world, engine);
        game_world.despawn_entities(&[entity]);
    }
}
