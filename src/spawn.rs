use crate::ecs::*;
use nightshade_api::nightshade::ecs::sync::EngineEntity;
use nightshade_api::prelude::*;

pub fn spawn_player(world: &mut World, game_world: &mut GameWorld) -> Entity {
    let origin = vec3(0.0, PLAYER_Y, 0.0);
    let render = spawn_object(
        world,
        Object {
            shape: Shape::Cube,
            position: origin,
            scale: Vec3::repeat(PLAYER_SIZE),
            color: PLAYER_COLOR,
            ..Object::default()
        },
    );

    let entity = game_world.spawn_entities(
        PLAYER | POSITION | VELOCITY | HEALTH | COLLIDER | FACTION | CONFINED | ENGINE_ENTITY,
        1,
    )[0];
    game_world.set_engine_entity(entity, EngineEntity(render));
    game_world.set_position(entity, Position(origin));
    game_world.set_velocity(entity, Velocity(Vec3::zeros()));
    game_world.set_health(
        entity,
        Health {
            current: PLAYER_MAX_HEALTH,
            max: PLAYER_MAX_HEALTH,
        },
    );
    game_world.set_collider(entity, Collider(PLAYER_RADIUS));
    game_world.set_faction(entity, Faction(Team::Player));
    game_world.set_confined(entity, Confined(ARENA_HALF));
    game_world.set_player(entity, Player(0.0));
    entity
}

pub fn spawn_enemy(world: &mut World, game_world: &mut GameWorld, origin: Vec3, max_speed: f32) {
    let render = spawn_object(
        world,
        Object {
            shape: Shape::Sphere,
            position: origin,
            scale: Vec3::repeat(ENEMY_SIZE),
            color: ENEMY_COLOR,
            ..Object::default()
        },
    );

    let entity = game_world.spawn_entities(
        SEEKER
            | POSITION
            | VELOCITY
            | HEALTH
            | COLLIDER
            | FACTION
            | CONFINED
            | SPIN
            | ENGINE_ENTITY,
        1,
    )[0];
    game_world.set_engine_entity(entity, EngineEntity(render));
    game_world.set_position(entity, Position(origin));
    game_world.set_velocity(entity, Velocity(Vec3::zeros()));
    game_world.set_health(
        entity,
        Health {
            current: ENEMY_MAX_HEALTH,
            max: ENEMY_MAX_HEALTH,
        },
    );
    game_world.set_collider(entity, Collider(ENEMY_RADIUS));
    game_world.set_faction(entity, Faction(Team::Enemy));
    game_world.set_confined(entity, Confined(ARENA_HALF));
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
    origin: Vec3,
    velocity: Vec3,
) {
    let render = spawn_object(
        world,
        Object {
            shape: Shape::Sphere,
            position: origin,
            scale: Vec3::repeat(BULLET_SIZE),
            color: BULLET_COLOR,
            ..Object::default()
        },
    );

    let entity = game_world.spawn_entities(
        POSITION | VELOCITY | DAMAGE | COLLIDER | FACTION | LIFETIME | ENGINE_ENTITY,
        1,
    )[0];
    game_world.set_engine_entity(entity, EngineEntity(render));
    game_world.set_position(entity, Position(origin));
    game_world.set_velocity(entity, Velocity(velocity));
    game_world.set_damage(entity, Damage(BULLET_DAMAGE));
    game_world.set_collider(entity, Collider(BULLET_RADIUS));
    game_world.set_faction(entity, Faction(Team::Player));
    game_world.set_lifetime(entity, Lifetime(BULLET_LIFETIME));
}

pub fn spawn_debris(world: &mut World, game_world: &mut GameWorld, origin: Vec3) {
    let render = spawn_object(
        world,
        Object {
            shape: Shape::Cube,
            position: origin,
            scale: Vec3::repeat(DEBRIS_SIZE),
            color: DEBRIS_COLOR,
            ..Object::default()
        },
    );

    let entity = game_world.spawn_entities(POSITION | SPIN | LIFETIME | ENGINE_ENTITY, 1)[0];
    game_world.set_engine_entity(entity, EngineEntity(render));
    game_world.set_position(entity, Position(origin));
    game_world.set_spin(
        entity,
        Spin {
            axis: Vec3::y(),
            speed: DEBRIS_SPIN,
        },
    );
    game_world.set_lifetime(entity, Lifetime(DEBRIS_LIFETIME));
}
