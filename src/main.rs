#[allow(dead_code)]
// use bevy::prelude::*;
#[path = "./lib.rs"]
mod lib;
use lib::*;
// mod ai;
// use ai::*;

fn main() {
    let mut app = bevy::prelude::App::new();
    app.add_plugins(WarTrancePlugin);

    app.run();
}
