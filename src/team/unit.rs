use bevy::prelude::*;

#[derive(
	Debug, Copy, Clone, Component, Deref, DerefMut, PartialEq, Eq, Hash,
)]
pub struct UnitId(pub usize);

impl UnitId {}


pub mod default_unit {
	use super::UnitId;
	pub const BARRACKS: UnitId = UnitId(0);
	pub const ARCHER: UnitId = UnitId(1);
	pub const PROJECTILE: UnitId = UnitId(2);
}
