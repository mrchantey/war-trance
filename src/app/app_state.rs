use crate::*;
use bevy::prelude::*;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
	#[default]
	MainMenu,
	InLevel,
}

#[derive(Debug, Default, Clone, Component)]
pub struct StateTag<S: States>(pub PhantomData<S>);

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
	fn build(&self, app: &mut App) { register_state::<AppState>(app); }
}

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnUpdate<S: States>(pub S);

pub fn register_state<S: States + Clone>(app: &mut App) -> &mut App {
	app.add_state::<AppState>();
	for variant in S::variants() {
		app.configure_set(
			Update,
			OnUpdate(variant.clone()).run_if(in_state(variant.clone())),
		);
		app.add_systems(
			OnExit(variant.clone()),
			(apply_deferred, despawn_marked::<StateTag<S>>),
		);
	}
	app
}
