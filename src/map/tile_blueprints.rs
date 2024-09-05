use crate::{entities::atmospherics::Atmosphere, graphics::char_to_glyph, Tile};
use lazy_static::lazy_static;
use rltk::prelude::*;
use std::{collections::HashMap, sync::Mutex};

lazy_static! {
    static ref TILES: Mutex<HashMap<String, Tile>> = Mutex::new(HashMap::new());
}

pub fn initalise() {
    TILES.lock().unwrap().insert(
        "glass_hull".to_string(),
        Tile::new(
            false,
            false,
            char_to_glyph('█'),
            char_to_glyph('█'),
            RGB::named(rltk::SKYBLUE).to_rgba(0.5),
            RGB::named(rltk::BLACK).to_rgba(1.0),
            true,
            Atmosphere::new_vacuume(),
            "Glass hull section".to_string(),
            true,
        ),
    );

    TILES.lock().unwrap().insert(
        "hull".to_string(),
        Tile::new(
            false,
            true,
            char_to_glyph('░'),
            char_to_glyph('█'),
            RGB::named(rltk::WHITE).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(1.0),
            true,
            Atmosphere::new_vacuume(),
            "Hull section".to_string(),
            true,
        ),
    );

    TILES.lock().unwrap().insert(
        "breathable_atmosphere".to_string(),
        Tile::new(
            true,
            false,
            char_to_glyph(' '),
            char_to_glyph(' '),
            RGB::named(rltk::WHITE).to_rgba(0.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
            false,
            Atmosphere::new_stp(),
            "Empty space".to_string(),
            false,
        ),
    );

    TILES.lock().unwrap().insert(
        "vacuume".to_string(),
        Tile::new(
            true,
            false,
            char_to_glyph(' '),
            char_to_glyph(' '),
            RGB::named(rltk::WHITE).to_rgba(0.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
            false,
            Atmosphere::new_vacuume(),
            "Vacuume".to_string(),
            false,
        ),
    );
}

pub fn get_tile(tile_type: &str) -> Option<Tile> {
    TILES.lock().unwrap().get(tile_type).cloned()
}
