use bevy::prelude::*;
use bevy_mod_debugdump::schedule_graph::Settings;
use std::fs::File;
use std::io::Write;
use war_trance::*;


fn main() {
	let mut app = bevy::prelude::App::new();
	app.add_plugins(WarTrancePlugin);

	let render_graph =
		bevy_mod_debugdump::schedule_graph_dot(&mut app, Update, &Settings {
			// include_system: Some(Box::new(|_| false)),
			..default()
		});
	let path = "target/graph";
	std::fs::create_dir_all(path).unwrap();
	let mut file = File::create("target/graph/render_graph.dot").unwrap();
	file.write_all(render_graph.as_bytes()).unwrap();
}
