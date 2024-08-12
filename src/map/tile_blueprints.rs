use std::{collections::HashMap, sync::Mutex};
use rltk::prelude::*;
use crate::{graphics::char_to_glyph, Tile};
use lazy_static::lazy_static;

lazy_static! {
    static ref TILES: Mutex<HashMap<String, Tile>> = Mutex::new(HashMap::new());
}

pub fn initalise() {
    TILES.lock().unwrap().insert("glass_hull".to_string(), 
        Tile::new(false,false,
        char_to_glyph('█'),
        char_to_glyph('█'),
        RGB::named(rltk::SKYBLUE).to_rgba(0.5),
        RGB::named(rltk::BLACK).to_rgba(1.0),
        "Glass hull section".to_string()));

    TILES.lock().unwrap().insert("hull".to_string(), 
    Tile::new(
        false,
        true,
        char_to_glyph('░'),
        char_to_glyph('█'),
        RGB::named(rltk::WHITE).to_rgba(1.0),
        RGB::named(rltk::BLACK).to_rgba(1.0),
        "Hull section".to_string(),
    ));
    
    TILES.lock().unwrap().insert("open_space".to_string(), 
    Tile::new(
        true,
        false,
        char_to_glyph(' '),
        char_to_glyph(' '),
        RGB::named(rltk::WHITE).to_rgba(0.0),
        RGB::named(rltk::BLACK).to_rgba(0.0),
        "Open space".to_string()));
}

pub fn get_tile(tile_type: &str) -> Option<Tile> {
    TILES.lock().unwrap().get(tile_type).cloned()
}