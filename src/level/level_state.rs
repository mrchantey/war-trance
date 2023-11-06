use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum LevelState {
	#[default]
	Void,
	PreRound,
	InRound,
	PostRound,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PauseState {
	#[default]
	Off,
	On,
}

#[derive(Debug, Component)]
pub struct InRoundTag;
#[derive(Debug, Clone, Eq, PartialEq, Hash, SystemSet)]
pub struct InRoundSet;
