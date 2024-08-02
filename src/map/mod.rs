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

    //Test corridor
    /*for x in 0..5 {
        for y in -map_size.y..map_size.y {
            for z in -3..4 {
                if z == 0 || z == 3 {
                    map.tiles.insert(Vector3i::new(x,y,z),
                    Tile::new(Vector3i::new(x, y, z), 
                    false,
                    true,
                    rltk::to_cp437('▓'), 
                    rltk::to_cp437('█'),
                    RGB::named(rltk::WHITE).to_rgba(1.0),
                    RGB::named(rltk::BLACK).to_rgba(1.0))); 
                }
                else if (z == 1 || z == 2) && (x == 0 || x == 4) {
                    map.tiles.insert(Vector3i::new(x,y,z),
                    Tile::new(Vector3i::new(x, y, z), 
                    false,
                    true,
                    rltk::to_cp437('▓'), 
                    rltk::to_cp437('█'),
                    RGB::named(rltk::WHITE).to_rgba(1.0),
                    RGB::named(rltk::BLACK).to_rgba(1.0))); 
                }            
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
    }*/

    //Test plain
    for x in -map_size.x..map_size.x {
        for y in -map_size.y..map_size.y {
            for z in -3..4 {
                if z == 0 {
                    map.tiles.insert(Vector3i::new(x,y,z),
                    Tile::new(Vector3i::new(x, y, z), 
                    false,
                    true,
                    rltk::to_cp437('▓'), 
                    rltk::to_cp437('█'),
                    RGB::named(rltk::WHITE).to_rgba(1.0),
                    RGB::named(rltk::BLACK).to_rgba(1.0))); 
                }         
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
    
    for x in -13..0 {
        for y in 0..13 {
            let z = 1;

            if x == -13 || x == -1 {
                map.tiles.remove(&Vector3i::new(x,y,z));
                map.tiles.insert(Vector3i::new(x,y,z),
                Tile::new(Vector3i::new(x, y, z), 
                false,
                true,
                rltk::to_cp437('▓'), 
                rltk::to_cp437('█'),
                RGB::named(rltk::GREEN).to_rgba(1.0),
                RGB::named(rltk::BLACK).to_rgba(1.0))); 
            }         
            else if y == 0 || y == 12 {
                map.tiles.remove(&Vector3i::new(x,y,z));
                map.tiles.insert(Vector3i::new(x,y,z),
                Tile::new(Vector3i::new(x, y, z), 
                false,
                true,
                rltk::to_cp437('▓'), 
                rltk::to_cp437('█'),
                RGB::named(rltk::GREEN).to_rgba(1.0),
                RGB::named(rltk::BLACK).to_rgba(1.0))); 
            }         
            
        }
    }
    for y in -map_size.y..map_size.y {
        if y % 5 == 0 {
            map.tiles.remove(&Vector3i::new(13,y,1));
                map.tiles.insert(Vector3i::new(13,y,1),
                Tile::new(Vector3i::new(13,y,1), 
                false,
                true,
                rltk::to_cp437('O'), 
                rltk::to_cp437('O'),
                RGB::named(rltk::ORANGE).to_rgba(1.0),
                RGB::named(rltk::BLACK).to_rgba(1.0))); 
        }
    }


    map
}