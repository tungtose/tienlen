use bevy::prelude::*;

use crate::resources::Global;

#[derive(Component)]
pub struct MainCamera;

pub fn init(mut commands: Commands) {
    info!("Tienlen client started");

    // Setup Camera
    commands.spawn((MainCamera, Camera2dBundle::default()));

    // Setup Global Resource
    let global = Global::default();

    // Insert Global Resource
    commands.insert_resource(global);
}

pub fn my_cursor_system(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(_world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        // eprintln!("World coords: {}/{}", world_position.x, world_position.y);
    }
}
