use crate::*;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use forky_play::*;

/// Runs before [UpdateSet] in [RoundPlaySet]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct EarlyUpdateSet;
/// Main Update Set in [RoundPlaySet]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct UpdateSet;
/// Runs after [UpdateSet] in [RoundPlaySet]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct LateUpdateSet;
/// Runs after [LateUpdateSet] in [RoundPlaySet]
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct DespawnSet;

/// Overarching system set for all systems that update while a round is playing.
/// This set will only run with [RoundLifecycle::Play] resource.
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct RoundPlaySet;

// todo use state transitions
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct RoundStart;
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct RoundEnd;
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct RoundPause;
#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, ScheduleLabel)]
pub struct RoundUnpause;

#[derive(Component, Resource, Deref, DerefMut)]
pub struct RoundId(pub usize);

#[derive(Resource, Eq, PartialEq)]
pub enum RoundLifecycle {
	Load,
	Play,
	Pause,
	Unload,
}

#[derive(Resource, Deref, DerefMut)]
pub struct PrevRoundLifecycle(pub RoundLifecycle);

pub struct RoundPlugin;

impl Plugin for RoundPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.configure_set(
				Update,
				RoundPlaySet.run_if(resource_equals(RoundLifecycle::Play)),
			)
			.configure_set(Update, EarlyUpdateSet.in_set(RoundPlaySet))
			.configure_set(
				Update,
				UpdateSet.in_set(RoundPlaySet).after(EarlyUpdateSet),
			)
			.configure_set(
				Update,
				LateUpdateSet.in_set(RoundPlaySet).after(UpdateSet),
			)
			.configure_set(
				Update,
				DespawnSet.in_set(RoundPlaySet).after(LateUpdateSet),
			)
			.__()
			.insert_resource(RoundId(0))
			.insert_resource(RoundLifecycle::Load)
			.insert_resource(PrevRoundLifecycle(RoundLifecycle::Pause))
			.add_systems(
				Update,
				(
					apply_deferred
						.in_set(RoundPlaySet)
						.after(LateUpdateSet)
						.before(DespawnSet),
					despawn_marked::<DespawnMarker>.in_set(DespawnSet),
					apply_deferred.in_set(RoundPlaySet).after(DespawnSet),
				),
			)
			.__();
	}
}
