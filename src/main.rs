mod camera;
mod board;
mod colours;

use bevy::prelude::*;
use bevy::window::{close_on_esc, WindowResolution};

fn main() {
  App::new()
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
    .add_systems(Update, close_on_esc)
    .run();
}
