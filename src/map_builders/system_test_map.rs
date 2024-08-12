use rltk::{to_cp437, RGB};
use specs::World;

use crate::{graphics::char_to_glyph, spawner, vectors::Vector3i, Map, Tile};

use super::MapBuilder;

pub struct SystemTestMapBuilder {
    map: Map,
    start_position: Vector3i,
}

const ROOM_SIZE: i32 = 22;
const CORRIDOR_LENGTH: i32 = 21;
const CORRIDOR_WIDTH: i32 = 5;

impl SystemTestMapBuilder {
    pub fn new(start_position: Vector3i) -> SystemTestMapBuilder {
        SystemTestMapBuilder {
            map: Map::new(),
            start_position,
        }
    }

    pub fn rooms_and_corridor(&mut self) {
        let glass_wall: Tile = Tile::new(false,false,
            char_to_glyph('█'),
            char_to_glyph('█'),
            RGB::named(rltk::SKYBLUE).to_rgba(0.5),
            RGB::named(rltk::BLACK).to_rgba(1.0),
            "Glass hull section".to_string(),
        );
        
        let hull_section: Tile = Tile::new(
            false,
            true,
            char_to_glyph('░'),
            char_to_glyph('█'),
            RGB::named(rltk::WHITE).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(1.0),
            "Hull section".to_string(),
        );

        let open_space = Tile::new(
            true,
            false,
            char_to_glyph(' '),
            char_to_glyph(' '),
            RGB::named(rltk::WHITE).to_rgba(0.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
            "Open space".to_string(),
        );

        //Rooms
        for x in self.start_position.x - ROOM_SIZE / 2..self.start_position.x + ROOM_SIZE / 2 + 1 {
            for y in self.start_position.y - ROOM_SIZE / 2..self.start_position.y + ROOM_SIZE / 2 + 1 {
                for z in self.start_position.z..self.start_position.z + 3 {
                    if z == self.start_position.z {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            hull_section.clone()
                        );

                        self.map.tiles.insert(
                            Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z),
                            hull_section.clone(),
                        );
                    }
                    else {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            open_space.clone(),
                        );
                        
                        self.map.tiles.insert(
                            Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z),
                            open_space.clone(),
                        );
                    }

                    //Walls 
                    if (x == self.start_position.x - ROOM_SIZE / 2) ||
                    (x == self.start_position.x + ROOM_SIZE / 2) || 

                    (y == self.start_position.y - ROOM_SIZE / 2) ||
                    (y == self.start_position.y + ROOM_SIZE / 2) {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            hull_section.clone(),
                        ); 

                        self.map.tiles.insert(
                            Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z),
                            hull_section.clone(),
                        );

                        self.map.tiles.insert(
                            Vector3i::new(x, y, z + 1),
                            hull_section.clone(),
                        ); 

                        self.map.tiles.insert(
                            Vector3i::new(x + CORRIDOR_LENGTH + ROOM_SIZE, y, z + 1),
                            hull_section.clone(),
                        );
                    }
                } 
            }
        }

        //Corridor
        for x in self.start_position.x + ROOM_SIZE / 2..self.start_position.x + ROOM_SIZE / 2 + CORRIDOR_LENGTH + 1 {
            for y in self.start_position.y - CORRIDOR_WIDTH / 2..self.start_position.y + CORRIDOR_WIDTH / 2 + 1 {
                for z in self.start_position.z..self.start_position.z + 3 {
                    if (x > self.start_position.x + ROOM_SIZE / 2 && x < self.start_position.x + ROOM_SIZE / 2 + CORRIDOR_LENGTH) && (y == self.start_position.y - CORRIDOR_WIDTH / 2 || y == self.start_position.y + CORRIDOR_WIDTH / 2) {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            glass_wall.clone(),
                        );
                        
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z + 1),
                            glass_wall.clone(),
                        );
                    }
                    else if z == self.start_position.z {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            hull_section.clone(),
                        );
                    }
                    else if (x == self.start_position.x + ROOM_SIZE / 2 || x == self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE / 2) &&
                    y != self.start_position.y {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z + 1),
                            hull_section.clone(),
                        );

                        self.map.tiles.insert(
                            Vector3i::new(x, y, z + 2),
                            hull_section.clone(),
                        );
                    }
                    else if (x == self.start_position.x + ROOM_SIZE / 2 || x == self.start_position.x + CORRIDOR_LENGTH +  ROOM_SIZE / 2) && (z == self.start_position.z + 2){
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z + 1),
                            hull_section.clone(),
                        );
                    }
                    else {
                        self.map.tiles.insert(
                            Vector3i::new(x, y, z),
                            open_space.clone(),
                        );
                    }         
                } 
            }
        }

        //Windows
        for y in self.start_position.y - ROOM_SIZE / 4 - 1..self.start_position.y - ROOM_SIZE / 4 + 2 {
            let target_position = Vector3i::new(self.start_position.x + ROOM_SIZE / 2, self.start_position.y + y, self.start_position.z + 1);
            self.map.tiles.insert(
                target_position,
                glass_wall.clone(),
            );

            let target_position = Vector3i::new(self.start_position.x + ROOM_SIZE / 2, self.start_position.y + y, self.start_position.z + 1);
            self.map.tiles.insert(
                target_position,
                glass_wall.clone(),

            );

            let target_position = Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE / 2, self.start_position.y + y, self.start_position.z + 1);
            self.map.tiles.insert(
                target_position,
                glass_wall.clone(),

            );

            let target_position = Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE / 2, self.start_position.y + y, self.start_position.z + 1);
            self.map.tiles.insert(
                target_position,
                glass_wall.clone(),

            );
        }

        for y in self.start_position.y + ROOM_SIZE / 2 - ROOM_SIZE / 4 - 2..self.start_position.y + ROOM_SIZE / 2 - ROOM_SIZE / 4 + 1 {
            let target_position = Vector3i::new(self.start_position.x + ROOM_SIZE / 2, self.start_position.y + y, self.start_position.z + 1);
            self.map.tiles.insert(
                target_position,
                glass_wall.clone(),

            );

            let target_position = Vector3i::new(self.start_position.x + ROOM_SIZE / 2, self.start_position.y + y, self.start_position.z + 1);
            self.map.tiles.insert(
                target_position,
                glass_wall.clone(),

            );

            let target_position = Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE / 2, self.start_position.y + y, self.start_position.z + 1);
            self.map.tiles.insert(
                target_position,
                glass_wall.clone(),

            );

            let target_position = Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE / 2, self.start_position.y + y, self.start_position.z + 1);
            self.map.tiles.insert(
                target_position,
                glass_wall.clone(),

            );
        }
        //self.map.tiles.insert(self.start_position, open_space.clone());
        //self.map.tiles.insert(self.start_position + Vector3i::new(ROOM_SIZE + CORRIDOR_LENGTH, 0, 0), open_space);

    }
}

