use rltk::RGB;
use specs::World;

use crate::{spawner, vectors::Vector3i, Map, Tile};

use super::MapBuilder;

pub struct SystemTestMapBuilder {
    map: Map,
    start_position: Vector3i,
}

impl SystemTestMapBuilder {
    pub fn new(start_position: Vector3i) -> SystemTestMapBuilder {
        SystemTestMapBuilder {
            map: Map::new(),
            start_position,
        }
    }

    pub fn rooms_and_corridor(&mut self) {
        const ROOM_SIZE: i32 = 22;
        const CORRIDOR_LENGTH: i32 = 21;
        const CORRIDOR_WIDTH: i32 = 5;

        //Rooms
        for x in self.start_position.x - ROOM_SIZE / 2..self.start_position.x + ROOM_SIZE / 2 + 1 {
            for y in self.start_position.y - ROOM_SIZE / 2..self.start_position.y + ROOM_SIZE / 2 + 1 {
                for z in self.start_position.z..self.start_position.z + 3 {
                    if z == self.start_position.z {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            Tile::new(
                                Vector3i::new(x, y, z),
                                false,
                                true,
                                rltk::to_cp437('░'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "Hull section".to_string(),
                            ),
                        );

                        self.map.tiles.insert(
                            Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z),
                            Tile::new(
                                Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z),
                                false,
                                true,
                                rltk::to_cp437('░'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "Hull section".to_string(),
                            ),
                        );
                    }
                    else {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            Tile::new(
                                Vector3i::new(x, y, z),
                                true,
                                false,
                                rltk::to_cp437(' '),
                                rltk::to_cp437(' '),
                                RGB::named(rltk::WHITE).to_rgba(0.0),
                                RGB::named(rltk::BLACK).to_rgba(0.0),
                                "Open space".to_string(),
                            ),
                        );
                        
                        self.map.tiles.insert(
                            Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z),
                            Tile::new(
                                Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z),
                                true,
                                false,
                                rltk::to_cp437(' '),
                                rltk::to_cp437(' '),
                                RGB::named(rltk::WHITE).to_rgba(0.0),
                                RGB::named(rltk::BLACK).to_rgba(0.0),
                                "Open space".to_string(),
                            ),
                        );
                    }

                    //Walls 
                    if (x == self.start_position.x - ROOM_SIZE / 2) ||
                    (x == self.start_position.x + ROOM_SIZE / 2) || 

                    (y == self.start_position.y - ROOM_SIZE / 2) ||
                    (y == self.start_position.y + ROOM_SIZE / 2) {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            Tile::new(
                                Vector3i::new(x, y, z),
                                false,
                                true,
                                rltk::to_cp437('░'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "Hull section".to_string(),
                            ),
                        ); 

                        self.map.tiles.insert(
                            Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z),
                            Tile::new(
                                Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z),
                                false,
                                true,
                                rltk::to_cp437('░'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "Hull section".to_string(),
                            ),
                        );

                        self.map.tiles.insert(
                            Vector3i::new(x, y, z + 1),
                            Tile::new(
                                Vector3i::new(x, y, z + 1),
                                false,
                                true,
                                rltk::to_cp437('░'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "Hull section".to_string(),
                            ),
                        ); 

                        self.map.tiles.insert(
                            Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z + 1),
                            Tile::new(
                                Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z + 1),
                                false,
                                true,
                                rltk::to_cp437('░'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "Hull section".to_string(),
                            ),
                        );
                    }
                } 
            }
        }

        //Corridor
        for x in self.start_position.x + ROOM_SIZE / 2..self.start_position.x + ROOM_SIZE / 2 + CORRIDOR_LENGTH + 1 {
            for y in self.start_position.y - CORRIDOR_WIDTH / 2..self.start_position.y + CORRIDOR_WIDTH / 2 + 1 {
                for z in self.start_position.z..self.start_position.z + 3 {
                    if y == self.start_position.y - CORRIDOR_WIDTH / 2 || y == self.start_position.y + CORRIDOR_WIDTH / 2 {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            Tile::new(
                                Vector3i::new(x, y, z),
                                false,
                                true,
                                rltk::to_cp437('░'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "Hull section".to_string(),
                            ),
                        );
                        
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z + 1),
                            Tile::new(
                                Vector3i::new(x, y, z),
                                false,
                                true,
                                rltk::to_cp437('░'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "Hull section".to_string(),
                            ),
                        );
                    }
                    else if z == self.start_position.z {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            Tile::new(
                                Vector3i::new(x, y, z),
                                false,
                                true,
                                rltk::to_cp437('░'),
                                rltk::to_cp437('█'),
                                RGB::named(rltk::WHITE).to_rgba(1.0),
                                RGB::named(rltk::BLACK).to_rgba(1.0),
                                "Hull section".to_string(),
                            ),
                        );
                    }
                    else {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            Tile::new(
                                Vector3i::new(x, y, z),
                                true,
                                false,
                                rltk::to_cp437(' '),
                                rltk::to_cp437(' '),
                                RGB::named(rltk::WHITE).to_rgba(0.0),
                                RGB::named(rltk::BLACK).to_rgba(0.0),
                                "Open space".to_string(),
                            ),
                        );
                    }         
                } 
            }
        }

    }
}

impl MapBuilder for SystemTestMapBuilder {
    fn build_map(&mut self) {
        self.rooms_and_corridor();
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        //Add a light
        spawner::ceiling_lamp(ecs, "Room 1 ceiling lamp".to_string(), Vector3i::new(self.start_position.x, self.start_position.y, self.start_position.z + 2), 1.0, RGB::named(rltk::RED).to_rgba(1.0));
        spawner::standing_lamp(ecs,"Room 1 standing lamp".to_string(), Vector3i::new(self.start_position.x - 7, self.start_position.y - 7, self.start_position.z + 1), 0.5, RGB::named(rltk::WHITE).to_rgba(1.0));
        spawner::ceiling_lamp(ecs,"Room 2 ceiling lamp".to_string(), Vector3i::new(self.start_position.x + 43, self.start_position.y, self.start_position.z + 2), 1.0,RGB::named(rltk::BLUE).to_rgba(1.0));
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_start_position(&mut self) -> Vector3i {
        self.start_position.clone()
    }
}
