use bevy::prelude::*;
mod app;
use app::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(WarTrancePlugin);

    app.run();
}
