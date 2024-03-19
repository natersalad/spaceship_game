mod debug;
mod spaceship;
mod movement;
mod camera;
mod asteroids;
mod asset_loader;
mod collision_detection;
mod despawn;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use collision_detection::CollisionDetectionPlugin;
use despawn::DespawnPlugin;
use spaceship::SpaceshipPlugin;
use movement::MovementPlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
        //Bevy Builtins for lighting
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 1000.0,
        })
        .add_plugins(DefaultPlugins)
        //User created Plugins
        .add_plugins(DespawnPlugin)
        .add_plugins(CollisionDetectionPlugin)
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(MovementPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(AsteroidPlugin)
        .run();
}
