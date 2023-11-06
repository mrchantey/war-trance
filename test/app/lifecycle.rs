use bevy::prelude::*;
use sweet::*;
// use war_trance::*;


fn checks_item(world: &World) {
	expect(world.entities().len()).to_be(1).unwrap();
}

fn spawn_item(mut commands: Commands) { commands.spawn_empty(); }


#[sweet_test]
pub fn exclusive_vs_commands() -> Result<()> {
	let mut app = App::new();
	app.add_systems(Startup, (spawn_item, apply_deferred, checks_item).chain());
	app.update();
	Ok(())
}
