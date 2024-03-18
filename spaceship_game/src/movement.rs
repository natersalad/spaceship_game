use bevy::prelude::*;
#[derive(Component, Debug)]
pub struct Velocity {
    pub value: Vec3,
}
pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_postion);
    }
}

//simple update system that updates the position with velocity
fn update_postion(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation += velocity.value * time.delta_seconds();
    }
}
