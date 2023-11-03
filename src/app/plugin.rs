use bevy::prelude::*;
use forky_play::*;
use gamai::*;

use crate::*;
// use crate::*;

// use crate::seek_system;

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct EarlyUpdateSet;
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct UpdateSet;
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct LateUpdateSet;

pub struct WarTrancePlugin;

impl Plugin for WarTrancePlugin {
    fn build(&self, app: &mut App) {
        app.__()
            .add_plugins(plugins::ForkyDebugPlugin::default().without_debug_cameras())
            .add_plugins(TreePlugin::new(AgentTree))
            .add_systems(Startup, spawn_camera)
            .add_systems(Startup, spawn_agents)
            .configure_set(Update, EarlyUpdateSet)
            .configure_set(Update, UpdateSet.after(EarlyUpdateSet))
            .configure_set(Update, LateUpdateSet.after(UpdateSet))
            .add_systems(Update, velocity_system.in_set(LateUpdateSet))
            .__();
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 10., -0.001))
            .looking_at(Vec3::ZERO, Vec3::UP),
        ..default()
    });
}
pub fn spawn_agents(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        AgentBundle::new(
            &mut meshes,
            &mut materials,
            Vec3::new(-0.5, 0., 0.),
            TeamId::team0(),
        ),
        TreeBundle::new(AgentTree),
    ));
    commands.spawn((
        AgentBundle::new(
            &mut meshes,
            &mut materials,
            Vec3::new(0.5, 0., 0.),
            TeamId::team1(),
        ),
        TreeBundle::new(AgentTree),
    ));
}
