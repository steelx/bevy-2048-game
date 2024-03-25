use std::cmp::PartialEq;
use std::time::Duration;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy_easings::*;
use rand::seq::IteratorRandom;

use crate::board::{Board, TILE_SIZE};
use crate::colours;
use crate::game_states::GameState;

#[derive(Component)]
pub struct TileText;
#[derive(Component, Debug, PartialEq)]
pub struct Tile {
  pub points: u32
}

#[derive(Component, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct TilePosition {
  pub y: u32,
  pub x: u32,
}

#[derive(Event)]
pub struct NewTileEvent;

pub struct TilesPlugin;

impl Plugin for TilesPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_event::<NewTileEvent>()
      .add_plugins(EasingsPlugin)
      .add_systems(PostStartup, spawn_tiles)
      .add_systems(Update, (render_tile_points, render_tiles_position, new_tile_event_handler).run_if(in_state(GameState::Playing)));
  }
}

fn spawn_tiles(mut commands: Commands, board_query: Query<&Board>) {
  let board = board_query.single();
  
  let mut rng = rand::thread_rng();
  // Get all possible tile indices (cartesian product [x,y]) as an iterator
  let all_indices = (0..board.size)
    .flat_map(|x| (0..board.size)
      .map(move |y| (x, y)));

  // Choose 2 random elements from the iterator using IteratorRandom
  let random_indices = all_indices.choose_multiple(&mut rng, 2);
  
  for (x, y) in random_indices {
    let tile_position = TilePosition { x: u32::from(x), y: u32::from(y) };
    spawn_tile_util(&mut commands, board, tile_position);
  }
}

fn new_tile_event_handler(
  mut commands: Commands,
  mut new_tile_event_reader: EventReader<NewTileEvent>,
  tiles: Query<&TilePosition>,
  board_query: Query<&Board>,
) {
  let board = board_query.single();
  for _event in new_tile_event_reader.read() {
    let mut rng = rand::thread_rng();
    // cartesian product [x,y]
    let all_indices = (0..board.size)
      .flat_map(|x| (0..board.size)
        .map(move |y| (x, y)));

    let possible_position = all_indices
      .filter_map(|tile_pos| {
        let new_pos = TilePosition { x: tile_pos.0, y: tile_pos.1 };
        match tiles.iter().find(|&&pos| pos == new_pos) {
          Some(_) => None,
          None => Some(new_pos)
        }
      })
      .choose(&mut rng);

    if let Some(tile_position) = possible_position {
      spawn_tile_util(&mut commands, board, tile_position);
    }
  }
}

fn spawn_tile_util(
  commands: &mut Commands,
  board: &Board,
  tile_position: TilePosition
) {
  let tile_x = board.tile_to_pixels(tile_position.x);
  let tile_y = board.tile_to_pixels(tile_position.y);
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

fn render_tiles_position(
  mut commands: Commands,
  mut tiles: Query<(Entity, &Transform, &TilePosition), Changed<TilePosition>>,
  board_query: Query<&Board>,
) {
  let board = board_query.single();
  for (entity, transform, pos) in tiles.iter_mut() {
    let x = board.tile_to_pixels(pos.x);
    let y = board.tile_to_pixels(pos.y);
    commands.entity(entity).insert(transform.ease_to(
      Transform::from_xyz(x, y, transform.translation.z),
      EaseFunction::QuadraticInOut,
      EasingType::Once {
        duration: Duration::from_millis(100),
      }
    ));
  }
}
