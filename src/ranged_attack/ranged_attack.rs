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
	team_assets: Res<TeamAssets>,
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
			if let Some(mut commands) = commands.get_entity(entity) {
				commands.insert(Prop::<_, N>::new(ActionResult::Success));
			}
			// commands
			// 	.entity(entity)
			// 	.insert(Prop::<_, N>::new(ActionResult::Success));
			// println!("{}", timer.last_start.elapsed().as_secs_f32());
			// time.
			if let Ok(target_pos) = target.to_position(&transforms) {
				let dir = (target_pos - transform.translation).normalize();
				ProjectileBundle::spawn(
					&mut commands,
					&team_assets,
					transform.translation,
					dir,
					team.clone(),
				);
			} else {
				//todo what happens here?
			}
			// println!(
			//     "time: {}, entity {:?} attacking {:?}",
			//     time.0, entity, target_pos
			// );
		}
	}
}
