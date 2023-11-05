use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;

#[derive(Component, Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);

#[derive(Bundle)]
pub struct BarracksBundle {
	pub spatial: SpatialBundle,
	pub team: TeamId,
	pub health: HealthBundle,
	pub timer: SpawnTimer,
	pub collider: ColliderBundle,
}

const RADIUS: f32 = 0.5;
const DIAMETER: f32 = RADIUS * 2.;

impl BarracksBundle {
	pub fn spawn(
		commands: &mut Commands,
		team_assets: &TeamAssets,
		position: Vec3,
		team: TeamId,
	) {
		let pbr_bundle =
			team_assets.bundle(team, default_unit::BARRACKS, DIAMETER);

		commands
			.spawn(Self::new(position, team))
			.with_children(|parent| {
				parent.spawn(pbr_bundle);
			});
	}

	fn new(position: Vec3, team: TeamId) -> Self {
		Self {
			team,
			health: HealthBundle::new(Health::full(1000.), RADIUS * 1.5),
			timer: SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
			collider: ColliderBundle::new(Collider::cuboid(
				RADIUS, RADIUS, RADIUS,
			)),
			spatial: SpatialBundle::from_transform(
				Transform::from_translation(position),
			),
		}
	}
}
