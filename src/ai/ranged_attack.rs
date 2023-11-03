use super::*;
use bevy::{core::FrameCount, prelude::*};
use gamai::*;
use std::time::Duration;

#[derive(Component)]
pub struct RangedWeapon {
    pub range: f32,
    pub windup: Duration,
}

impl Default for RangedWeapon {
    fn default() -> Self {
        Self {
            range: 3.,
            windup: Duration::from_secs(1),
        }
    }
}

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
    enemies: Query<(Entity, &Transform, &TeamId)>,
) {
    for (transform, team, ranged_weapon, mut target, mut score) in query.iter_mut() {
        if let Some(enemy) = closest_enemy(&transform.translation, team, enemies.iter()) {
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

/// Executes a ranged attack if the windup has elapsed.
/// # Components
/// - [`Transform`]
/// - [`RangedWeapon`]
/// # Props
/// - [`ActionTimer`]
/// - [`SeekTarget`]
#[action(props=(ActionTimer::default(),SeekTarget::default()))]
pub fn ranged_attack<N: AiNode>(
    time: Res<FrameCount>,
    mut commands: Commands,
    transforms: Query<&Transform>,
    mut query: Query<
        (
            Entity,
            &Transform,
            &RangedWeapon,
            &Prop<ActionTimer, N>,
            &mut Prop<SeekTarget, N>,
        ),
        With<Prop<Running, N>>,
    >,
) {
    for (entity, transform, weapon, timer, target) in query.iter_mut() {
        if timer.last_start.elapsed() > weapon.windup {
            commands
                .entity(entity)
                .insert(Prop::<_, N>::new(ActionResult::Success));
            // println!("{}", timer.last_start.elapsed().as_secs_f32());
            // time.
            let target_pos = target.to_position(&transforms).unwrap();
            println!(
                "time: {}, entity {:?} attacking {:?}",
                time.0, entity, target_pos
            );
        }
    }
}
