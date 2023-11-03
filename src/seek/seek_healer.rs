use super::*;
use crate::*;
use bevy::prelude::*;
use gamai::*;

#[derive(Component)]
pub struct Healer;

/// Finds the closest entity with the [Healer] tag
#[action(props=Score::Fail, order=ActionOrder::PreParentUpdate)]
pub fn seek_healer_scorer<N: AiNode>(
    mut query: Query<(
        &Transform,
        &TeamId,
        &Health,
        &mut Prop<Score, N>,
        &mut Prop<SeekTarget, N>,
    )>,
    healers: Query<(Entity, &Transform, &TeamId), With<Healer>>,
) {
    for (transform, team, health, mut score, mut seek) in query.iter_mut() {
        if let Some(other_entity) = closest_ally(&transform.translation, team, healers.iter()) {
            **score = Score::Weight((health.max - **health) / health.max);
            **seek = SeekTarget::Entity(other_entity);
        } else {
            **score = Score::Fail;
        }
    }
}
