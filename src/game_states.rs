use bevy::prelude::*;

pub struct GameStatesPlugin;

impl Plugin for GameStatesPlugin {
  fn build(&self, app: &mut App) {
    app
      .init_state::<GameState>()
      .insert_state(GameState::Playing);
  }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
  #[default]
  Playing,
  GameOver,
}