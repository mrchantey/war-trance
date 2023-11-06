use bevy::prelude::*;
use sweet::*;
use war_trance::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	app.add_plugins((AppStatePlugin, LevelPlugin));
	app.update();

	expect(**app.world.resource::<State<AppState>>())
		.to_be(AppState::MainMenu)?;
	expect(**app.world.resource::<State<LevelState>>())
		.to_be(LevelState::Void)?;

	app.insert_resource(NextState(Some(AppState::InLevel)));
	app.update();
	expect(**app.world.resource::<State<AppState>>())
		.to_be(AppState::InLevel)?;
	expect(**app.world.resource::<State<LevelState>>())
		.to_be(LevelState::Void)?;

	app.update();
	expect(**app.world.resource::<State<LevelState>>())
		.to_be(LevelState::PreRound)?;

	// app.insert_resource(NextState(Some(LevelState::Void)));
	app.insert_resource(NextState(Some(AppState::MainMenu)));
	app.update();
	expect(**app.world.resource::<State<AppState>>())
		.to_be(AppState::MainMenu)?;
	expect(**app.world.resource::<State<LevelState>>())
		.to_be(LevelState::PreRound)?;

	app.update();
	expect(**app.world.resource::<State<LevelState>>())
		.to_be(LevelState::Void)?;

	Ok(())
}
