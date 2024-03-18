use bevy::prelude::*;
pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, position_info);
    }
}

//Info function that will tell me how the position is changed
fn position_info(query: Query<(Entity, &Transform)>) {
    for (entity, transform) in query.iter() {
        info!("Entity: {:?} is at {:?}", entity, transform);
    }
}
