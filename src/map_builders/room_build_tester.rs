use std::collections::HashSet;

use rltk::{a_star_search, RGB};
use specs::World;

use crate::{graphics::char_to_glyph, pathfinding::find_path_with_width, spawner::{self, door, lay_wiring}, vectors::{utils::get_cardinal_neighbours, Vector3i}, Map, Tile};

use super::{common::{rand_wall_adj_tile, Room}, MapBuilder};

pub struct RoomTestMapBuilder {
    map: Map,
    start_position: Vector3i,
}

impl RoomTestMapBuilder {
    pub fn new(start_position: Vector3i) -> Self {
        Self {
            map: Map::new(),
            start_position,
        }
    }

    pub fn build_room(&mut self, room: &Room) {
        let size = room.size;
        let position = room.centre; 
        let door_sides = &room.door_sides;


        let mut hull = Tile::new_empty_stp();
        if let Some(tile) = crate::tile_blueprints::get_tile("hull") { hull = tile; }

        //let mut glass_hull= Tile::new_empty();
        //if let Some(tile) = crate::tile_blueprints::get_tile("glass_hull") { glass_hull = tile; }

        let mut open_space= Tile::new_empty_stp();
        if let Some(tile) = crate::tile_blueprints::get_tile("vacuume") { open_space = tile; }

        let mut breathable_atmosphere = Tile::new_empty_stp();
        if let Some(tile) = crate::tile_blueprints::get_tile("breathable_atmosphere") { breathable_atmosphere = tile; }

        let x_lower_limit = position.x - size.x / 2;
        let x_upper_limit = position.x + size.x / 2;

        let y_lower_limit = position.y - size.y / 2;
        let y_upper_limit = position.y + size.y / 2;

        let z_lower_limit = position.z - size.z / 2;
        let z_upper_limit = position.z + size.z / 2;

        let vacuume_gap = 5;

        //Place floor and roof, with empty space in between
        for x in x_lower_limit..x_upper_limit + 1 {
            for y in y_lower_limit..y_upper_limit + 1 {
                for z in z_lower_limit..z_upper_limit - 1 {
                    let current_position = Vector3i::new(x, y, z);

                    if (z == z_lower_limit || z == z_upper_limit) || (x == x_lower_limit || x == x_upper_limit) || (y == y_lower_limit || y == y_upper_limit) {
                        self.map.tiles.insert(current_position, hull.clone());
                    }
                    else {
                        self.map.tiles.insert(current_position, breathable_atmosphere.clone());
                    }
                }
            }
        }

        for side in door_sides {
            self.map.tiles.insert(Vector3i::new(position.x + side.x * ((size.x / 2)), position.y + side.y * ((size.y / 2)), position.z - 1), breathable_atmosphere.clone());
        }
      
    }

    pub fn build_corridor(&mut self, start_position: Vector3i, target: Vector3i, width: usize) {
        let hull = crate::tile_blueprints::get_tile("hull").unwrap_or_else(|| Tile::new_empty_stp());
        let breathable_atmosphere = crate::tile_blueprints::get_tile("breathable_atmosphere").unwrap_or_else(|| Tile::new_empty_stp());
    
        let mut path = find_path_with_width(self.map.clone(), start_position, target, width);
        let half_width = (width / 2) as i32;

        let mut prev_position = start_position;
    
        let mut count = 0;
        let mut prev_direction = *path.get(1).unwrap_or(&Vector3i::new_equi(0)) - *path.get(0).unwrap_or(&Vector3i::new_equi(0));

        let mut corridor_tiles = HashSet::new();
        
        for position in path.iter() {
            let mut direction = *position - prev_position;
            let mut next_direction= *position - prev_position;

            let next_position = path.get(count + 1);

            if let Some(next_position) = next_position {
                next_direction = *next_position - *position;
    
                if prev_position == *position {
                    direction = next_direction;
                }
            }

            let half_width = (width / 2) as i32;

            for x in -half_width..=half_width {
                for y in -half_width..=half_width {
                    if let None = self.map.tiles.get(&(*position + Vector3i::new(x, y, 0))) {
                        self.map.tiles.insert(*position + Vector3i::DOWN + Vector3i::new(x, y, 0), hull.clone());
                        self.map.tiles.insert(*position + Vector3i::new(x, y, 0), breathable_atmosphere.clone());
                        self.map.tiles.insert(*position + Vector3i::UP + Vector3i::new(x, y, 0), breathable_atmosphere.clone());
                        self.map.tiles.insert(*position + Vector3i::UP * 2 + Vector3i::new(x, y, 0), hull.clone());
                        corridor_tiles.insert(*position + Vector3i::new(x, y, 0));
                    }   
                }
            }

            prev_position = *position;
            prev_direction = direction;
            count += 1;
        }

        for position in corridor_tiles.iter() {
            let mut neighbours = 0;

            for neighbour in get_cardinal_neighbours(*position).iter() {
                if corridor_tiles.contains(neighbour) {
                    neighbours += 1;
                }
            }

            if neighbours < 4 {
                self.map.tiles.insert(*position, hull.clone());
                self.map.tiles.insert(*position + Vector3i::UP, hull.clone());
            }
        }
    
        
    
        // Ensure start and end points are open
        for i in -half_width..=half_width {
            for position in [start_position, target] {
                for z in 0..=1 {
                    //self.map.tiles.insert(position + Vector3i::UP * z + Vector3i::W * (half_width * i), breathable_atmosphere.clone());
                    //self.map.tiles.insert(position + Vector3i::UP * z + Vector3i::E * (half_width * i), breathable_atmosphere.clone());
                    //self.map.tiles.insert(position + Vector3i::UP * z, breathable_atmosphere.clone());
                }
            }
        }
    }

