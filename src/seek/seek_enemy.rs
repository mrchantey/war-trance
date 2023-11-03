use super::*;
use crate::*;
use bevy::prelude::*;
use gamai::*;

#[action(props=Score::Fail, order=ActionOrder::PreParentUpdate)]
pub fn seek_enemy_scorer<N: AiNode>(
	mut query: Query<(
		&Transform,
		&TeamId,
		&RangedWeapon,
		&mut Prop<Score, N>,
		&mut Prop<SeekTarget, N>,
	)>,
	enemies: Query<(Entity, &Transform, &TeamId), With<Health>>,
) {
	for (transform, team, weapon, mut score, mut seek) in query.iter_mut() {
		if let Some(other_entity) =
			closest_enemy(&transform.translation, team, enemies.iter())
		{
			let enemy_pos = enemies.get(other_entity).unwrap().1.translation;
			let dir_enemy_to_self =
				(transform.translation - enemy_pos).normalize();
			let pos_in_range =
				enemy_pos + dir_enemy_to_self * (weapon.range * 0.9); //offset to avoid rounding errors
			**seek = SeekTarget::Position(pos_in_range);
			**score = Score::Pass;
		// **seek = SeekTarget::Entity(other_entity);
		} else {
			**score = Score::Fail;
		}
	}
}
