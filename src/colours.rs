use bevy::prelude::*;


/// Color picker
/// https://colorjs.io/apps/picker/
pub const BOARD: Color = Color::Lcha {
  lightness: 0.08, chroma: 0.088, hue: 5., alpha: 1.,
};

pub const TILE_PLACEHOLDER: Color = Color::Lcha {
  lightness: 0.5, chroma: 0.4, hue: 0.8, alpha: 0.7,
};

pub const TILE: Color = Color::Lcha {
  lightness: 0.85, chroma: 0.4, hue: 0.8, alpha: 1.0,
};
