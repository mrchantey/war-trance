use crate::*;
use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct TimedDestroy(pub Timer);

pub fn timed_destroy(
	mut commands: Commands,
	time: Res<Time>,
	mut query: Query<(Entity, &mut TimedDestroy)>,
) {
	for (entity, mut timer) in query.iter_mut() {
		timer.tick(time.delta());
		if timer.finished() {
			commands.entity(entity).insert(DespawnMarker);
		}
	}
}
