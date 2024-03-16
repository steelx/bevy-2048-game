use bevy::math::vec2;
use bevy::prelude::*;

const TILE_SIZE: f32 = 40.;

#[derive(Component)]
struct Board {
  pub size: u8
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_board);
  }
}

fn setup_board(mut commands: Commands) {
  let board = Board {
    size: 4
  };

  let physical_board_size = f32::from(board.size) * TILE_SIZE;

  commands.spawn((
    SpriteBundle {
      sprite: Sprite {
        custom_size: Some(vec2(
          physical_board_size, physical_board_size
        )),
        ..default()
      },
      ..default()
    },
    board
  ));
}
