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
    for entity in game_world.query_entities(COLLIDER | FACTION | ENGINE_ENTITY) {
        let (Some(collider), Some(faction), Some(position)) = (
            game_world.get_collider(entity).copied(),
            game_world.get_faction(entity).copied(),
            engine_position(world, game_world, entity),
        ) else {
            continue;
        };
        let kind = if game_world.get_player(entity).is_some() {
            Kind::Player
        } else if game_world.get_damage(entity).is_some() {
            Kind::Projectile
        } else {
            Kind::Enemy
        };
        let damage = game_world
            .get_damage(entity)
            .map(|damage| damage.amount)
            .unwrap_or(0.0);
        bodies.push(Body {
            entity,
            position,
            radius: collider.radius,
            team: faction.0,
            kind,
            damage,
        });
    }

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
                (Kind::Projectile, Kind::Enemy)
                    if !destroyed.contains(&a.entity) && !destroyed.contains(&b.entity) =>
                {
                    hits.push((b.entity, a.damage));
                    destroyed.insert(a.entity);
                }
                (Kind::Enemy, Kind::Projectile)
                    if !destroyed.contains(&a.entity) && !destroyed.contains(&b.entity) =>
                {
                    hits.push((a.entity, b.damage));
                    destroyed.insert(b.entity);
                }
                (Kind::Enemy, Kind::Player) if !destroyed.contains(&a.entity) => {
                    hits.push((b.entity, ENEMY_CONTACT_DAMAGE));
                    destroyed.insert(a.entity);
                }
                (Kind::Player, Kind::Enemy) if !destroyed.contains(&b.entity) => {
                    hits.push((a.entity, ENEMY_CONTACT_DAMAGE));
                    destroyed.insert(b.entity);
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
