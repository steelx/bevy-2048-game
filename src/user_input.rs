use std::cmp::Ordering;
use itertools::Itertools;
use bevy::prelude::*;
use crate::board::Board;
use crate::tiles::{NewTileEvent, Tile, TilePosition};

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update_user_input);
  }
}

fn update_user_input(
  mut commands: Commands,
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut tiles: Query<(Entity, &mut TilePosition, &mut Tile)>,
  board_query: Query<&Board>,
  mut new_tile_event_writer: EventWriter<NewTileEvent>
) {
  let board = board_query.single();
  let user_input = keyboard_input.get_just_pressed().find_map(|value| {
    UserInput::try_from(value).ok()
  });
  
    if let Some(user_input) = user_input {
      let mut it = tiles.iter_mut()
        .sorted_by(|a, b| user_input.sorted_by_tile_position(&a.1, &b.1))
        .peekable();
      // dbg!(it.collect::<Vec<_>>());
      
      let mut column: u32 = 0;
      // loop over all tiles from bottom-left to right (next column)
      // then move to next row and go column-wise
      // if tiles are in same row we merge them, while despawn the 2nd tile
      while let Some(mut tile) = it.next() {
        user_input.set_column_position(&mut tile.1, column, board.size);
        if let Some(tile_next) = it.peek() {
          // check y rows
          if user_input.get_row_position(&tile_next.1) != user_input.get_row_position(&tile.1) {
            column = 0; // dont merge
          }
          // check values
          else if tile_next.2.points != tile.2.points {
            column += 1;// different values, dont merge
          }
          // all case match, merge
          else {
            let real_next_tile = it.next().expect("Tile should have been there!");
            tile.2.points += real_next_tile.2.points;
            commands.entity(real_next_tile.0).despawn_recursive();

            if let Some(next_next_tile) = it.peek() {
              if user_input.get_row_position(&next_next_tile.1) != user_input.get_row_position(&tile.1) {
                column = 0;
              } else {
                column += 1;
              }
            }
          }
        }
      }
      new_tile_event_writer.send(NewTileEvent);
    }
}

#[derive(Debug)]
pub enum UserInput {
  Left,
  Right,
  Up,
  Down
}

impl TryFrom<&KeyCode> for UserInput {
  type Error = &'static str;
  
  fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
    match value {
      KeyCode::KeyA |
      KeyCode::ArrowLeft => Ok(UserInput::Left),
      KeyCode::KeyD |
      KeyCode::ArrowRight => Ok(UserInput::Right),
      KeyCode::KeyW |
      KeyCode::ArrowUp => Ok(UserInput::Up),
      KeyCode::KeyS |
      KeyCode::ArrowDown => Ok(UserInput::Down),
      _ => Err("Invalid user input!")
    }
  }
}

impl UserInput {
  fn sorted_by_tile_position(&self, a: &TilePosition, b: &TilePosition) -> Ordering {
    match self { 
      UserInput::Left => {
        match Ord::cmp(&a.y, &b.y) {
          Ordering::Equal => Ord::cmp(&a.x, &b.x), // if Equal means 2 tiles are in a row
          ordering => ordering
        }
      }
      UserInput::Down => {
        match Ord::cmp(&a.x, &b.x) {
          Ordering::Equal => Ord::cmp(&a.y, &b.y), // if Equal means 2 tiles are in a row
          ordering => ordering
        }
      }
      UserInput::Right => {
        match Ord::cmp(&b.y, &a.y) {
          Ordering::Equal => Ord::cmp(&b.x, &a.x), // if Equal means 2 tiles are in a row
          ordering => ordering
        }
      }
      UserInput::Up => {
        match Ord::cmp(&b.x, &a.x) {
          Ordering::Equal => Ord::cmp(&b.y, &a.y), // if Equal means 2 tiles are in a row
          ordering => ordering
        }
      }
    }
  }

  fn set_column_position(&self, pos: &mut TilePosition, index: u32, board_size: u32) {
    match self {
      UserInput::Left => {
        pos.x = index;
      }
      UserInput::Right => {
        pos.x = board_size - 1 - index;
      }
      UserInput::Up => {
        pos.y = board_size - 1 - index;
      }
      UserInput::Down => {
        pos.y = index;
      }
    }
  }
  fn get_row_position(&self, pos: &TilePosition) -> u32 {
    match self {
      UserInput::Left | UserInput::Right => {pos.y}
      UserInput::Up | UserInput::Down => {pos.x}
    }
  }
}