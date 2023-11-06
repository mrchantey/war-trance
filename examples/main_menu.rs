use bevy::prelude::{App, Startup};
use bevy::DefaultPlugins;
use war_trance::*;


pub fn main() {
	App::new()
    .add_systems(Startup, spawn_camera)
		.add_plugins(DefaultPlugins)
		.add_plugins(AppStatePlugin)
		.add_plugins(MenuPlugin)
		.run();
}
