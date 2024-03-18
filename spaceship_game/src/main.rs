mod debug;
mod spaceship;
mod movement;
mod camera;

use bevy::prelude::*;
use debug::DebugPlugin;
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
        .add_plugins(CameraPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MovementPlugin)
        .run();
}
