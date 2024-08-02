use crate::vectors::Vector3i;

use rltk::{RGB, RGBA};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct Tile {
    pub position: Vector3i,
    pub passable: bool,
    pub opaque: bool,
    pub top_glyph: rltk::FontCharType,
    pub side_glyph: rltk::FontCharType,
    pub foreground: RGBA,
    pub background: RGBA,
    pub light_level: f32,
    pub light_color: RGBA,
}

impl Tile {
    pub fn new(position: Vector3i, passable: bool, opaque: bool, top_glyph: rltk::FontCharType, side_glyph: rltk::FontCharType, foreground: RGBA, background: RGBA) -> Tile {
        Tile {
            position,
            passable,
            opaque,
            top_glyph,
            side_glyph,
            foreground,
            background,
            light_level: 0.0,
            light_color: RGB::named(rltk::WHITE).to_rgba(1.0),
        }
    }
}