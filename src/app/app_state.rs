use crate::*;
use bevy::prelude::*;
use extend::ext;
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
	fn build(&self, app: &mut App) { app.register_state::<AppState>(); }
}

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OnUpdate<S: States>(pub S);


#[ext(name=AppExtState)]
pub impl App {
	fn register_state<S: States + Clone>(&mut self) -> &mut Self {
		self.add_state::<S>();
		for variant in S::variants() {
			self.configure_set(
				Update,
				OnUpdate(variant.clone()).run_if(in_state(variant.clone())),
			);
			self.add_systems(
				OnExit(variant.clone()),
				(apply_deferred, despawn_marked::<StateTag<S>>),
			);
		}
		self
	}
}
