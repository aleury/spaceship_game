use bevy::prelude::*;

use crate::collision_detection::Collider;

pub struct MovementPlugin;

#[derive(Debug, Component)]
pub struct Velocity {
    pub value: Vec3,
}

impl Velocity {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Debug, Component)]
pub struct Acceleration {
    pub value: Vec3,
}

impl Acceleration {
    pub fn new(value: Vec3) -> Self {
        Self { value }
    }
}

#[derive(Bundle)]
pub struct MovingObjectBundle {
    pub model: SceneBundle,
    pub velocity: Velocity,
    pub collider: Collider,
    pub acceleration: Acceleration,
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_velocity, update_position));
    }
}

fn update_velocity(time: Res<Time>, mut query: Query<(&Acceleration, &mut Velocity)>) {
    for (acceleration, mut velocity) in &mut query {
        velocity.value += acceleration.value * time.delta_seconds();
    }
}

fn update_position(time: Res<Time>, mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
