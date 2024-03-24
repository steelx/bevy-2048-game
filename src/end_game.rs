use std::collections::HashMap;
use bevy::prelude::*;
use crate::board::Board;
use crate::game_states::GameState;
use crate::tiles::{Tile, TilePosition};

pub struct EndGamePlugin;

impl Plugin for EndGamePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, has_game_ended);
  }
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn has_game_ended(
  tiles: Query<(&TilePosition, &Tile)>,
  board_query: Query<&Board>,
  mut next_state: ResMut<NextState<GameState>>,
) {
  let board = board_query.single();

  if tiles.iter().len() == board.size.pow(2) as usize {
    let tiles_map = tiles.iter()
      .collect::<HashMap<&TilePosition, &Tile>>();

    let board_range = 0..(board.size as i32);

    // We iterate over all Tiles such that to check their neighbour tiles points
    let has_move = tiles.iter().any(
      |(TilePosition { x, y }, tile)| {
        DIRECTIONS
          .iter()
          .filter_map(|(x1, y1)| {
            let new_x = *x1 + *x as i32;
            let new_y = *y1 + *y as i32;

            if !board_range.contains(&new_x) || !board_range.contains(&new_y) {
              return None;
            }

            tiles_map.get(&TilePosition {
              x: new_x.try_into().unwrap(),
              y: new_y.try_into().unwrap(),
            })
          })
          // if two tiles have equal point means they can be merged
          // hence we can still move the board
          .any(|&v| v.points == tile.points)
      }
    );

    if !has_move {
      dbg!("Game Over!");
      next_state.set(GameState::GameOver);
    }
  }
}