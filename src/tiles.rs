use bevy::math::vec2;
use bevy::prelude::*;
use rand::seq::IteratorRandom;

use crate::board::{Board, TILE_SIZE};
use crate::colours;

#[derive(Component)]
pub struct TileText;
#[derive(Component, Debug)]
pub struct Tile {
  pub points: u32
}

#[derive(Component, Debug)]
pub struct TilePosition {
  pub y: u32,
  pub x: u32,
}


pub struct TilesPlugin;

impl Plugin for TilesPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(PostStartup, spawn_tiles)
      .add_systems(Update, render_tile_points);
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
    let tile_position = TilePosition { x: u32::from(x), y: u32::from(y) };
    let tile_x = board.tile_to_pixels(x);
    let tile_y = board.tile_to_pixels(y);
    commands
      .spawn((
        Tile { points: 2 },
        tile_position,
        SpriteBundle {
          sprite: Sprite {
            color: colours::TILE,
            custom_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
            ..default()
          },
          transform: Transform::from_xyz(tile_x, tile_y, 1.),
          ..default()
        },
      ))
      .with_children(|builder| {
        builder
          .spawn((
            TileText,
            Text2dBundle {
              text: Text::from_section("0", TextStyle {
                font_size: 32., color: Color::MIDNIGHT_BLUE, ..default()
              }),
              transform: Transform::from_xyz(0., 0., 1.),
              ..default()
            }
          ));
      });
    }
}

fn render_tile_points(
  mut texts: Query<&mut Text, With<TileText>>,
  tiles: Query<(&Tile, &Children)>
) {
  for (tile, children) in tiles.iter() {
    if let Some(entity) = children.first() {
      let mut text = texts.get_mut(*entity).expect("Text or Text2dBundle missing");
      let text_section = text.sections.first_mut().expect("Text section");
      text_section.value = tile.points.to_string();
    }
  }
}
