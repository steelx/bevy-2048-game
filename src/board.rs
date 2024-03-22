use bevy::math::vec2;
use bevy::prelude::*;
use crate::colours;

pub const TILE_SIZE: f32 = 40.;
pub const TILE_GAP: f32 = 10.;

#[derive(Component, Copy, Clone)]
pub struct Board {
  pub size: u32,
  pub size_in_pixels: f32
}

impl Board {
  pub fn new(size: u32) -> Self {
    let size_in_pixels =
      (size as f32) * TILE_SIZE + (size+1) as f32 * TILE_GAP;
    Self {
      size, size_in_pixels
    }
  }
  
  pub fn tile_to_pixels(&self, tile: u32) -> f32 {
    let bottom_left = -self.size_in_pixels / 2. + (TILE_SIZE*0.5);
    bottom_left
      + (tile as f32)*TILE_SIZE
      + (tile+1) as f32 *TILE_GAP
  }
}

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_board);
  }
}

fn setup_board(mut commands: Commands) {
  let board = Board::new(4);

  commands.spawn((
    SpriteBundle {
      sprite: Sprite {
        color: colours::BOARD,
        custom_size: Some(vec2(
          board.size_in_pixels, board.size_in_pixels
        )),
        ..default()
      },
      ..default()
    },
    board
  )).with_children(|builder| {
    for y in 0..board.size {
      for x in 0..board.size {
        let tile_x = board.tile_to_pixels(x);
        let tile_y = board.tile_to_pixels(y);
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
