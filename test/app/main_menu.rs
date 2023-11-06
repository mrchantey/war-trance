use bevy::prelude::*;
use sweet::*;
use war_trance::*;


#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	app.add_plugins(AppStatePlugin).add_plugins(MenuPlugin);
	app.update();

	expect(&mut app)
		.num_components::<StateTag<AppState>>()
		.to_be(1)?;
	app.insert_resource(NextState(Some(AppState::InLevel)));
	app.update();
	expect(&mut app)
		.num_components::<StateTag<AppState>>()
		.to_be(0)?;
	Ok(())
}
