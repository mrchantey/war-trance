use super::*;
use bevy::prelude::*;
use gamai::*;

#[derive(Component)]
pub struct Healer;

#[action]
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
    for (seeker_transform, team, health, mut score, mut seek) in query.iter_mut() {
        if let Some((other_entity, ..)) = healers
            .iter()
            .filter(|(_, _, other_team, ..)| **other_team == *team)
            .min_by_key(|(_, other_transform, ..)| {
                (seeker_transform
                    .translation
                    .distance_squared(other_transform.translation)
                    * 1000.) as i32
            })
        {
            **score = Score::Weight((health.max - **health) / health.max);
            **seek = SeekTarget::Entity(other_entity);
        } else {
            **score = Score::Fail;
        }
    }
}
