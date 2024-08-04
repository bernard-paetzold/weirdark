use rltk::RGB;

use crate::{vectors::Vector3i, Map, Tile};

use super::MapBuilder;

pub struct SystemTestMapBuilder {}

impl MapBuilder for SystemTestMapBuilder {
    fn build(map_size: Vector3i) -> Map {
        let mut map = Map::new();

        for x in -map_size.x..map_size.x {
            for y in -map_size.y..map_size.y {
                for z in -3..4 {
                    if z == 0 {
                        map.tiles.insert(
                            Vector3i::new(x, y, z),
                            Tile::new(
                                Vector3i::new(x, y, z),
                                false,
                                true,
                                rltk::to_cp437('▓'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "White wall".to_string(),
                            ),
                        );
                    }
                    else {
                        map.tiles.insert(
                            Vector3i::new(x, y, z),
                            Tile::new(
                                Vector3i::new(x, y, z),
                                true,
                                false,
                                rltk::to_cp437(' '),
                                rltk::to_cp437(' '),
                                RGB::named(rltk::BLACK).to_rgba(0.0),
                                RGB::named(rltk::BLACK).to_rgba(0.0),
                                "Empty space".to_string(),
                            ),
                        );
                    }
                }
            }
        }

        map
    }
}
