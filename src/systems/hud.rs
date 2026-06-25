use crate::ecs::*;
use nightshade_api::prelude::*;

const PANEL_BG: [f32; 4] = [0.05, 0.06, 0.09, 0.86];
const HINT_BG: [f32; 4] = [0.05, 0.06, 0.09, 0.78];
const LABEL_COLOR: [f32; 4] = [0.62, 0.70, 0.85, 1.0];
const VALUE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const HINT_COLOR: [f32; 4] = [0.82, 0.88, 1.0, 1.0];
const TRACK_COLOR: [f32; 4] = [0.16, 0.17, 0.22, 1.0];
const HEALTH_GREEN: [f32; 4] = [0.30, 0.82, 0.38, 1.0];
const HEALTH_AMBER: [f32; 4] = [0.95, 0.75, 0.20, 1.0];
const HEALTH_RED: [f32; 4] = [0.90, 0.27, 0.22, 1.0];

const BAR_X: f32 = 16.0;
const BAR_Y: f32 = 72.0;
const BAR_WIDTH: f32 = 232.0;
const BAR_HEIGHT: f32 = 16.0;

pub fn build_hud(world: &mut World, game_world: &mut GameWorld) {
    let panel = spawn_panel_at(
        world,
        ScreenAnchor::TopLeft,
        vec2(20.0, 20.0),
        vec2(264.0, 104.0),
        PANEL_BG,
    );
    panel_text(
        world,
        panel,
        "SCORE",
        [16.0, 12.0, 90.0, 24.0],
        15.0,
        LABEL_COLOR,
        TextAlignment::Left,
    );
    let score_value = panel_text(
        world,
        panel,
        "0",
        [16.0, 10.0, 232.0, 26.0],
        22.0,
        VALUE_COLOR,
        TextAlignment::Right,
    );
    panel_text(
        world,
        panel,
        "HEALTH",
        [16.0, 44.0, 120.0, 20.0],
        13.0,
        LABEL_COLOR,
        TextAlignment::Left,
    );
    panel_box(
        world,
        panel,
        vec2(BAR_X, BAR_Y),
        vec2(BAR_WIDTH, BAR_HEIGHT),
        TRACK_COLOR,
    );
    let health_fill = panel_box(
        world,
        panel,
        vec2(BAR_X, BAR_Y),
        vec2(BAR_WIDTH, BAR_HEIGHT),
        HEALTH_GREEN,
    );
    let health_text = panel_text(
        world,
        panel,
        "100 / 100",
        [BAR_X, BAR_Y, BAR_WIDTH, BAR_HEIGHT],
        12.0,
        VALUE_COLOR,
        TextAlignment::Center,
    );

    let hint = spawn_panel_at(
        world,
        ScreenAnchor::BottomCenter,
        vec2(0.0, -20.0),
        vec2(560.0, 40.0),
        HINT_BG,
    );
    panel_text(
        world,
        hint,
        "WASD to move    auto-fire at the nearest enemy    survive",
        [0.0, 0.0, 560.0, 40.0],
        15.0,
        HINT_COLOR,
        TextAlignment::Center,
    );

    game_world.resources.hud = Hud {
        score_value: Some(score_value),
        health_fill: Some(health_fill),
        health_text: Some(health_text),
    };
}

pub fn hud(world: &mut World, game_world: &mut GameWorld) {
    let hud = game_world.resources.hud;
    let score = game_world.resources.score;
    let (current, max) = game_world
        .resources
        .player_entity
        .and_then(|player| game_world.get_health(player))
        .map(|health| (health.current.max(0.0), health.max))
        .unwrap_or((0.0, PLAYER_MAX_HEALTH));
    let fraction = if max > 0.0 {
        (current / max).clamp(0.0, 1.0)
    } else {
        0.0
    };

    if let Some(score_value) = hud.score_value {
        set_panel_text(world, score_value, &format!("{score}"));
    }
    if let Some(health_fill) = hud.health_fill {
        set_panel_rect(
            world,
            health_fill,
            vec2(BAR_X, BAR_Y),
            vec2(BAR_WIDTH * fraction, BAR_HEIGHT),
        );
        let color = if fraction > 0.5 {
            HEALTH_GREEN
        } else if fraction > 0.25 {
            HEALTH_AMBER
        } else {
            HEALTH_RED
        };
        set_panel_color(world, health_fill, color);
    }
    if let Some(health_text) = hud.health_text {
        set_panel_text(world, health_text, &format!("{current:.0} / {max:.0}"));
    }
}
