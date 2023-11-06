use super::*;
use crate::*;
use bevy::prelude::*;
use forky_play::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.register_state::<LevelState>()
			.add_systems(
				OnEnter(AppState::InLevel),
				|mut next: ResMut<NextState<LevelState>>| {
					next.set(LevelState::PreRound)
				},
			)
			.add_systems(
				OnExit(AppState::InLevel),
				|mut next: ResMut<NextState<LevelState>>| {
					next.set(LevelState::Void)
				},
				// OnEnter(LevelState::Void),
				// |mut next: ResMut<NextState<AppState>>| {
				// 	next.set(AppState::MainMenu)
				// },
			)
			.__();
	}
}
