use crate::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct AgentBundle {
    pub pbr: PbrBundle,
    pub velocity: Velocity,
    pub team: TeamId,
    pub health: Health,
    pub weapon: RangedWeapon,
}

impl AgentBundle {
    pub fn new(
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        position: Vec3,
        team: TeamId,
    ) -> Self {
        Self {
            team,
            weapon: RangedWeapon::default(),
            health: Health::default(),
            velocity: Velocity::default(),
            pbr: PbrBundle {
                mesh: meshes.add(
                    shape::UVSphere {
                        radius: 0.2,
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
