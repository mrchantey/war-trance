use bevy::animation::RepeatAnimation;
use forky_bevy::prelude::close_on_esc;
use beet::prelude::*;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use war_trance::{init_animators, PlayAnimation};
use std::{f32::consts::PI, time::Duration};
pub fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_plugins(DefaultBeetPlugins)
		.add_systems(Startup, setup)
		.add_systems(Update, init_animators)
		.add_systems(Update, close_on_esc)
		.add_plugins(ActionPlugin::<PlayAnimation>::default())
		.run();
}


// fn setup(commands: Commands) {}

fn setup(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
	mut graphs: ResMut<Assets<AnimationGraph>>,
) {
	// Build the animation graph
	let mut graph = AnimationGraph::new();

	let anim1 = graph.add_clip(asset_server.load("models/KayKit_Adventurers_1.0_FREE/Characters/gltf/Rogue.glb#Animation0"), 1.0, graph.root);
	let anim2 = graph.add_clip(asset_server.load("models/KayKit_Adventurers_1.0_FREE/Characters/gltf/Rogue.glb#Animation1"), 1.0, graph.root);

	// Insert a resource with the current scene information
	let graph = graphs.add(graph);

	// Camera
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(100.0, 100.0, 150.0)
			.looking_at(Vec3::new(0.0, 20.0, 0.0), Vec3::Y),
		..default()
	});

	// Plane
	commands.spawn(PbrBundle {
		mesh: meshes.add(Plane3d::default().mesh().size(500000.0, 500000.0)),
		material: materials.add(Color::srgb(0.3, 0.5, 0.3)),
		..default()
	});

	// Light
	commands.spawn(DirectionalLightBundle {
		transform: Transform::from_rotation(Quat::from_euler(
			EulerRot::ZYX,
			0.0,
			1.0,
			-PI / 4.,
		)),
		directional_light: DirectionalLight {
			shadows_enabled: true,
			..default()
		},
		cascade_shadow_config: CascadeShadowConfigBuilder {
			first_cascade_far_bound: 200.0,
			maximum_distance: 400.0,
			..default()
		}
		.into(),
		..default()
	});

	commands.spawn((
			SceneBundle {
			scene: asset_server.load("models/KayKit_Adventurers_1.0_FREE/Characters/gltf/Rogue.glb#Scene0"),
			transform: Transform::from_scale(Vec3::splat(10.)),
			..default()
		},
		graph.clone(),
		AnimationTransitions::new(),
	)).with_children(|parent|{
		let agent = parent.parent_entity();
		parent.spawn((
			Running,
			Repeat,
			SequenceSelector,
		)).with_children(|parent|{
			parent.spawn((
				LogOnRun::new("running 1"),
				TargetAgent(agent), 
				PlayAnimation::new(anim1,graph.clone()).repeat(RepeatAnimation::Forever),
				RunTimer::default(),
				InsertInDuration::new(RunResult::Success, Duration::from_secs(2))));
				parent.spawn((
				LogOnRun::new("running 2"),
				TargetAgent(agent), 
				RunTimer::default(),
				PlayAnimation::new(anim2,graph.clone()).repeat(RepeatAnimation::Forever),
				InsertInDuration::new(RunResult::Success, Duration::from_secs(1))));
		});
	});
}