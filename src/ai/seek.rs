use crate::*;
use anyhow::Result;
use bevy::prelude::*;
use gamai::*;
// #[derive(Debug, Clone, Copy, Component)]
// pub struct Seek {
//     pub target: Vec3,
// }
#[derive(Debug, Clone, Copy, Component)]
pub enum SeekTarget {
    Position(Vec3),
    // Seek an entity, if it has been despawned, seek component is removed
    Entity(Entity),
}

impl SeekTarget {
    pub fn to_position(&self, transforms: &Query<&Transform>) -> Result<Vec3> {
        match self {
            SeekTarget::Position(val) => Ok(*val),
            SeekTarget::Entity(entity) => Ok(transforms.get(*entity)?.translation),
        }
    }
}

/// Seeks a [SeekTarget]
///
/// # Fails
/// If the target is an entity and it has been despawned
#[action]
pub fn seek_target<N: AiNode>(
    mut commands: Commands,
    mut query: Query<
        (Entity, &Transform, &mut Velocity, &Prop<SeekTarget, N>),
        With<Prop<Running, N>>,
    >,
    transforms: Query<&Transform>,
) {
    for (entity, transform, mut velocity, seek) in query.iter_mut() {
        let target = match **seek {
            SeekTarget::Position(pos) => Some(pos),
            SeekTarget::Entity(target_entity) => {
                if let Ok(target_transform) = transforms.get(target_entity) {
                    Some(target_transform.translation)
                } else {
                    None
                }
            }
        };

        match target {
            Some(target) => {
                let delta = (target - transform.translation).normalize_or_zero() * velocity.max;
                **velocity = delta;
            }
            None => {
                commands
                    .entity(entity)
                    .insert(Prop::<_, N>::new(ActionResult::Failure));
            }
        }

        // if let SeekTarget::Entity(target_entity) = seek_target {
        //     if let Ok(target) = query.get(*target_entity) {
        //         transform.translation = target.translation;
        //     }
        // }
    }
}

// pub fn seek_system(
//     mut commands: Commands,
//     mut queries: ParamSet<(
//         Query<(Entity, &mut Transform, &mut Seek, &SeekTarget)>,
//         Query<&Transform>,
//         Query<&Transform, Changed<Transform>>,
//     )>,
// ) {
//     // let _ = entities;

//     for (_, _, mut seek, seek_target) in queries.p0().iter_mut() {
//         if let SeekTarget::Entity(target_entity) = seek_target {
//             if let Ok(target) = queries.p1().get(*target_entity) {
//                 seek.target = target.translation;
//             } else {
//                 commands.entity(*target_entity).remove::<SeekTarget>();
//             }
//         }
//     }

//     // let seeks = params
//     //     .p0()
//     //     .iter()
//     //     .map(|(entity, _, seek)| (entity, *seek))
//     //     .collect::<Vec<_>>();

//     // let targets = seeks
//     //     .into_iter()
//     //     .filter_map(|(seek_entity, seek)| match seek {
//     //         SeekTarget::Position(pos) => Some((seek_entity, pos)),
//     //         SeekTarget::Entity(target_entity) => {
//     //             if let Ok(target) = queries.p1().get(target_entity) {
//     //                 Some((seek_entity, target.translation))
//     //             } else {
//     //                 commands.entity(seek_entity).remove::<SeekTarget>();
//     //                 None
//     //             }
//     //         }
//     //     });

//     // if let Ok(target) = params.p1().get(*entity) {
//     //     let delta = (target.translation - transform.translation).normalize_or_zero();
//     //     transform.translation += delta;
//     // }
//     // for (mut transform, seek) in params.p0().iter_mut() {
//     //     }
//     // }
//     // f
// }
