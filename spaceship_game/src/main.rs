use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_spaceship)
        .add_systems(Update, (update_postion, position_info))
        .run();
}

//Create a Position Component
#[derive(Component, Debug)]
struct Position {
    x: f32,
    y: f32,
}

//Create a Velocity Component
#[derive(Component, Debug)]
struct Velocity {
    x: f32,
    y: f32,
}

//Create an initialize system for the spaceship
fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((Position { x: 0.0, y: 0.0 }, Velocity { x: 1.0, y: 1.0 }));
}

//simple update system that updates the position with velocity
fn update_postion(mut query: Query<(&Velocity, &mut Position)>) {
    for (velocity, mut position) in query.iter_mut() {
        position.x += velocity.x;
        position.y += velocity.y;
    }
}

//Info function that will tell me how the position is changed
fn position_info(query: Query<&Position>) {
    for position in query.iter() {
        info!("x position: {:?}, y position: {:?}", position.x, position.y);
    }
}
