use crate::ecs::*;
use nightshade_api::prelude::*;

pub fn hud(world: &mut World, game_world: &mut GameWorld) {
    let Some(text) = game_world.resources.score_text else {
        return;
    };
    let score = game_world.resources.score;
    let health = game_world
        .resources
        .player_entity
        .and_then(|player| game_world.get_health(player))
        .map(|health| health.current.max(0.0))
        .unwrap_or(0.0);
    set_text(world, text, &format!("Score {score}    Health {health:.0}"));
}
