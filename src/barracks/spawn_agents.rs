use crate::*;
use bevy::prelude::*;
use gamai::*;
use rand::Rng;
use std::time::Duration;

const MIN_SPAWN_COOLDOWN: f32 = 1.0;
const MAX_SPAWN_COOLDOWN: f32 = 5.0;


const SPAWN_RADIUS: f32 = 3.0;

pub fn spawn_agents(
	team_count: Res<TeamCount>,
	time: Res<Time>,
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut query: Query<(&Transform, &TeamId, &mut SpawnTimer)>,
) {
	for (transform, team, mut timer) in query.iter_mut() {
		let cooldown = lerp(
			MIN_SPAWN_COOLDOWN,
			MAX_SPAWN_COOLDOWN,
			1. - team_count.fraction(*team),
		);
		timer.set_duration(Duration::from_secs_f32(cooldown));
		timer.tick(time.delta());
		if timer.finished() {
			let mut rng = rand::thread_rng();
			let z: f32 = rng.gen_range(-SPAWN_RADIUS..SPAWN_RADIUS);
			commands.spawn((
				AgentBundle::new(
					&mut meshes,
					&mut materials,
					transform.translation + Vec3::new(0., 0., z),
					*team,
				),
				TreeBundle::new(AgentTree),
			));
		}
	}
}
