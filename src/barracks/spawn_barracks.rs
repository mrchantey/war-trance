use crate::*;
use bevy::prelude::*;

pub fn spawn_two_barracks(
	mut commands: Commands,
	team_assets: Res<TeamAssets>,
) {
	BarracksBundle::spawn(
		&mut commands,
		&team_assets,
		Vec3::new(-3., 0., 0.),
		default_teams::BLUE,
	);
	BarracksBundle::spawn(
		&mut commands,
		&team_assets,
		Vec3::new(3., 0., 0.),
		default_teams::RED,
	);
}
