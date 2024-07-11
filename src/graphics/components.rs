use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
pub struct Renderable {
    pub top_glyph: rltk::FontCharType,
    pub side_glyph: rltk::FontCharType,
    pub foreground: RGB,
    pub background: RGB,
}

impl Renderable {
    pub fn new(top_glyph: rltk::FontCharType, side_glyph: rltk::FontCharType, foreground: RGB, background: RGB) -> Renderable {
        Renderable {
            top_glyph,
            side_glyph,
            foreground,
            background,
        }
    }
}