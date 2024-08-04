use rltk::RGB;

use crate::{vectors::Vector3i, Map, Tile};

use super::MapBuilder;

pub struct EmptyPlainMapBuilder {}

impl MapBuilder for EmptyPlainMapBuilder {
    fn build(map_size: Vector3i) -> Map {
        let mut map = Map::new();

        //Test plain
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
                    } else {
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

        for x in -13..0 {
            for y in 0..13 {
                let z = 1;

                if (x == -13 || x == -1) && !(x == -13 && y == 6) && !(x == -1 && y == 6)  {
                    map.tiles.remove(&Vector3i::new(x, y, z));
                    map.tiles.insert(
                        Vector3i::new(x, y, z),
                        Tile::new(
                            Vector3i::new(x, y, z),
                            false,
                            true,
                            rltk::to_cp437('▓'),
                            rltk::to_cp437('█'),
                            RGB::named(rltk::GREEN).to_rgba(1.0),
                            RGB::named(rltk::BLACK).to_rgba(1.0),
                            "Green wall".to_string(),
                        ),
                    );
                } 
                else if (y == 0 || y == 12) && !(x == -7 && y == 0) && !(x == -6 || x == -7 || x == -8) {
                    map.tiles.remove(&Vector3i::new(x, y, z));
                    map.tiles.insert(
                        Vector3i::new(x, y, z),
                        Tile::new(
                            Vector3i::new(x, y, z),
                            false,
                            true,
                            rltk::to_cp437('▓'),
                            rltk::to_cp437('█'),
                            RGB::named(rltk::GREEN).to_rgba(1.0),
                            RGB::named(rltk::BLACK).to_rgba(1.0),
                            "Green wall".to_string(),
                        ),
                    );
                }
                else if (x == -13 && y == 6) || (x == -1 && y == 6) || (x == -6 && y == 12) || (x == -7 && y == 12) || (x == -8 && y == 12) {
                    map.tiles.remove(&Vector3i::new(x, y, z));
                    map.tiles.insert(
                        Vector3i::new(x, y, z),
                        Tile::new(
                            Vector3i::new(x, y, z),
                            false,
                            false,
                            rltk::to_cp437('▓'),
                            rltk::to_cp437('█'),
                            RGB::named(rltk::BLUE).to_rgba(0.1),
                            RGB::named(rltk::BLACK).to_rgba(0.0),
                            "Glass wall".to_string(),
                        ),
                    );
                }
            }
        }
        for y in -map_size.y..map_size.y {
            if y % 5 == 0 {
                map.tiles.remove(&Vector3i::new(13, y, 1));
                map.tiles.insert(
                    Vector3i::new(13, y, 1),
                    Tile::new(
                        Vector3i::new(13, y, 1),
                        false,
                        true,
                        rltk::to_cp437('O'),
                        rltk::to_cp437('O'),
                        RGB::named(rltk::ORANGE).to_rgba(1.0),
                        RGB::named(rltk::BLACK).to_rgba(1.0),
                        "Orange pillar".to_string(),
                    ),
                );
            }
        }
        map
    }
}
