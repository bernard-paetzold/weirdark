use rltk::{RGB, RGBA};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
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