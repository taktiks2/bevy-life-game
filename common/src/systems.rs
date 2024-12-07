use bevy::prelude::*;

pub fn despawn_entity<T: Component>(mut commands: Commands, to_despawn: Query<Entity, With<T>>) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn setup_camera<T: Component>(mut commands: Commands, camera_type: T) {
    commands.spawn((Camera2d, camera_type));
}
