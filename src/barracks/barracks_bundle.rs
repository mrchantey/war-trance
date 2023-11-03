use crate::*;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct SpawnTimer(pub Timer);

#[derive(Bundle)]
pub struct BarracksBundle {
	pub pbr: PbrBundle,
	pub team: TeamId,
	pub health: HealthBundle,
	pub timer: SpawnTimer,
}

impl BarracksBundle {
	pub fn new(
		meshes: &mut Assets<Mesh>,
		materials: &mut Assets<StandardMaterial>,
		position: Vec3,
		team: TeamId,
	) -> Self {
		Self {
			team,
			health: HealthBundle::new(Health::default(), 0.6),
			timer: SpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
			pbr: PbrBundle {
				mesh: meshes.add(
					shape::Plane {
						size: 1.,
						..default()
					}
					.into(),
				),
				material: materials.add(team.color().into()),
				transform: Transform::from_translation(position),
				..default()
			},
		}
	}
}
