use bevy::prelude::*;

pub fn closest_ally<'a>(
	position: &Vec3,
	team: &TeamId,
	query: impl Iterator<Item = (Entity, &'a Transform, &'a TeamId)>,
) -> Option<Entity> {
	closest_on_team(position, query, |other_team| other_team == team)
}
pub fn closest_enemy<'a>(
	position: &Vec3,
	team: &TeamId,
	query: impl Iterator<Item = (Entity, &'a Transform, &'a TeamId)>,
) -> Option<Entity> {
	closest_on_team(position, query, |other_team| other_team != team)
}
pub fn closest_on_team<'a>(
	position: &Vec3,
	query: impl Iterator<Item = (Entity, &'a Transform, &'a TeamId)>,
	cmp: impl Fn(&TeamId) -> bool,
) -> Option<Entity> {
	query
		.filter(|(_, _, other_team, ..)| cmp(*other_team))
		.min_by_key(|(_, other_transform, ..)| {
			(position.distance_squared(other_transform.translation) * 1000.)
				as i32
		})
		.map(|(entity, ..)| entity)
}

#[derive(
	Debug, Copy, Clone, Component, Deref, DerefMut, PartialEq, Eq, Hash,
)]
pub struct TeamId(pub usize);

impl TeamId {
	pub fn team0() -> Self { Self(TEAM0) }
	pub fn team1() -> Self { Self(TEAM1) }

	pub fn color(&self) -> Color {
		match **self {
			0 => Color::rgb(0.3, 0.5, 0.3),
			1 => Color::rgb(0.3, 0.3, 0.5),
			_ => Color::rgb(0.5, 0.5, 0.5),
		}
	}
}

pub const TEAM0: usize = 0;
pub const TEAM1: usize = 1;
