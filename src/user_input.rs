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
  keyboard_input: Res<ButtonInput<KeyCode>>,
  mut tiles: Query<(Entity, &mut TilePosition, &mut Tile)>
) {
  let user_input = keyboard_input.get_just_pressed().find_map(|value| {
    UserInput::try_from(value).ok()
  });
  
  match user_input {
    Some(UserInput::Left) => {
      dbg!("left");
      let mut it = tiles.iter_mut().sorted_by(|a, b| {
        match Ord::cmp(&a.1.y, &b.1.y) {
          Ordering::Equal => Ord::cmp(&a.1.x, &b.1.x), // if Equal means 2 tiles are in a row
          ordering => ordering
        }
      });
      dbg!(it.collect::<Vec<_>>());
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