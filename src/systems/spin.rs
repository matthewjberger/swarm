use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn spin(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);
    game_world.for_each(SPIN | ENGINE_ENTITY, 0, |_, table, index| {
        let spin = table.spin[index];
        rotate(
            world,
            table.engine_entity[index].0,
            spin.axis,
            spin.speed * delta,
        );
    });
}
