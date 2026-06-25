# swarm

A small top-down arena survival game built on [nightshade-api](https://crates.io/crates/nightshade-api). You are a cube, red spheres home in on you, and you auto-fire at the nearest one. Stay alive and rack up kills. Runs natively and in the browser from the same code.

## Run

```sh
just run       # native window
just run-web   # serve in the browser via trunk
```

`just run-web` needs the wasm target and trunk, which `just init-web` installs.

## How it works

The game uses the dual-world ECS pattern. The engine `World` owns rendering, transforms, and the camera. A second `freecs::ecs!` `GameWorld` owns the game logic, and an `EngineEntity` component links each game entity to its render entity. After the game systems run, a sync pass writes positions back into the engine world.

Behavior lives in components, not labels. `Velocity`, `Health`, `Collider`, `Seeker`, `Confined`, `Lifetime`, `Spin`, `Damage`, and `Faction` are plain data, and each system acts on whatever entities carry the components it cares about. One motion system moves the player, the enemies, and the bullets because they all carry `Velocity`. One lifetime system reaps bullets and debris alike. Collision reacts to which components an entity has rather than to any per-kind tag.

- `src/ecs.rs` the `GameWorld`, the components, the tuning constants, and the engine-position helper
- `src/spawn.rs` spawn helpers that build the render entity and the linked game entity together
- `src/setup.rs` camera, floor, player, and HUD, returning the `GameWorld` as the run-loop state
- `src/systems/` one file per system: input, spawning, seek, motion, confine, spin, collision, lifetime, death, hud
- `src/main.rs` `run!(setup, ...systems)`, with the `GameWorld` carried as the run state so the same code runs on native and wasm

## Controls

WASD to move. Firing is automatic at the nearest enemy. Touching an enemy costs health, and running out resets the run.
