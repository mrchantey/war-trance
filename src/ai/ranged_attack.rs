use super::*;
use bevy::prelude::*;
use gamai::*;

#[derive(Component)]
pub struct RangedWeapon {
    pub range: f32,
    pub cooldown: f32,
}

impl Default for RangedWeapon {
    fn default() -> Self {
        Self {
            range: 2.,
            cooldown: 1.,
        }
    }
}

#[action]
pub fn ranged_attack_scorer<N: AiNode>(
    mut query: Query<(
        &Transform,
        &TeamId,
        &RangedWeapon,
        &mut Prop<SeekTarget, N>,
        &mut Prop<Score, N>,
    )>,
    enemies: Query<(Entity, &Transform, &TeamId)>,
) {
    for (transform, team, ranged_weapon, mut target, mut score) in query.iter_mut() {
        if let Some(enemy) = closest_enemy(&transform.translation, team, &enemies) {
            let (_, enemy_transform, ..) = enemies.get(enemy).unwrap();
            let dist = transform.translation.distance(enemy_transform.translation);
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
