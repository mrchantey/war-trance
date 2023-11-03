use crate::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct ProjectileBundle {
    pub pbr: PbrBundle,
    pub velocity: Velocity,
    pub team: TeamId,
    pub timed_destroy: TimedDestroy,
}

impl ProjectileBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        position: Vec3,
        velocity: Velocity,
        team: TeamId,
    ) -> Self {
        Self {
            team,
            velocity,
            timed_destroy: TimedDestroy(Timer::from_seconds(3., TimerMode::Once)),
            pbr: PbrBundle {
                mesh: meshes.add(
                    shape::UVSphere {
                        radius: 0.1,
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
