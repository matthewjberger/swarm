use nightshade_api::nightshade::ecs::sync::EngineEntity;
use nightshade_api::nightshade::prelude::freecs;
use nightshade_api::prelude::*;

pub use freecs::Entity;

freecs::ecs! {
    GameWorld {
        engine_entity: EngineEntity => ENGINE_ENTITY,
        position: Position => POSITION,
        velocity: Velocity => VELOCITY,
        health: Health => HEALTH,
        collider: Collider => COLLIDER,
        faction: Faction => FACTION,
        seeker: Seeker => SEEKER,
        confined: Confined => CONFINED,
        lifetime: Lifetime => LIFETIME,
        spin: Spin => SPIN,
        damage: Damage => DAMAGE,
        player: Player => PLAYER,
    }
    GameResources {
        player_entity: Option<Entity>,
        score_text: Option<Entity>,
        score: u32,
        elapsed: f32,
        spawn_timer: f32,
        rng: u64,
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Team {
    #[default]
    Player,
    Enemy,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Position(pub Vec3);

#[derive(Debug, Clone, Copy, Default)]
pub struct Velocity(pub Vec3);

#[derive(Debug, Clone, Copy, Default)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Collider {
    pub radius: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Faction(pub Team);

#[derive(Debug, Clone, Copy, Default)]
pub struct Seeker {
    pub acceleration: f32,
    pub max_speed: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Confined {
    pub half_extent: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Lifetime {
    pub remaining: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Spin {
    pub axis: Vec3,
    pub speed: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Damage {
    pub amount: f32,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Player {
    pub fire_cooldown: f32,
}

pub const ARENA_HALF: f32 = 14.0;
pub const SPAWN_RADIUS: f32 = 13.0;

pub const PLAYER_Y: f32 = 0.5;
pub const ENEMY_Y: f32 = 0.6;
pub const BULLET_Y: f32 = 0.5;

pub const PLAYER_SIZE: f32 = 0.7;
pub const ENEMY_SIZE: f32 = 0.7;
pub const BULLET_SIZE: f32 = 0.28;
pub const DEBRIS_SIZE: f32 = 0.4;

pub const PLAYER_RADIUS: f32 = 0.45;
pub const ENEMY_RADIUS: f32 = 0.55;
pub const BULLET_RADIUS: f32 = 0.2;

pub const PLAYER_SPEED: f32 = 8.0;
pub const PLAYER_MAX_HEALTH: f32 = 100.0;
pub const PLAYER_FIRE_INTERVAL: f32 = 0.16;

pub const BULLET_SPEED: f32 = 20.0;
pub const BULLET_DAMAGE: f32 = 40.0;
pub const BULLET_LIFETIME: f32 = 1.4;

pub const ENEMY_MAX_HEALTH: f32 = 100.0;
pub const ENEMY_ACCELERATION: f32 = 14.0;
pub const ENEMY_BASE_SPEED: f32 = 2.6;
pub const ENEMY_MAX_SPEED_CAP: f32 = 5.5;
pub const ENEMY_CONTACT_DAMAGE: f32 = 16.0;
pub const ENEMY_SPIN: f32 = 2.0;

pub const SPAWN_INTERVAL_BASE: f32 = 1.3;
pub const SPAWN_INTERVAL_MIN: f32 = 0.35;

pub const DEBRIS_LIFETIME: f32 = 0.45;
pub const DEBRIS_SPIN: f32 = 12.0;

pub const PLAYER_COLOR: [f32; 4] = [0.25, 0.55, 1.0, 1.0];
pub const ENEMY_COLOR: [f32; 4] = [1.0, 0.3, 0.25, 1.0];
pub const BULLET_COLOR: [f32; 4] = [1.0, 0.95, 0.4, 1.0];
pub const DEBRIS_COLOR: [f32; 4] = [1.0, 0.6, 0.2, 1.0];
pub const FLOOR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const FLOOR_TILES: f32 = 6.0;
pub const FLOOR_TEXTURE: &str = "proto_dark_06";
pub const RANDOM_SEED: u64 = 0x9E37_79B9_7F4A_7C15;

pub fn next_random(state: &mut u64) -> f32 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    (*state >> 11) as f32 / (1u64 << 53) as f32
}

pub fn player_position(game_world: &GameWorld) -> Option<Vec3> {
    let player = game_world.resources.player_entity?;
    Some(game_world.get_position(player)?.0)
}
