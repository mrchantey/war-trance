use bevy::prelude::*;
use bevy_health_bar3d::prelude::*;
use std::ops::Deref;
use std::ops::DerefMut;

#[derive(Debug, Copy, Clone, Component)]
pub struct Health {
	pub value: f32,
	pub max: f32,
}

impl Percentage for Health {
	fn value(&self) -> f32 { self.value / self.max }
}

impl DerefMut for Health {
	fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}

impl Deref for Health {
	type Target = f32;
	fn deref(&self) -> &Self::Target { &self.value }
}

impl Default for Health {
	fn default() -> Self { Self::new(100., 100.) }
}

impl Health {
	pub fn new(value: f32, max: f32) -> Self { Self { value, max } }
	pub fn full(value: f32) -> Self { Self { value, max: value } }
}


#[derive(Bundle)]
pub struct HealthBundle {
	pub health: Health,
	pub health_bar: BarBundle<Health>,
}

impl HealthBundle {
	pub fn new(health: Health, offset: f32) -> Self {
		Self {
			health,
			health_bar: BarBundle::<Health> {
				width: BarWidth::new(0.5),
				offset: BarOffset::new(offset),
				height: BarHeight::Static(0.05),
				..default()
			},
		}
	}
}
