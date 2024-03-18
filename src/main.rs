mod camera;
mod board;
mod colours;
mod tiles;
mod user_input;

use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowResolution};

fn main() {
  App::new()
    .insert_resource(ClearColor(Color::hex("#371845").unwrap()))
    .add_plugins(DefaultPlugins.set(WindowPlugin {
      primary_window: Some(Window {
        title: "2048".to_string(),
        resolution: WindowResolution::new(500., 600.),
        ..default()
      }),
      ..default()
    }))
    .add_plugins(camera::CameraPlugin)
    .add_plugins(board::BoardPlugin)
    .add_plugins(tiles::TilesPlugin)
    .add_plugins(user_input::UserInputPlugin)
    .add_systems(Update, close_on_esc)
    .run();
}