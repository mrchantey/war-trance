use crate::*;
use bevy::prelude::*;
use forky_play::AppExt;


pub struct MenuPlugin;


impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app.__()
			.add_systems(OnEnter(AppState::MainMenu), setup_menu)
			.add_systems(Update, menu.in_set(OnUpdate(AppState::MainMenu)))
			.__();
	}
}


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

// fn setup(mut commands: Commands) { commands.spawn(Camera2dBundle::default()); }

pub fn setup_menu(mut commands: Commands) {
	let _button_entity = commands
		.spawn((
			Name::new("Main Menu"),
			StateTag::<AppState>::default(),
			NodeBundle {
				style: Style {
					// center button
					width: Val::Percent(100.),
					justify_content: JustifyContent::Center,
					align_items: AlignItems::Center,
					..default()
				},
				..default()
			},
		))
		.with_children(|parent| {
			parent
				.spawn(ButtonBundle {
					style: Style {
						width: Val::Px(150.),
						height: Val::Px(65.),
						// horizontally center child text
						justify_content: JustifyContent::Center,
						// vertically center child text
						align_items: AlignItems::Center,
						..default()
					},
					background_color: NORMAL_BUTTON.into(),
					..default()
				})
				.with_children(|parent| {
					parent.spawn(TextBundle::from_section("Play", TextStyle {
						font_size: 40.0,
						color: Color::rgb(0.9, 0.9, 0.9),
						..default()
					}));
				});
		})
		.id();
}

pub fn menu(
	mut next_state: ResMut<NextState<AppState>>,
	mut interaction_query: Query<
		(&Interaction, &mut BackgroundColor),
		(Changed<Interaction>, With<Button>),
	>,
) {
	for (interaction, mut color) in &mut interaction_query {
		match *interaction {
			Interaction::Pressed => {
				*color = PRESSED_BUTTON.into();
				next_state.set(AppState::InLevel);
			}
			Interaction::Hovered => {
				*color = HOVERED_BUTTON.into();
			}
			Interaction::None => {
				*color = NORMAL_BUTTON.into();
			}
		}
	}
}
