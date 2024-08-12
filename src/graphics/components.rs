use rltk::FontCharType;
use rltk::RGBA;
use specs::prelude::*;
use serde::Serialize;
use serde::Deserialize;

use specs_derive::Component;

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct Renderable {
    pub top_glyph: FontCharType,
    pub side_glyph: FontCharType,
    pub foreground: RGBA,
    pub background: RGBA,
}

impl Renderable {
    pub fn new(top_glyph: u16, side_glyph: u16, foreground: RGBA, background: RGBA) -> Renderable {
        Renderable {
            top_glyph,
            side_glyph,
            foreground,
            background,
        }
    }
}