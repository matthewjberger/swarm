use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn render_sync(world: &mut World, game_world: &mut GameWorld) {
    game_world.for_each(POSITION | ENGINE_ENTITY, 0, |_, table, index| {
        set_position(world, table.engine_entity[index].0, table.position[index].0);
    });
}
