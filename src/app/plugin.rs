use bevy::prelude::*;
use forky_play::plugins::ForkyDebugPlugin;

pub struct WarTrancePlugin;

impl Plugin for WarTrancePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ForkyDebugPlugin::default());
        // app.add_systems(Startup, spawn_camera);
    }
}

// pub fn spawn_camera(mut commands: Commands) {
//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_translation(Vec3::new(0., 10., 0.))
//             .looking_at(Vec3::ZERO, Vec3::UP),
//         ..default()
//     });
// }
