use crate::entities::atmospherics::Atmosphere;
use crate::graphics::char_to_glyph;
use crate::Photometry;
use crate::Renderable;
use rltk::RGB;
use rltk::RGBA;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub passable: bool,
    pub opaque: bool,
    pub renderable: Renderable,
    pub name: String,
    pub airtight: bool,
    pub photometry: Photometry,
    pub atmosphere: Atmosphere,
}

impl Tile {
    pub fn new(
        passable: bool,
        opaque: bool,
        top_glyph: u16,
        side_glyph: u16,
        foreground: RGBA,
        background: RGBA,
        airtight: bool,
        atmosphere: Atmosphere,
        name: String,
        visible: bool,
    ) -> Tile {
        Tile {
            passable,
            opaque,
            renderable: Renderable::new(top_glyph, side_glyph, foreground, background, visible),
            photometry: Photometry::new(),
            atmosphere,
            name,
            airtight,
        }
    }
    pub fn new_empty_stp() -> Tile {
        Tile {
            passable: true,
            opaque: false,
            renderable: Renderable::new(
                char_to_glyph(' '),
                char_to_glyph(' '),
                RGB::named(rltk::WHITE).to_rgba(0.0),
                RGB::named(rltk::WHITE).to_rgba(0.0),
                false,
            ),
            photometry: Photometry::new(),
            atmosphere: Atmosphere::new_stp(),
            name: "Empty space".to_string(),
            airtight: false,
        }
    }
    #[allow(dead_code)]
    pub fn new_vacuume() -> Tile {
        Tile {
            passable: true,
            opaque: false,
            renderable: Renderable::new(
                char_to_glyph(' '),
                char_to_glyph(' '),
                RGB::named(rltk::WHITE).to_rgba(0.0),
                RGB::named(rltk::WHITE).to_rgba(0.0),
                false,
            ),
            photometry: Photometry::new(),
            atmosphere: Atmosphere::new_vacuume(),
            name: "Vacuume".to_string(),
            airtight: false,
        }
    }
}
