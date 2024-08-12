use crate::graphics::char_to_glyph;
use crate::Renderable;
use crate::Photometry;
use rltk::to_cp437;
use rltk::RGB;
use specs_derive::Component;
use rltk::RGBA;
use serde::{Deserialize, Serialize};
use specs::prelude::*;


#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub passable: bool,
    pub opaque: bool,
    pub renderable: Renderable, 
    pub name: String,
    pub photometry: Photometry,
}

impl Tile {
    pub fn new(passable: bool, opaque: bool, top_glyph: rltk::FontCharType, side_glyph: rltk::FontCharType, foreground: RGBA, background: RGBA, name: String) -> Tile {
        Tile {
            passable,
            opaque,
            renderable: Renderable::new(top_glyph, side_glyph, foreground, background),
            photometry: Photometry::new(),
            name,
        }
    }
    pub fn new_empty() -> Tile {
        Tile {
            passable: true,
            opaque: false,
            renderable: Renderable::new(char_to_glyph(' '), char_to_glyph(' '), RGB::named(rltk::WHITE).to_rgba(0.0), RGB::named(rltk::WHITE).to_rgba(0.0)),
            photometry: Photometry::new(),
            name: "empty space".to_string(),
        }
    }
}

