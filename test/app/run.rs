use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = bevy::prelude::App::new();
	app.add_plugins(WarTrancePlugin);

	Ok(())
}
