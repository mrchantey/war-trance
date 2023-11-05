use super::*;
use bevy::prelude::*;
use bevy::utils::HashMap;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AgentKey {
	pub team: TeamId,
	pub unit: UnitId,
}

impl AgentKey {
	pub fn new(team: TeamId, unit: UnitId) -> Self { Self { team, unit } }
}

pub struct AgentAssets {
	pub mesh: Handle<Mesh>,
	pub material: Handle<StandardMaterial>,
}

#[derive(Default, Resource)]
pub struct TeamAssets {
	pub assets: HashMap<AgentKey, AgentAssets>,
	pub colors: HashMap<TeamId, Color>,
	pub meshes: HashMap<UnitId, Handle<Mesh>>,
	pub materials: HashMap<TeamId, Handle<StandardMaterial>>,
}


impl TeamAssets {
	/// # Panics
	/// If the requested pair doesnt exist
	pub fn bundle(&self, team: TeamId, unit: UnitId, scale: f32) -> PbrBundle {
		let key = AgentKey::new(team, unit);
		let AgentAssets { mesh, material } = self
			.assets
			.get(&key)
			.expect(format!("No asset for {:?}", key).as_str());

		PbrBundle {
			mesh: mesh.clone(),
			material: material.clone(),
			transform: Transform::from_scale(Vec3::splat(scale)),
			..default()
		}
	}
}

pub fn create_default_team_assets(
	mut commands: Commands,
	mut material_assets: ResMut<Assets<StandardMaterial>>,
	mut mesh_assets: ResMut<Assets<Mesh>>,
) {
	let mut colors = HashMap::new();
	let mut meshes = HashMap::new();

	let mut assets = HashMap::new();
	let mut materials = HashMap::new();

	colors.insert(default_teams::BLUE, Color::rgb(0., 1., 1.));
	colors.insert(default_teams::RED, Color::rgb(1., 0., 1.));

	meshes.insert(
		default_unit::ARCHER,
		mesh_assets.add(
			shape::UVSphere {
				radius: 0.5,
				..default()
			}
			.into(),
		),
	);
	meshes.insert(
		default_unit::PROJECTILE,
		mesh_assets.add(
			shape::UVSphere {
				radius: 0.5,
				..default()
			}
			.into(),
		),
	);
	meshes.insert(
		default_unit::BARRACKS,
		mesh_assets.add(
			shape::Plane {
				size: 1.,
				..default()
			}
			.into(),
		),
	);

	for (team, color) in colors.iter() {
		let material = material_assets.add(color.clone().into());
		materials.insert(team.clone(), material.clone());

		for (unit, mesh) in meshes.iter() {
			assets.insert(
				AgentKey::new(team.clone(), unit.clone()),
				AgentAssets {
					mesh: mesh.clone(),
					material: material.clone(),
				},
			);
		}
	}
	commands.insert_resource(TeamAssets {
		assets,
		colors,
		meshes,
		materials,
	});
}


// impl TeamAssets {
// 	pub fn new() -> Self {
// 		Self {
// 			materials: HashMap::new(),
// 			meshes: HashMap::new(),
// 		}
// 	}
// 	pub fn add_team(
// 		&mut self,
// 		team: TeamId,
// 		materials: &mut Assets<StandardMaterial>,
// 		meshes: &mut Assets<Mesh>,
// 	) {
// 		if let Some(_) = self.materials.get(&team) {
// 			return;
// 		}

// 		self.materials
// 			.insert(team, materials.add(team.color().into()));
// 		self.meshes.insert(
// 			team,
// 			meshes.add(
// 				shape::Plane {
// 					size: 0.5,
// 					..default()
// 				}
// 				.into(),
// 			),
// 		);
// 	}
// 	pub fn get_material(&self, team: TeamId) -> Handle<StandardMaterial> {
// 		self.materials.get(&team).unwrap().clone()
// 	}
// 	pub fn get_mesh(&self, team: TeamId) -> Handle<Mesh> {
// 		self.meshes.get(&team).unwrap().clone()
// 	}
// }
