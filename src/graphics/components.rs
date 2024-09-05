use rltk::FontCharType;
use rltk::RGBA;
use serde::Deserialize;
use serde::Serialize;
use specs::prelude::*;

use specs_derive::Component;

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct Renderable {
    pub top_glyph: FontCharType,
    pub side_glyph: FontCharType,
    pub foreground: RGBA,
    pub background: RGBA,
    pub visible: bool,
}

impl Renderable {
    pub fn new(
        top_glyph: u16,
        side_glyph: u16,
        foreground: RGBA,
        background: RGBA,
        visible: bool,
    ) -> Renderable {
        Renderable {
            top_glyph,
            side_glyph,
            foreground,
            background,
            visible,
        }
    }
}
