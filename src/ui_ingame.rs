use bevy::prelude::*;
use crate::board::Game;
use crate::game_states::GameState;
use crate::styles::small_block_styles;

#[derive(Component)]
pub struct InGameMenuParent;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct BestScoreText;


pub struct InGameUiPlugin;

impl Plugin for InGameUiPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_in_game_ui)
      .add_systems(Update, update_current_score.run_if(in_state(GameState::Playing)))
      .add_systems(Update, despawn_in_game_ui.run_if(in_state(GameState::GameOver)));
  }
}

fn update_current_score(mut text_query: Query<&mut Text, With<ScoreText>>, game: Res<Game>) {
  for mut text in &mut text_query {
    let update_text = text.sections.first_mut().expect("text !");
    update_text.value = game.score.to_string();
  }
}

fn despawn_in_game_ui(
  mut commands: Commands,
  in_game_menu_query: Query<Entity, With<InGameMenuParent>>
)
{
  if let Ok(in_game_menu) = in_game_menu_query.get_single() {
    commands.entity(in_game_menu).despawn_recursive();
  }
}

fn spawn_in_game_ui(mut commands: Commands)
{
  commands
    .spawn((
      InGameMenuParent {},
      NodeBundle {
       style: Style {
         width: Val::Percent(100.),
         height: Val::Percent(20.),
         display: Display::Flex,
         justify_content: JustifyContent::SpaceEvenly,
         ..default()
       },
       ..default()
      },
    ))
    // left DIV with Text "2040 Game"
    .with_children(|builder| {
      builder
        .spawn((
          NodeBundle {
            style: small_block_styles(),
            ..default()
          },
        ))
        .with_children(|parent| {
          parent
            .spawn(TextBundle {
              text: Text::from_section("2048", TextStyle {
                font_size: 32., color: Color::YELLOW, ..default()
              }),
              ..default()
            });
        });
    })
    // Center DIV with Text Current Score
    .with_children(|builder| {
      builder
        .spawn((
          NodeBundle {
            style: small_block_styles(),
            background_color: Color::YELLOW.into(),
            ..default()
          },
        ))
        .with_children(|parent| {
          parent
            .spawn((
              ScoreText {},
              TextBundle {
                text: Text::from_section("<Score>", TextStyle {
                  font_size: 32., color: Color::MIDNIGHT_BLUE, ..default()
                }),
                ..default()
              },
            ));
          parent
            .spawn(TextBundle {
              text: Text::from_section("Score", TextStyle {
                font_size: 22., color: Color::MIDNIGHT_BLUE, ..default()
              }),
              ..default()
            });
        });
    });
}