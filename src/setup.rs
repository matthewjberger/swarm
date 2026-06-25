use crate::ecs::*;
use crate::spawn::spawn_player;
use nightshade_api::prelude::*;

pub fn setup(world: &mut World) -> GameWorld {
    fixed_camera(world, vec3(0.0, 26.0, 18.0), vec3(0.0, 0.0, 0.0));
    set_taa(world, false);

    load_texture(
        world,
        FLOOR_TEXTURE,
        include_bytes!("../assets/textures/proto_dark_06.png"),
    );
    let floor = spawn_floor(world, ARENA_HALF + 1.0);
    set_color(world, floor, FLOOR_COLOR);
    set_texture(world, floor, FLOOR_TEXTURE);
    set_texture_tiling(world, floor, FLOOR_TILES);

    let mut game_world = GameWorld::default();
    let player = spawn_player(world, &mut game_world);
    game_world.resources.player_entity = Some(player);
    game_world.resources.rng = RANDOM_SEED;
    game_world.resources.spawn_timer = SPAWN_INTERVAL_BASE;

    let score_text = spawn_text(world, "Score 0    Health 100", ScreenAnchor::TopCenter);
    game_world.resources.score_text = Some(score_text);
    spawn_text(
        world,
        "WASD to move. You auto-fire at the nearest enemy. Stay alive.",
        ScreenAnchor::BottomCenter,
    );

    game_world
}
