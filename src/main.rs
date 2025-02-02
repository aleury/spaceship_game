#![allow(
    dead_code,
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions
)]
mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod despawn;
mod movement;
mod spaceship;

use bevy::prelude::*;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidsPlugin;
use camera::CameraPlugin;
use collision_detection::CollisionDetectionPlugin;
use debug::DebugPlugin;
use despawn::DepsawnPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

const BG_COLOR: Color = Color::rgb(0.1, 0.0, 0.15);

fn main() {
    App::new()
        // Bevy built-ins.
        .insert_resource(ClearColor(BG_COLOR))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins(DefaultPlugins)
        // User defined plugins.
        .add_plugins(CameraPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidsPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(DepsawnPlugin)
        .add_plugins(DebugPlugin)
        .run();
}
