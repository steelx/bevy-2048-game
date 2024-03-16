use bevy::math::vec2;
use bevy::prelude::*;
use crate::colours;

const TILE_SIZE: f32 = 40.;
const TILE_GAP: f32 = 10.;

#[derive(Component, Copy, Clone)]
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

  let physical_board_size =
    f32::from(board.size) * TILE_SIZE + f32::from(board.size+1) * TILE_GAP;

  commands.spawn((
    SpriteBundle {
      sprite: Sprite {
        color: colours::BOARD,
        custom_size: Some(vec2(
          physical_board_size, physical_board_size
        )),
        ..default()
      },
      ..default()
    },
    board
  )).with_children(|builder| {
    let bottom_left = -physical_board_size / 2. + (TILE_SIZE*0.5);

    for y in 0..board.size {
      for x in 0..board.size {
        let tile_x = bottom_left + f32::from(x)*TILE_SIZE + f32::from(x+1)*TILE_GAP;
        let tile_y = bottom_left + f32::from(y)*TILE_SIZE + f32::from(y+1)*TILE_GAP;
        builder.spawn(
          SpriteBundle {
            sprite: Sprite {
              color: colours::TILE_PLACEHOLDER,
              custom_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
              ..default()
            },
            transform: Transform::from_xyz(tile_x, tile_y, 1.),
            ..default()
          }
        );
      };
    };
  });
}
