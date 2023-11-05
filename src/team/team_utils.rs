use bevy::prelude::*;
use super::*;
pub fn closest_ally<'a>(
	position: &Vec3,
	team: &TeamId,
	query: impl Iterator<Item = (Entity, &'a Transform, &'a TeamId)>,
) -> Option<(Entity, &'a Transform, &'a TeamId)> {
	closest_on_team(position, query, |other_team| other_team == team)
}
pub fn closest_enemy<'a>(
	position: &Vec3,
	team: &TeamId,
	query: impl Iterator<Item = (Entity, &'a Transform, &'a TeamId)>,
) -> Option<(Entity, &'a Transform, &'a TeamId)> {
	closest_on_team(position, query, |other_team| other_team != team)
}
pub fn closest_on_team<'a>(
	position: &Vec3,
	query: impl Iterator<Item = (Entity, &'a Transform, &'a TeamId)>,
	cmp: impl Fn(&TeamId) -> bool,
) -> Option<(Entity, &'a Transform, &'a TeamId)> {
	query
		.filter(|(_, _, other_team, ..)| cmp(*other_team))
		.min_by_key(|(_, other_transform, ..)| {
			(position.distance_squared(other_transform.translation) * 1000.)
				as i32
		})
	// .map(|(entity, ..)| entity)
}
