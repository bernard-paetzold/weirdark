use rltk::RGBA;
use specs::prelude::*;
use specs::saveload::Marker;
use specs::saveload::ConvertSaveload;
use specs::error::NoError;
use serde::Serialize;
use serde::Deserialize;

use specs_derive::{Component, ConvertSaveload};

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct Renderable {
    pub top_glyph: rltk::FontCharType,
    pub side_glyph: rltk::FontCharType,
    pub foreground: RGBA,
    pub background: RGBA,
}

impl Renderable {
    pub fn new(top_glyph: rltk::FontCharType, side_glyph: rltk::FontCharType, foreground: RGBA, background: RGBA) -> Renderable {
        Renderable {
            top_glyph,
            side_glyph,
            foreground,
            background,
        }
    }
}