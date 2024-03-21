use std::cmp::Ordering;
use itertools::Itertools;
use bevy::prelude::*;
use crate::tiles::{Tile, TilePosition};

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
  mut tiles: Query<(Entity, &mut TilePosition, &mut Tile)>
) {
  let user_input = keyboard_input.get_just_pressed().find_map(|value| {
    UserInput::try_from(value).ok()
  });
  
  match user_input {
    Some(UserInput::Left) => {
      let mut it = tiles.iter_mut()
        .sorted_by(|a, b| {
          match Ord::cmp(&a.1.y, &b.1.y) {
            Ordering::Equal => Ord::cmp(&a.1.x, &b.1.x), // if Equal means 2 tiles are in a row
            ordering => ordering
          }
        })
        .peekable();
      // dbg!(it.collect::<Vec<_>>());
      
      let mut column: u32 = 0;
      // loop over all tiles from bottom-left to right (next column)
      // then move to next row and go column-wise
      // if tiles are in same row we merge them, while despawn the 2nd tile
      while let Some(mut tile) = it.next() {
        tile.1.x = column;// move tile to left most
        match it.peek() {
          None => {},
          Some(tile_next) => {
            // check y rows
            if tile_next.1.y != tile.1.y {
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
                if next_next_tile.1.y != tile.1.y {
                  column = 0;
                } else {
                  column += 1;
                }
              }
            }
          }
        }
      }
    }
    Some(UserInput::Right) => {
      dbg!("right");
    }
    Some(UserInput::Up) => {
      dbg!("up");
    }
    Some(UserInput::Down) => {
      dbg!("down");
    }
    None => ()
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