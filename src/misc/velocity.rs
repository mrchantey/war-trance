use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Component, Deref, DerefMut)]
pub struct Velocity(pub Vec3);
#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct VelocityMax(pub f32);

impl Default for VelocityMax {
	fn default() -> Self { Self(1.) }
}

pub fn velocity_system(
	time: Res<Time>,
	mut query: Query<(&mut Transform, &Velocity)>,
) {
	for (mut transform, velocity) in query.iter_mut() {
		if **velocity != Vec3::ZERO {
			transform.translation += **velocity * time.delta_seconds();
		}
	}
}
