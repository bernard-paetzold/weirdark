use std::collections::HashMap;
use rltk::RGB;
use crate::{vectors::Vector3i, Tile};

pub struct Map {
    pub tiles: HashMap<Vector3i, Tile>,
}

impl Map {
    fn new() -> Map {
        Map { tiles: HashMap::new() }
    }
}

pub mod components;

pub fn initialise_map(map_size: Vector3i) -> Map {
    let mut map = Map::new();

    for x in 0..map_size.x {
        for y in 0..map_size.y {
            for z in 0..map_size.z - 1 {
                map.tiles.insert(Vector3i::new(x,y,z),
                 Tile::new(Vector3i::new(x, y, z), 
                 false, 
                 rltk::to_cp437('.'), 
                 rltk::to_cp437('#'),
                 RGB::named(rltk::WHITE),
                 RGB::named(rltk::BLACK)));
            }
        }
    }

    map
}