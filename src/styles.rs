use bevy::prelude::*;

pub fn small_block_styles() -> Style {
  Style {
    width: Val::Vw(25.),
    height: Val::Vh(10.),
    flex_direction: FlexDirection::ColumnReverse,
    align_items: AlignItems::Center,
    justify_items: JustifyItems::Center,
    padding: UiRect::all(Val::Px(5.)),
    ..Style::DEFAULT
  }
}
