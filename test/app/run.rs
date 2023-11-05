use bevy::prelude::*;
use sweet::*;
use war_trance::*;

#[sweet_test(non_send)]
pub fn compiles() -> Result<()> {
	let mut app = App::new();
	app.add_plugins(WarTrancePlugin);
	// app.finish();
	// app.update();

	Ok(())
}
