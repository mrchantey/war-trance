use super::*;
use crate::*;
use bevy::prelude::*;
use gamai::*;

/// Calculates the score for a ranged attack.
/// # Components
/// - [`Transform`]
/// - [`TeamId`]
/// - [`RangedWeapon`]
/// # Props
/// - [`SeekTarget`]
/// - [`Score`]
#[action(props=Score::Fail, order=ActionOrder::PreParentUpdate)]
pub fn ranged_attack_scorer<N: AiNode>(
	mut query: Query<(
		&Transform,
		&TeamId,
		&RangedWeapon,
		&mut Prop<SeekTarget, N>,
		&mut Prop<Score, N>,
	)>,
	enemies: Query<(Entity, &Transform, &TeamId), With<Health>>,
) {
	for (transform, team, ranged_weapon, mut target, mut score) in
		query.iter_mut()
	{
		if let Some((enemy, enemy_transform, _)) =
			closest_enemy(&transform.translation, team, enemies.iter())
		{
			let dist =
				transform.translation.distance(enemy_transform.translation);
			if dist < ranged_weapon.range {
				**score = Score::Pass;
				**target = SeekTarget::Entity(enemy);
			} else {
				**score = Score::Fail;
			}
		} else {
			**score = Score::Fail;
		}
	}
}
