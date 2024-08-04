use rltk::RGB;

use crate::{vectors::Vector3i, Map, Tile};

use super::MapBuilder;

pub struct SystemTestMapBuilder {}

impl MapBuilder for SystemTestMapBuilder {
    fn build(map_size: Vector3i) -> Map {
        let mut map = Map::new();



        map
    }
}
