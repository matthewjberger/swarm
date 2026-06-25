mod ecs;
mod setup;
mod spawn;
mod systems;

use nightshade_api::prelude::*;
use systems::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run!(
        setup::setup,
        enemy_spawning,
        player_input,
        seek,
        integrate_motion,
        collision,
        lifetime,
        death,
        spin,
        render_sync,
        hud,
    )
}
