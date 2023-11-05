use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::Collider;
use gamai::TreeBundle;


#[derive(Bundle)]
pub struct AgentBundle {
	pub spatial: SpatialBundle,
	pub velocity: Velocity,
	pub velocity_max: VelocityMax,
	pub team: TeamId,
	pub weapon: RangedWeapon,
	pub health: HealthBundle,
	pub collider: ColliderBundle,
}
const RADIUS: f32 = 0.2;
const DIAMETER: f32 = RADIUS * 2.;

impl AgentBundle {
	pub fn spawn(
		commands: &mut Commands,
		team_assets: &TeamAssets,
		position: Vec3,
		team: TeamId,
	) {
		let pbr_bundle =
			team_assets.bundle(team, default_unit::ARCHER, DIAMETER);

		commands
			.spawn((Self::new(position, team), TreeBundle::new(AgentTree)))
			.with_children(|parent| {
				parent.spawn(pbr_bundle);
			});
	}

	fn new(position: Vec3, team: TeamId) -> Self {
		Self {
			team,
			weapon: RangedWeapon::default(),
			health: HealthBundle::new(Health::default(), RADIUS * 1.5),
			velocity: Velocity::default(),
			velocity_max: VelocityMax::default(),
			collider: ColliderBundle::new(Collider::ball(RADIUS)),
			spatial: SpatialBundle::from_transform(
				Transform::from_translation(position),
			),
		}
	}
}
