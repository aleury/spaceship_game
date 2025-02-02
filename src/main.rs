#![allow(
    dead_code,
    clippy::needless_pass_by_value,
    clippy::module_name_repetitions
)]
mod asset_loader;
mod asteroids;
mod camera;
mod debug;
mod movement;
mod spaceship;

use asset_loader::AssetLoaderPlugin;
use bevy::prelude::*;

use asteroids::AsteroidsPlugin;
use camera::CameraPlugin;
use debug::DebugPlugin;
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
        .add_plugins(DebugPlugin)
        .run();
}
