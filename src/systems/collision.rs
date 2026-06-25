use crate::ecs::*;
use nightshade_api::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Kind {
    Player,
    Enemy,
    Projectile,
}

struct Body {
    entity: Entity,
    position: Vec3,
    radius: f32,
    team: Team,
    kind: Kind,
    damage: f32,
}

pub fn collision(world: &mut World, game_world: &mut GameWorld) {
    let mut bodies: Vec<Body> = Vec::new();
    game_world.for_each(COLLIDER | FACTION | POSITION, 0, |entity, table, index| {
        let kind = if table.mask & PLAYER != 0 {
            Kind::Player
        } else if table.mask & DAMAGE != 0 {
            Kind::Projectile
        } else {
            Kind::Enemy
        };
        let damage = if table.mask & DAMAGE != 0 {
            table.damage[index].0
        } else {
            0.0
        };
        bodies.push(Body {
            entity,
            position: table.position[index].0,
            radius: table.collider[index].0,
            team: table.faction[index].0,
            kind,
            damage,
        });
    });

    let mut hits: Vec<(Entity, f32)> = Vec::new();
    let mut destroyed: HashSet<Entity> = HashSet::new();
    for index_a in 0..bodies.len() {
        for index_b in (index_a + 1)..bodies.len() {
            let a = &bodies[index_a];
            let b = &bodies[index_b];
            if a.team == b.team {
                continue;
            }
            let mut delta = a.position - b.position;
            delta.y = 0.0;
            let reach = a.radius + b.radius;
            if delta.magnitude_squared() > reach * reach {
                continue;
            }

            match (a.kind, b.kind) {
                (Kind::Projectile, Kind::Enemy) | (Kind::Enemy, Kind::Projectile) => {
                    let (projectile, enemy) = if matches!(a.kind, Kind::Projectile) {
                        (a, b)
                    } else {
                        (b, a)
                    };
                    if !destroyed.contains(&projectile.entity) && !destroyed.contains(&enemy.entity)
                    {
                        hits.push((enemy.entity, projectile.damage));
                        destroyed.insert(projectile.entity);
                    }
                }
                (Kind::Enemy, Kind::Player) | (Kind::Player, Kind::Enemy) => {
                    let (enemy, player) = if matches!(a.kind, Kind::Enemy) {
                        (a, b)
                    } else {
                        (b, a)
                    };
                    if !destroyed.contains(&enemy.entity) {
                        hits.push((player.entity, ENEMY_CONTACT_DAMAGE));
                        destroyed.insert(enemy.entity);
                    }
                }
                _ => {}
            }
        }
    }

    for (target, amount) in hits {
        if let Some(health) = game_world.get_health_mut(target) {
            health.current -= amount;
        }
    }

    for entity in destroyed {
        if let Some(engine) = game_world.get_engine_entity(entity).copied() {
            despawn(world, engine.0);
        }
        game_world.despawn_entities(&[entity]);
    }
}
