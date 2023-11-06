use crate::*;
use bevy::prelude::*;
use bevy_health_bar3d::prelude::*;
use bevy_rapier3d::prelude::*;
use forky_play::*;
use gamai::*;
// use crate::*;

// use crate::seek_system;
pub struct AppPlugin;

impl Plugin for AppPlugin {
	fn build(&self, app: &mut App) {
		app.add_plugins((
			WarTrancePlugin,
			plugins::ForkyDebugPlugin::default().without_debug_cameras(),
		));
	}
}

pub struct WarTrancePlugin;

impl Plugin for WarTrancePlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.__() //INTERNAL PLUGINS
			.add_plugins(RoundPlugin)
			.__() //PHYSICS
			.add_plugins((
				RapierPhysicsPlugin::<NoUserData>::default(),
				// RapierDebugRenderPlugin::default(),
			))
			// .add_systems(Update, display_events.in_set(UpdateSet))
			.__() //HEALTH BARS
			.add_plugins(HealthBarPlugin::<Health>::default())
			.insert_resource(
				ColorScheme::<Health>::new()
					.foreground_color(ForegroundColor::Static(Color::GREEN))
					.background_color(Color::RED),
			)
			.__() // AI
			.add_plugins(TreePlugin::new(AgentTree))
			.__()
			.add_systems(PreStartup, create_default_team_assets)
			.add_systems(Startup, setup_ui)
			.add_systems(Startup, spawn_camera)
			.add_systems(Startup, spawn_two_barracks)
			.add_systems(Update, spawn_agents.in_set(EarlyUpdateSet))
			.__()
			.insert_resource(UnitCount::default())
			.add_systems(Update, update_team_count.in_set(EarlyUpdateSet))
			.__()
			.add_systems(Update, damage_on_collide.in_set(LateUpdateSet))
			.add_systems(Update, timed_destroy.in_set(LateUpdateSet))
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
