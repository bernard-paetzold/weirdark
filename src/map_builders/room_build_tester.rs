use std::collections::HashSet;

use rltk::RGB;
use specs::World;

use crate::{spawner::{self, lay_wiring}, vectors::Vector3i, Map, Tile};

use super::{common::rand_wall_adj_tile, MapBuilder};

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

    pub fn build_room(&mut self, size: Vector3i) {
        let mut hull = Tile::new_empty();
        if let Some(tile) = crate::tile_blueprints::get_tile("hull") { hull = tile; }

        //let mut glass_hull= Tile::new_empty();
        //if let Some(tile) = crate::tile_blueprints::get_tile("glass_hull") { glass_hull = tile; }

        let mut open_space= Tile::new_empty();
        if let Some(tile) = crate::tile_blueprints::get_tile("open_space") { open_space = tile; }

        let x_lower_limit = self.start_position.x - size.x / 2;
        let x_upper_limit = self.start_position.x + size.x / 2;

        let y_lower_limit = self.start_position.y - size.y / 2;
        let y_upper_limit = self.start_position.y + size.y / 2;

        let z_lower_limit = self.start_position.z - size.z / 2;
        let z_upper_limit = self.start_position.z + size.z / 2;

        //Place floor and roof, with empty space in between
        for x in x_lower_limit..x_upper_limit + 1 {
            for y in y_lower_limit..y_upper_limit + 1 {
                for z in z_lower_limit..z_upper_limit - 1 {
                    let current_position = Vector3i::new(x, y, z);

                    if (z == z_lower_limit || z == z_upper_limit) || (x == x_lower_limit || x == x_upper_limit) || (y == y_lower_limit || y == y_upper_limit) {
                        self.map.tiles.insert(current_position, hull.clone());
                    }
                    else {
                        self.map.tiles.insert(current_position, open_space.clone());
                    }
                }
            }
        }
    }
    pub fn spawn_room_entities(&mut self, ecs: &mut World, size: Vector3i, power_systems: bool, ceiling_light: bool, power_source: bool) {
        let x_lower_limit = self.start_position.x - size.x / 2 + 1;
        //let x_upper_limit = self.start_position.x + size.x / 2 - 1;

        let y_lower_limit = self.start_position.y - size.y / 2 + 1;
        //let y_upper_limit = self.start_position.y + size.y / 2 - 1;

        //let z_lower_limit = self.start_position.z - size.z / 2 + 1;
        let z_upper_limit = self.start_position.z + size.z / 2 - 2;

        let mut occupied_tiles = HashSet::new();
        
        //Add power infrastructure
        if power_systems {
            let mut device_positions = Vec::new();

            let breaker_position = rand_wall_adj_tile(self.start_position, size);
            spawner::breaker_box(ecs, breaker_position);

            occupied_tiles.insert(breaker_position);

            if ceiling_light {
                let ceiling_light_position = Vector3i::new(x_lower_limit , y_lower_limit, z_upper_limit);
                spawner::ceiling_lamp(ecs, ceiling_light_position, 1.0, RGB::named(rltk::WHITE).to_rgba(1.0), true);
                device_positions.push(ceiling_light_position)
            }

            if power_source {
                let mut power_source_position;

                loop  {
                    power_source_position = rand_wall_adj_tile(self.start_position, size);
                    if !occupied_tiles.contains(&power_source_position) { break; }
                } 
                spawner::power_source(ecs, power_source_position, true, 100.0);
                device_positions.push(power_source_position);
            }

            while let  Some(position) = device_positions.pop() {
                lay_wiring(ecs, self.get_map(), position, breaker_position, true);
            }
        }
    }
}

impl MapBuilder for RoomTestMapBuilder {
    fn build_map(&mut self) {
        self.build_room(Vector3i::new(15, 20, 4));
    }

    fn spawn_entities(&mut self, ecs: &mut World) {
        self.spawn_room_entities(ecs, Vector3i::new(15, 20, 4), true, true, true);
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_start_position(&mut self) -> Vector3i {
        self.start_position
    }
}