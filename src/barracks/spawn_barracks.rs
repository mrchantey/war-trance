use crate::*;
use bevy::prelude::*;
use gamai::*;

pub fn spawn_two_barracks(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	commands.spawn((
		BarracksBundle::new(
			&mut meshes,
			&mut materials,
			Vec3::new(-3., 0., 0.),
			TeamId::team0(),
		),
		TreeBundle::new(AgentTree),
	));
	commands.spawn((
		BarracksBundle::new(
			&mut meshes,
			&mut materials,
			Vec3::new(3., 0., 0.),
			TeamId::team1(),
		),
		TreeBundle::new(AgentTree),
	));
}
