use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn seek(world: &mut World, game_world: &mut GameWorld) {
    let delta = delta_time(world);
    let Some(target) = player_position(game_world) else {
        return;
    };

    game_world.for_each_mut(SEEKER | VELOCITY | POSITION, 0, |_, table, index| {
        let mut to_target = target - table.position[index].0;
        to_target.y = 0.0;
        if to_target.magnitude_squared() < 1.0e-6 {
            return;
        }

        let seeker = table.seeker[index];
        let velocity = table.velocity[index].0;
        let desired = to_target.normalize() * seeker.max_speed;
        let steering = desired - velocity;
        let max_delta = seeker.acceleration * delta;
        let applied = if steering.magnitude() > max_delta {
            steering.normalize() * max_delta
        } else {
            steering
        };

        let mut next = velocity + applied;
        if next.magnitude() > seeker.max_speed {
            next = next.normalize() * seeker.max_speed;
        }
        table.velocity[index].0 = next;
    });
}
