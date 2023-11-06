use bevy::prelude::*;
use bevy::utils::HashMap;

/// Indicate the team to which an entity belongs.
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


/// Indicate this is the singleton entity for a team, every team must have exactly one.
#[derive(Debug, Copy, Clone, Component)]
pub struct TeamSingleton;
/// Indicate this is the singleton entity for a team, every team must have exactly one.
#[derive(Debug, Clone, Resource, Deref, DerefMut)]
pub struct TeamSingletonLookup(pub HashMap<TeamId, Entity>);
