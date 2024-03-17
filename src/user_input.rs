use bevy::prelude::*;

pub struct UserInputPlugin;

impl Plugin for UserInputPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Update, update_user_input);
  }
}

fn update_user_input(keyboard_input: Res<ButtonInput<KeyCode>>) {
  let user_input = keyboard_input.get_just_pressed().find_map(|value| {
    UserInput::try_from(value).ok()
  });
  
  if let Some(key) = user_input {
    println!("{:?}", key);
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