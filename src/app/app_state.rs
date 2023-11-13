use crate::*;
use bevy::prelude::*;
use extend::ext;
use forky_play::*;
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
	fn build(&self, app: &mut App) {
		app.__()
			.register_state::<AppState>()
			.register_variant(AppState::MainMenu)
			.register_variant(AppState::InLevel)
			.__();
	}
}

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnUpdate<S: States>(pub S);


#[ext(name=AppExtState)]
pub impl App {
	fn register_state<S: States + Clone>(&mut self) -> &mut Self {
		self.add_state::<S>();
		self
	}
	fn register_variant<S: States + Clone>(&mut self, variant: S) -> &mut Self {
		self.configure_sets(
			Update,
			OnUpdate(variant.clone()).run_if(in_state(variant.clone())),
		);
		self.add_systems(
			OnExit(variant.clone()),
			(apply_deferred, despawn_marked::<StateTag<S>>),
		);
		self
	}
}
