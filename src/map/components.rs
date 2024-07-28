use crate::vectors::Vector3i;

use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct Tile {
    pub position: Vector3i,
    pub passable: bool,
    pub opaque: bool,
    pub top_glyph: rltk::FontCharType,
    pub side_glyph: rltk::FontCharType,
    pub foreground: RGB,
    pub background: RGB,
}

impl Tile {
    pub fn new(position: Vector3i, passable: bool, opaque: bool, top_glyph: rltk::FontCharType, side_glyph: rltk::FontCharType, foreground: RGB, background: RGB) -> Tile {
        Tile {
            position,
            passable,
            opaque,
            top_glyph,
            side_glyph,
            foreground,
            background,
        }
    }
}