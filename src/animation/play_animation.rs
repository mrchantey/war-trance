use beet::prelude::*;
use bevy::animation::RepeatAnimation;
use bevy::ecs::schedule::SystemConfigs;
use bevy::prelude::*;
use std::time::Duration;


/// Play an animation on the agent when this action starts running.
#[derive(Debug, Default, Clone, PartialEq, Component, Reflect)]
#[reflect(Default, Component, ActionMeta)]
pub struct PlayAnimation {
	animation: AnimationNodeIndex,
	graph: Handle<AnimationGraph>,
	pub trigger_if_playing: bool,
	pub repeat: RepeatAnimation,
	pub transition_duration: Duration,
}

impl PlayAnimation {
	pub fn new(
		animation: AnimationNodeIndex,
		graph: Handle<AnimationGraph>,
	) -> Self {
		Self {
			animation,
			graph,
			trigger_if_playing: false,
			repeat: RepeatAnimation::Forever,
			transition_duration: Duration::from_millis(250),
		}
	}
	pub fn with_duration(mut self, duration: Duration) -> Self {
		self.transition_duration = duration;
		self
	}
	pub fn repeat(mut self, repeat: RepeatAnimation) -> Self {
		self.repeat = repeat;
		self
	}
	pub fn trigger_if_playing(mut self) -> Self {
		self.trigger_if_playing = true;
		self
	}
}

/// Play animations for behaviors that run after the agent loads
fn play_animation_on_run(
	mut animators: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
	children: Query<&Children>,
	query: Query<(&TargetAgent, &PlayAnimation), Added<Running>>,
) {
	for (agent, play_animation) in query.iter() {
		// log::info!("playonrun {}", agents.iter().count());
		// let Ok((mut player, mut transitions)) = agents.get_mut(agent.0) else {
		// 	continue;
		// };
		let Some(target) = ChildrenExt::first(**agent, &children, |entity| {
			animators.contains(entity)
		}) else {
			continue;
		};
		// safe unwrap, just checked
		let (mut player, mut transitions) = animators.get_mut(target).unwrap();

		if !player.is_playing_animation(play_animation.animation)
			|| play_animation.trigger_if_playing
		{
			log::info!("playonrun 2");
			transitions
				.play(
					&mut player,
					play_animation.animation,
					play_animation.transition_duration,
				)
				.set_repeat(play_animation.repeat);
		}
	}
}

/// Play animations for animators that load after the behavior starts
fn play_animation_on_load(
	parents: Query<&Parent>,
	mut loaded_animators: Query<
		(Entity, &mut AnimationPlayer, &mut AnimationTransitions),
		Added<AnimationPlayer>,
	>,
	query: Query<(&TargetAgent, &PlayAnimation), With<Running>>,
) {
	for (entity, mut player, mut transitions) in loaded_animators.iter_mut() {
		log::info!("playonload 0");
		let Some(play_animation) =
			ParentExt::find(entity, &parents, |parent| {
				query.iter().find_map(|(target, play_animation)| {
					if **target == parent {
						Some(play_animation)
					} else {
						log::info!("nope");
						None
					}
				})
			})
		else {
			continue;
		};
		log::info!("playonload 1");
		if !player.is_playing_animation(play_animation.animation)
			|| play_animation.trigger_if_playing
		{
			log::info!("playonload 2");

			transitions
				.play(
					&mut player,
					play_animation.animation,
					play_animation.transition_duration,
				)
				.set_repeat(play_animation.repeat);
		}
	}
}

impl ActionMeta for PlayAnimation {
	fn graph_role(&self) -> GraphRole { GraphRole::Node }
}

impl ActionSystems for PlayAnimation {
	fn systems() -> SystemConfigs {
		(play_animation_on_run, play_animation_on_load).in_set(TickSet)
	}
}



// // Once the scene is loaded, start the animation
pub fn init_animators(
	mut commands: Commands,
	parents: Query<&Parent>,
	graphs: Query<&Handle<AnimationGraph>>,
	mut players: Query<Entity, Added<AnimationPlayer>>,
) {
	for entity in &mut players {
		if let Some(graph) =
			ParentExt::find(entity, &parents, |entity| graphs.get(entity).ok())
		{
			commands.entity(entity).insert(graph.clone());
		}
		commands.entity(entity).insert(AnimationTransitions::new());
	}
}
