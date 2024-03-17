use bevy::math::vec2;
use bevy::prelude::*;
use rand::seq::IteratorRandom;
use crate::board::{Board, TILE_SIZE};
use crate::colours;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(PostStartup, spawn_tiles);
  }
}

fn spawn_tiles(mut commands: Commands, board_query: Query<&Board>) {
  let board = board_query.single();
  
  let mut rng = rand::thread_rng();
  // Get all possible tile indices as an iterator
  let all_indices = (0..board.size)
    .flat_map(|x| (0..board.size)
      .map(move |y| (x, y)));

  // Choose 2 random elements from the iterator using IteratorRandom
  let random_indices = all_indices.choose_multiple(&mut rng, 2);

  for (x, y) in random_indices {
    let tile_x = board.tile_to_pixels(x);
    let tile_y = board.tile_to_pixels(y);
    commands.spawn(
      SpriteBundle {
        sprite: Sprite {
          color: colours::TILE,
          custom_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
          ..default()
        },
        transform: Transform::from_xyz(tile_x, tile_y, 1.),
        ..default()
      },
    );
  }
  
}