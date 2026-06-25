use crate::ecs::*;
use nightshade_api::nightshade::ecs::sync::EngineEntity;
use nightshade_api::prelude::*;

pub fn spawn_player(world: &mut World, game_world: &mut GameWorld) -> Entity {
    let render = spawn_cube(world, vec3(0.0, PLAYER_Y, 0.0));
    set_scale(world, render, vec3(PLAYER_SIZE, PLAYER_SIZE, PLAYER_SIZE));
    set_color(world, render, PLAYER_COLOR);

    let entity = game_world.spawn_entities(
        PLAYER | VELOCITY | HEALTH | COLLIDER | FACTION | CONFINED | ENGINE_ENTITY,
        1,
    )[0];
    game_world.set_engine_entity(entity, EngineEntity(render));
    game_world.set_velocity(entity, Velocity(Vec3::zeros()));
    game_world.set_health(
        entity,
        Health {
            current: PLAYER_MAX_HEALTH,
            max: PLAYER_MAX_HEALTH,
        },
    );
    game_world.set_collider(
        entity,
        Collider {
            radius: PLAYER_RADIUS,
        },
    );
    game_world.set_faction(entity, Faction(Team::Player));
    game_world.set_confined(
        entity,
        Confined {
            half_extent: ARENA_HALF,
        },
    );
    game_world.set_player(entity, Player { fire_cooldown: 0.0 });
    entity
}

pub fn spawn_enemy(world: &mut World, game_world: &mut GameWorld, position: Vec3, max_speed: f32) {
    let render = spawn_sphere(world, position);
    set_scale(world, render, vec3(ENEMY_SIZE, ENEMY_SIZE, ENEMY_SIZE));
    set_color(world, render, ENEMY_COLOR);

    let entity = game_world.spawn_entities(
        SEEKER | VELOCITY | HEALTH | COLLIDER | FACTION | CONFINED | SPIN | ENGINE_ENTITY,
        1,
    )[0];
    game_world.set_engine_entity(entity, EngineEntity(render));
    game_world.set_velocity(entity, Velocity(Vec3::zeros()));
    game_world.set_health(
        entity,
        Health {
            current: ENEMY_MAX_HEALTH,
            max: ENEMY_MAX_HEALTH,
        },
    );
    game_world.set_collider(
        entity,
        Collider {
            radius: ENEMY_RADIUS,
        },
    );
    game_world.set_faction(entity, Faction(Team::Enemy));
    game_world.set_confined(
        entity,
        Confined {
            half_extent: ARENA_HALF,
        },
    );
    game_world.set_seeker(
        entity,
        Seeker {
            acceleration: ENEMY_ACCELERATION,
            max_speed,
        },
    );
    game_world.set_spin(
        entity,
        Spin {
            axis: Vec3::y(),
            speed: ENEMY_SPIN,
        },
    );
}

pub fn spawn_projectile(
    world: &mut World,
    game_world: &mut GameWorld,
    position: Vec3,
    velocity: Vec3,
) {
    let render = spawn_sphere(world, position);
    set_scale(world, render, vec3(BULLET_SIZE, BULLET_SIZE, BULLET_SIZE));
    set_color(world, render, BULLET_COLOR);

    let entity = game_world.spawn_entities(
        VELOCITY | DAMAGE | COLLIDER | FACTION | LIFETIME | ENGINE_ENTITY,
        1,
    )[0];
    game_world.set_engine_entity(entity, EngineEntity(render));
    game_world.set_velocity(entity, Velocity(velocity));
    game_world.set_damage(
        entity,
        Damage {
            amount: BULLET_DAMAGE,
        },
    );
    game_world.set_collider(
        entity,
        Collider {
            radius: BULLET_RADIUS,
        },
    );
    game_world.set_faction(entity, Faction(Team::Player));
    game_world.set_lifetime(
        entity,
        Lifetime {
            remaining: BULLET_LIFETIME,
        },
    );
}

pub fn spawn_debris(world: &mut World, game_world: &mut GameWorld, position: Vec3) {
    let render = spawn_cube(world, position);
    set_scale(world, render, vec3(DEBRIS_SIZE, DEBRIS_SIZE, DEBRIS_SIZE));
    set_color(world, render, DEBRIS_COLOR);

    let entity = game_world.spawn_entities(SPIN | LIFETIME | ENGINE_ENTITY, 1)[0];
    game_world.set_engine_entity(entity, EngineEntity(render));
    game_world.set_spin(
        entity,
        Spin {
            axis: Vec3::y(),
            speed: DEBRIS_SPIN,
        },
    );
    game_world.set_lifetime(
        entity,
        Lifetime {
            remaining: DEBRIS_LIFETIME,
        },
    );
}
