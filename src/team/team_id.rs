use bevy::prelude::*;

#[derive(
	Debug, Copy, Clone, Component, Deref, DerefMut, PartialEq, Eq, Hash,
)]
pub struct TeamId(pub usize);

impl TeamId {}

pub mod default_teams {
	use super::TeamId;

	pub const BLUE: TeamId = TeamId(0);
	pub const RED: TeamId = TeamId(1);
}
