use super::*;
use bevy::prelude::*;
use gamai::*;

#[action]
pub fn seek_enemy_scorer<N: AiNode>(
    mut query: Query<(
        &Transform,
        &TeamId,
        &mut Prop<Score, N>,
        &mut Prop<SeekTarget, N>,
    )>,
    enemies: Query<(Entity, &Transform, &TeamId)>,
) {
    for (transform, team, mut score, mut seek) in query.iter_mut() {
        if let Some(other_entity) = closest_enemy(&transform.translation, team, enemies.iter()) {
            **score = Score::Pass;
            **seek = SeekTarget::Entity(other_entity);
        } else {
            **score = Score::Fail;
        }
    }
}
