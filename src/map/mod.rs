use std::collections::HashMap;
use rltk::RGB;
use crate::{vectors::Vector3i, Tile};

pub struct Map {
    pub tiles: HashMap<Vector3i, Tile>,
}

impl Map {
    fn new() -> Map {
        Map { 
            tiles: HashMap::new() 
        }
    }
}

pub mod components;

pub fn initialise_map(map_size: Vector3i) -> Map {
    let mut map = Map::new();
    for x in -map_size.x..map_size.x {
        for y in -map_size.y..map_size.y {
            for z in map_size.z - 4..map_size.z - 1 {
                if z < map_size.z - 2 {
                    map.tiles.insert(Vector3i::new(x,y,z),
                    Tile::new(Vector3i::new(x, y, z), 
                    false,
                    true,
                    rltk::to_cp437('▓'), 
                    rltk::to_cp437('█'),
                    RGB::named(rltk::DARKRED).to_rgba(1.0),
                    RGB::named(rltk::BLACK).to_rgba(0.0)));
                }
                /*else if (x == 5 || x == 15) && y % 5 == 0 {
                    map.tiles.insert(Vector3i::new(x,y,z),
                    Tile::new(Vector3i::new(x, y, z), 
                    false,
                    true,
                    rltk::to_cp437('O'), 
                    rltk::to_cp437('0'),
                    RGB::named(rltk::WHITE).to_rgba(1.0),
                    RGB::named(rltk::BLACK).to_rgba(0.0)));
                }*/
                else {
                    map.tiles.insert(Vector3i::new(x,y,z),
                    Tile::new(Vector3i::new(x, y, z), 
                    true,
                    false,
                    rltk::to_cp437(' '), 
                    rltk::to_cp437(' '),
                    RGB::named(rltk::BLACK).to_rgba(0.0),
                    RGB::named(rltk::BLACK).to_rgba(0.0)));
                }
            }
        }
    }


    map
}