use crate::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::Ccd;
use bevy_rapier3d::prelude::Collider;

#[derive(Bundle)]
pub struct ProjectileBundle {
	pub spatial: SpatialBundle,
	pub velocity: Velocity,
	pub team: TeamId,
	pub timed_destroy: TimedDestroy,
	pub collider: ColliderBundle,
	pub damage: DamageOnCollide,
	pub ccd: Ccd,
}


const RADIUS: f32 = 0.1;
const SPEED: f32 = 10.;
const DIAMETER: f32 = RADIUS * 2.;
const DAMAGE: f32 = 20.;

impl ProjectileBundle {
	pub fn spawn(
		commands: &mut Commands,
		team_assets: &TeamAssets,
		position: Vec3,
		dir: Vec3,
		team: TeamId,
	) {
		let pbr_bundle =
			team_assets.bundle(team, default_unit::PROJECTILE, DIAMETER);

		commands
			.spawn((Self::new(position, Velocity(dir * SPEED), team),))
			.with_children(|parent| {
				parent.spawn(pbr_bundle);
			});
	}


	pub fn new(position: Vec3, velocity: Velocity, team: TeamId) -> Self {
		Self {
			ccd: Ccd::enabled(),
			team,
			velocity,
			damage: DamageOnCollide(DAMAGE),
			timed_destroy: TimedDestroy(Timer::from_seconds(
				3.,
				TimerMode::Once,
			)),
			collider: ColliderBundle::new(Collider::ball(RADIUS)),
			spatial: SpatialBundle::from_transform(
				Transform::from_translation(position),
			),
		}
	}
}
