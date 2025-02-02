use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Collider {
    pub radius: f32,
    pub colliding_entities: Vec<Entity>,
}

impl Collider {
    pub fn new(radius: f32) -> Self {
        Self {
            radius,
            colliding_entities: vec![],
        }
    }
}

pub struct CollisionDetectionPlugin;

impl Plugin for CollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collision_detection);
    }
}

fn collision_detection(mut query: Query<(Entity, &GlobalTransform, &mut Collider)>) {
    let mut colliding_entities: HashMap<Entity, Vec<Entity>> = HashMap::new();

    // First phase: Detect collisions.
    for (entity_a, transform_a, collider_a) in &query {
        for (entity_b, transform_b, collider_b) in &query {
            if entity_a == entity_b {
                continue;
            }
            let distance = transform_a
                .translation()
                .distance(transform_b.translation());
            if distance < collider_a.radius + collider_b.radius {
                colliding_entities
                    .entry(entity_a)
                    .or_default()
                    .push(entity_b);
            }
        }
    }

    // Second phase: Update colliders
    for (entity, _, mut collider) in &mut query {
        collider.colliding_entities.clear();
        if let Some(collisions) = colliding_entities.get(&entity) {
            collider.colliding_entities.extend_from_slice(collisions);
        }
    }
}
