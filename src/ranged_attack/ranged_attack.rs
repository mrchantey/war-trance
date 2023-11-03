use super::*;
use crate::ProjectileBundle;
use crate::*;
use bevy::prelude::*;
use gamai::*;

/// Executes a ranged attack if the windup has elapsed.
/// # Components
/// - [`Transform`]
/// - [`RangedWeapon`]
/// # Props
/// - [`ActionTimer`]
/// - [`SeekTarget`]
#[action(props=(ActionTimer::default(),SeekTarget::default()))]
pub fn ranged_attack<N: AiNode>(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    transforms: Query<&Transform>,
    mut query: Query<
        (
            Entity,
            &Transform,
            &RangedWeapon,
            &TeamId,
            &Prop<ActionTimer, N>,
            &mut Prop<SeekTarget, N>,
        ),
        With<Prop<Running, N>>,
    >,
) {
    for (entity, transform, weapon, team, timer, target) in query.iter_mut() {
        if timer.last_start.elapsed() > weapon.windup {
            commands
                .entity(entity)
                .insert(Prop::<_, N>::new(ActionResult::Success));
            // println!("{}", timer.last_start.elapsed().as_secs_f32());
            // time.
            let target_pos = target.to_position(&transforms).unwrap();
            let dir = (target_pos - transform.translation).normalize() * 0.2;
            commands.spawn(ProjectileBundle::new(
                &mut meshes,
                &mut materials,
                transform.translation,
                Velocity::new(dir, 1.),
                team.clone(),
            ));
            // println!(
            //     "time: {}, entity {:?} attacking {:?}",
            //     time.0, entity, target_pos
            // );
        }
    }
}
