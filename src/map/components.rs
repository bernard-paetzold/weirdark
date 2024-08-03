use crate::vectors::Vector3i;

use rltk::{RGB, RGBA};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
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
    pub name: String,
}

impl Tile {
    pub fn new(position: Vector3i, passable: bool, opaque: bool, top_glyph: rltk::FontCharType, side_glyph: rltk::FontCharType, foreground: RGBA, background: RGBA, name: String) -> Tile {
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
            name,
        }
    }
}

/*impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("Tile", 10)?;
    
        //Convert colors
        let foreground_s: SerialisableRGBA = self.foreground.into();
        let background_s: SerialisableRGBA = self.foreground.into();
        let light_color_s: SerialisableRGBA = self.foreground.into();

        state.serialize_field("position", &self.position)?;
        state.serialize_field("passable", &self.passable)?;
        state.serialize_field("opaque", &self.opaque)?;
        state.serialize_field("top_glyph", &self.top_glyph)?;
        state.serialize_field("side_glyph", &self.side_glyph)?;
        state.serialize_field("foreground", &foreground_s)?;
        state.serialize_field("background", &background_s)?;
        state.serialize_field("light_level", &self.light_level)?;
        state.serialize_field("light_color", &light_color_s)?;
        state.serialize_field("name", &self.name)?;

        state.end()
    }
}*/