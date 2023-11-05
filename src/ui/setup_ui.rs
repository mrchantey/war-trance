use crate::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct RoundUi;

#[derive(Component)]
pub struct TeamScoreUi {
	pub team: TeamId,
	pub score: usize,
}
#[derive(Component)]
pub struct TeamRatioUi {
	pub team: TeamId,
	pub num_players: usize,
}

pub fn setup_ui(mut commands: Commands) {
	// root node
	commands
		.spawn(NodeBundle {
			style: Style {
				// size: Size::new(Val::Percent(100.), Val::Percent(100.)),
				// justify_content: JustifyContent::Center,
				// align_items: AlignItems::Center,
				flex_direction: FlexDirection::Column,
				padding: UiRect::all(Val::Px(5.)),

				..default()
			},
			..default()
		})
		.with_children(|parent| {
			let style = TextStyle {
				font_size: 32.,
				..default()
			};
			// title
			parent.spawn(TextBundle::from_section("War Trance", style.clone()));
			parent.spawn(TextBundle::from_section("Round: 0", style.clone()));
			parent.spawn(TextBundle::from_section("Blue: 0", style.clone()));
			parent.spawn(TextBundle::from_section("Red: 0", style.clone()));
		});
}
