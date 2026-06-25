use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn integrate_motion(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);
    game_world.for_each_mut(POSITION | VELOCITY, 0, |_, table, index| {
        let velocity = table.velocity[index].0;
        let mut next = table.position[index].0;
        next.x += velocity.x * delta;
        next.z += velocity.z * delta;
        if table.mask & CONFINED != 0 {
            let half = table.confined[index].0;
            next.x = next.x.clamp(-half, half);
            next.z = next.z.clamp(-half, half);
        }
        table.position[index].0 = next;
    });
}
