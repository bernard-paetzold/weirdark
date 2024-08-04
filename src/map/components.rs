use std::collections::HashMap;

use crate::Renderable;
use crate::{vectors::Vector3i, Photometry};
use specs_derive::{Component, ConvertSaveload};
use rltk::{RGB, RGBA};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs::error::NoError;
use specs::saveload::{Marker, ConvertSaveload};


#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub position: Vector3i,
    pub passable: bool,
    pub opaque: bool,
    pub renderable: Renderable, 
    pub name: String,
    pub photometry: Photometry,
}

impl Tile {
    pub fn new(position: Vector3i, passable: bool, opaque: bool, top_glyph: rltk::FontCharType, side_glyph: rltk::FontCharType, foreground: RGBA, background: RGBA, name: String) -> Tile {
        Tile {
            position,
            passable,
            opaque,
            renderable: Renderable::new(top_glyph, side_glyph, foreground, background),
            photometry: Photometry::new(),
            name,
        }
    }
}

