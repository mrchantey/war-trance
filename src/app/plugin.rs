use crate::*;
use bevy::prelude::*;
use bevy_health_bar3d::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_play::*;
use gamai::*;
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
			.add_plugins(
				plugins::ForkyDebugPlugin::default().without_debug_cameras(),
			)
			.__()
			.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
			// .add_plugins(RapierDebugRenderPlugin::default())
			// .add_systems(Update, display_events.in_set(UpdateSet))
			.__()
			.add_plugins(HealthBarPlugin::<Health>::default())
			.insert_resource(
				ColorScheme::<Health>::new()
					.foreground_color(ForegroundColor::Static(Color::GREEN))
					.background_color(Color::RED),
			)
			.__()
			.add_plugins(TreePlugin::new(AgentTree))
			.__()
			.configure_set(Update, EarlyUpdateSet)
			.configure_set(Update, UpdateSet.after(EarlyUpdateSet))
			.configure_set(Update, LateUpdateSet.after(UpdateSet))
			.__()
			.add_systems(PreStartup, create_default_team_assets)
			.add_systems(Startup, spawn_camera)
			.add_systems(Startup, spawn_two_barracks)
			.add_systems(Update, spawn_agents.in_set(EarlyUpdateSet))
			.__()
			.insert_resource(TeamCount::default())
			.add_systems(Update, update_team_count.in_set(EarlyUpdateSet))
			.__()
			.add_systems(Update, damage_on_collide.in_set(LateUpdateSet))
			.add_systems(Update, timed_destroy.in_set(LateUpdateSet))
			.add_systems(Update, velocity_system.in_set(LateUpdateSet))
			.add_systems(
				Last,
				(apply_deferred, despawn_marked, apply_deferred).chain(),
			)
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
