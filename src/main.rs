#![feature(let_chains)]
#[allow(dead_code)]
#[path = "./lib.rs"]
mod lib;
use lib::*;
// mod ai;
// use bevy::prelude::*;
// use ai::*;

fn main() {
	let mut app = bevy::prelude::App::new();
	app.add_plugins(WarTrancePlugin);

	app.run();
}
