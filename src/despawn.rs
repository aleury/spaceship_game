use bevy::prelude::*;

use crate::{health::Health, schedule::InGameSet, state::GameState};

const DESPAWN_DISTANCE: f32 = 100.0;

pub struct DepsawnPlugin;

impl Plugin for DepsawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (despawn_far_away_entities, despawn_dead_entities).in_set(InGameSet::DespawnEntities),
        )
        .add_systems(OnEnter(GameState::GameOver), despawn_all_entities);
    }
}

fn despawn_far_away_entities(mut commands: Commands, query: Query<(Entity, &GlobalTransform)>) {
    for (entity, transform) in &query {
        let distance = transform.translation().distance(Vec3::ZERO);

        // Entity is far away from the camera's viewport origin, i.e. (0,0,0).
        if distance > DESPAWN_DISTANCE {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_dead_entities(mut commands: Commands, query: Query<(Entity, &Health)>) {
    for (entity, health) in &query {
        if health.value <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn despawn_all_entities(mut commands: Commands, query: Query<Entity, With<Health>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}