    pub fn connect_rooms(&mut self, room_one: &Room, room_two: &Room, width: usize) {
        let hull = crate::tile_blueprints::get_tile("hull").unwrap_or_else(|| Tile::new_empty_stp());
        let breathable_atmosphere = crate::tile_blueprints::get_tile("breathable_atmosphere").unwrap_or_else(|| Tile::new_empty_stp());
    
        //Check doors to decide what doors to connect
        let room_direction = (room_two.centre - room_one.centre).normalize_delta();
        let room_one_door;
        let room_two_door;

        let half_width = (width / 2) as i32;

        room_one_door = pick_best_side(&room_one, room_direction);
        room_two_door = pick_best_side(&room_two, room_direction * -1);

        //Start at the position of the first door
        let door_one_position = Vector3i::new(room_one.centre.x + room_one_door.x * ((room_one.size.x / 2)), room_one.centre.y + room_one_door.y * ((room_one.size.y / 2)), room_one.centre.z - 1);
        let start_position = door_one_position + (room_direction * half_width);

        let door_two_position = Vector3i::new(room_two.centre.x + room_two_door.x * ((room_two.size.x / 2)), room_two.centre.y + room_two_door.y * ((room_two.size.y / 2)), room_two.centre.z - 1);
        let target = door_two_position + (room_direction * half_width);

        let mut path = find_path_with_width(self.map.clone(), start_position, target, width);


        let mut prev_position = start_position;
    
        let mut count = 0;
        let mut prev_direction = *path.get(1).unwrap_or(&Vector3i::new_equi(0)) - *path.get(0).unwrap_or(&Vector3i::new_equi(0));

        let mut corridor_tiles = HashSet::new();
        
        for position in path.iter() {
            let mut direction = *position - prev_position;
            let mut next_direction= *position - prev_position;

            let next_position = path.get(count + 1);

            if let Some(next_position) = next_position {
                next_direction = *next_position - *position;
    
                if prev_position == *position {
                    direction = next_direction;
                }
            }

            for x in -half_width..=half_width {
                for y in -half_width..=half_width {
                    if let None = self.map.tiles.get(&(*position + Vector3i::new(x, y, 0))) {
                        self.map.tiles.insert(*position + Vector3i::DOWN + Vector3i::new(x, y, 0), hull.clone());
                        self.map.tiles.insert(*position + Vector3i::new(x, y, 0), breathable_atmosphere.clone());
                        self.map.tiles.insert(*position + Vector3i::UP + Vector3i::new(x, y, 0), breathable_atmosphere.clone());
                        self.map.tiles.insert(*position + Vector3i::UP * 2 + Vector3i::new(x, y, 0), hull.clone());
                        corridor_tiles.insert(*position + Vector3i::new(x, y, 0));
                    }
                    else if let Some(tile)  = self.map.tiles.get(&(*position + Vector3i::new(x, y, 0))) {
                        if !tile.passable {
                            corridor_tiles.insert(*position + Vector3i::new(x, y, 0));
                        }
                    }   
                }
            }

            prev_position = *position;
            prev_direction = direction;
            count += 1;
        }

        for position in corridor_tiles.iter() {
            let mut neighbours = 0;

            for neighbour in get_cardinal_neighbours(*position).iter() {
                if corridor_tiles.contains(neighbour) {
                    neighbours += 1;
                }
            }

            if neighbours < 4 {
                self.map.tiles.insert(*position, hull.clone());
                self.map.tiles.insert(*position + Vector3i::UP, hull.clone());
            }
        }
    
        
    
        // Ensure start and end points are open
        let mut current_position = door_one_position;
        let direction = (start_position - current_position).normalize_delta();

        while current_position != start_position {
            self.map.tiles.insert(current_position, breathable_atmosphere.clone());
            self.map.tiles.insert(current_position , breathable_atmosphere.clone());
            current_position += direction;
        }

        let mut current_position = door_two_position;
        let direction = (target - current_position).normalize_delta();

        while current_position != target {
            self.map.tiles.insert(current_position, breathable_atmosphere.clone());
            self.map.tiles.insert(current_position , breathable_atmosphere.clone());
            current_position += direction;
        }
    }