impl MapBuilder for SystemTestMapBuilder {
    fn build_map(&mut self) {
        self.rooms_and_corridor();
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        //Add a light
        spawner::ceiling_lamp(ecs,  Vector3i::new(self.start_position.x, self.start_position.y, self.start_position.z + 2), 1.0, RGB::named(rltk::WHITE).to_rgba(1.0), true);

        //spawner::standing_lamp(ecs, Vector3i::new(self.start_position.x + ROOM_SIZE / 3 + CORRIDOR_LENGTH + ROOM_SIZE, self.start_position.y - ROOM_SIZE / 3, self.start_position.z + 1), 1.0, RGB::named(rltk::RED).to_rgba(1.0), true);

        spawner::ceiling_lamp(ecs, Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE + 3, self.start_position.y, self.start_position.z + 2), 1.0,RGB::named(rltk::WHITE).to_rgba(1.0), false);

        spawner::ceiling_lamp(ecs, Vector3i::new(self.start_position.x + CORRIDOR_LENGTH, self.start_position.y, self.start_position.z + 2), 1.0,RGB::named(rltk::WHITE).to_rgba(1.0), true);

        spawner::door(ecs, Vector3i::new(self.start_position.x + 22 / 2, self.start_position.y, self.start_position.z + 1), false,RGB::named(rltk::ORANGE).to_rgba(1.0), char_to_glyph('/'), char_to_glyph('+'));
        spawner::door(ecs, Vector3i::new(self.start_position.x + 21 +  22 / 2, self.start_position.y, self.start_position.z + 1), true,RGB::named(rltk::SKYBLUE).to_rgba(0.5), char_to_glyph('/'), char_to_glyph('+'));
    
        spawner::power_source(ecs, Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE, self.start_position.y - 5, self.start_position.z + 1), true, 100.0);

        
        spawner::lay_wiring(ecs, self.get_map(), Vector3i::new(self.start_position.x, self.start_position.y, self.start_position.z + 2), Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE, self.start_position.y - 5, self.start_position.z + 1), true);
        spawner::lay_wiring(ecs, self.get_map(), Vector3i::new(self.start_position.x + CORRIDOR_LENGTH, self.start_position.y, self.start_position.z + 2), Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE, self.start_position.y - 5, self.start_position.z + 1), true);
        
        spawner::lay_wiring(ecs, self.get_map(), Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE + 3, self.start_position.y, self.start_position.z + 2), Vector3i::new(self.start_position.x + CORRIDOR_LENGTH + ROOM_SIZE, self.start_position.y - 5, self.start_position.z + 1), true);
    
        //spawner::lay_ducting(ecs, self.get_map(), self.start_position + Vector3i::new(1, 0, -1), self.start_position + Vector3i::new(CORRIDOR_LENGTH + ROOM_SIZE - 1, 0, -1));
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_start_position(&mut self) -> Vector3i {
        self.start_position.clone()
    }
}