    pub fn spawn_room_entities(&mut self, ecs: &mut World, room: Room, power_systems: bool, ceiling_lights: bool, power_source: bool, heater: bool) {
        let position = room.centre;
        let size = room.size;
        let doors = room.door_sides;
        
        let x_lower_limit = position.x - size.x / 2 + 1;
        let x_upper_limit = position.x + size.x / 2 - 1;

        let y_lower_limit = position.y - size.y / 2 + 1;
        let y_upper_limit = position.y + size.y / 2 - 1;

        //let z_lower_limit = position.z - size.z / 2 + 1;
        let z_upper_limit = position.z + size.z / 2 - 2;

        let mut occupied_tiles = HashSet::new();
        
        //Add power infrastructure
        if power_systems {
            let mut device_positions = Vec::new();

            let breaker_position = rand_wall_adj_tile(position, size);
            spawner::breaker_box(ecs, breaker_position);

            occupied_tiles.insert(breaker_position);

            if ceiling_lights {
                let ceiling_light_position = Vector3i::new(position.x, position.y, z_upper_limit);
                spawner::ceiling_lamp(ecs, ceiling_light_position, 1.0, RGB::named(rltk::WHITE).to_rgba(1.0), true);
                device_positions.push(ceiling_light_position);
            }

            if power_source {
                let mut power_source_position;

                loop  {
                    power_source_position = rand_wall_adj_tile(position, size);
                    if !occupied_tiles.contains(&power_source_position) { break; }
                } 
                spawner::power_source(ecs, power_source_position, true, 1100.0);
                lay_wiring(ecs, self.get_map(), power_source_position, breaker_position, RGB::named(rltk::BLUE).to_rgba(1.0), "blue".to_string(), true);
            }

            if heater {
                let mut heater_position;

                loop  {
                    heater_position = rand_wall_adj_tile(position, size);
                    if !device_positions.contains(&heater_position) { break; }
                } 
                //spawner::heater(ecs, heater_position, 300.0, true);
                //device_positions.push(heater_position);
            }

            while let  Some(position) = device_positions.pop() {
                lay_wiring(ecs, self.get_map(), position, breaker_position, RGB::named(rltk::RED).to_rgba(1.0), "red".to_string(), true);
            }
        }

        for side in doors {
            door(ecs, Vector3i::new(position.x + side.x * ((size.x / 2)), position.y + side.y * ((size.y / 2)), position.z - 1), false, RGB::named(rltk::GRAY).to_rgba(1.0), char_to_glyph('/'), char_to_glyph('+'));
        }
    }
}

impl MapBuilder for RoomTestMapBuilder {
    fn build_map(&mut self) {
        //let mut rooms = Vec::new();

        let room_one = Room::new(self.start_position, Vector3i::new(16, 16, 4), vec![Vector3i::E]);
        let room_two = Room::new(self.start_position + Vector3i::new(40, 0, 0), Vector3i::new(16, 16, 4), vec![Vector3i::E]);

        self.build_room(&room_one);
        self.build_room(&room_two);

        self.connect_rooms(&room_one, &room_two, 5);
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        let room_one = Room::new(self.start_position, Vector3i::new(16, 16, 4), vec![Vector3i::E]);
        let room_two = Room::new(self.start_position + Vector3i::new(40, 0, 0), Vector3i::new(16, 16, 4), vec![Vector3i::E]);
        
        self.spawn_room_entities(ecs, room_one, true, true, true, true);
        self.spawn_room_entities(ecs, room_two, true, true, true, true);
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_start_position(&mut self) -> Vector3i {
        self.start_position
    }
}

fn pick_best_side(room: &Room, direction: Vector3i) -> Vector3i {
    let side;

    if direction == Vector3i::N {
        if room.door_sides.contains(&Vector3i::N) {
            side = Vector3i::N;
        }
        else if room.door_sides.contains(&Vector3i::W) {
            side = Vector3i::W;
        }
        else if room.door_sides.contains(&Vector3i::E) {
            side = Vector3i::E;
        }
        else {
            side = Vector3i::S;
        }
    }
    else if direction == Vector3i::E {
        if room.door_sides.contains(&Vector3i::E) {
            side = Vector3i::E;
        }
        else if room.door_sides.contains(&Vector3i::N) {
            side = Vector3i::N;
        }
        else if room.door_sides.contains(&Vector3i::S) {
            side = Vector3i::S;
        }
        else {
            side = Vector3i::W;
        }
    }
    else if direction == Vector3i::S {
        if room.door_sides.contains(&Vector3i::S) {
            side = Vector3i::S;
        }
        else if room.door_sides.contains(&Vector3i::E) {
            side = Vector3i::E;
        }
        else if room.door_sides.contains(&Vector3i::W) {
            side = Vector3i::W;
        }
        else {
            side = Vector3i::N;
        }
    }
    else {
        if room.door_sides.contains(&Vector3i::W) {
            side = Vector3i::W;
        }
        else if room.door_sides.contains(&Vector3i::N) {
            side = Vector3i::N;
        }
        else if room.door_sides.contains(&Vector3i::S) {
            side = Vector3i::S;
        }
        else {
            side = Vector3i::E;
        }
    }

    side
}