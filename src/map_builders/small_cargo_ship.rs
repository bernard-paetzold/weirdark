use std::{
    collections::{BTreeMap, HashMap, HashSet},
    i32::MAX,
};

use rltk::RGB;
use specs::World;

use crate::{
    graphics::char_to_glyph,
    pathfinding::find_path_with_width,
    rng::{self, range},
    spawner::{self, lay_wiring},
    vectors::{utils::get_cardinal_neighbours, Vector3i},
    Map, Tile,
};

use super::{
    common::{Area, AreaType, Corridor, Room},
    MapBuilder,
};

pub struct SmallCargoShipMapBuilder {
    map: Map,
    start_position: Vector3i,
    areas: Vec<Box<dyn Area>>,
}

impl SmallCargoShipMapBuilder {
    pub fn new(start_position: Vector3i) -> Self {
        Self {
            map: Map::new(),
            start_position,
            areas: Vec::new(),
        }
    }

    pub fn build_corridor(&mut self, corridor: &mut Corridor) {
        let hull =
            crate::tile_blueprints::get_tile("hull").unwrap_or_else(|| Tile::new_empty_stp());
        let breathable_atmosphere = crate::tile_blueprints::get_tile("breathable_atmosphere")
            .unwrap_or_else(|| Tile::new_empty_stp());

        let path = find_path_with_width(
            self.map.clone(),
            corridor.start,
            corridor.end,
            corridor.width,
        );

        let mut corridor_tiles = HashSet::new();

        let half_width = (corridor.width / 2) as i32;

        for position in path.iter() {
            for x in -half_width..=half_width {
                for y in -half_width..=half_width {
                    if let None = self.map.tiles.get(&(*position + Vector3i::new(x, y, 0))) {
                        self.map.tiles.insert(
                            *position + Vector3i::DOWN + Vector3i::new(x, y, 0),
                            hull.clone(),
                        );
                        self.map.tiles.insert(
                            *position + Vector3i::new(x, y, 0),
                            breathable_atmosphere.clone(),
                        );
                        self.map.tiles.insert(
                            *position + Vector3i::UP + Vector3i::new(x, y, 0),
                            breathable_atmosphere.clone(),
                        );
                        self.map.tiles.insert(
                            *position + Vector3i::UP * 2 + Vector3i::new(x, y, 0),
                            hull.clone(),
                        );

                        //Add empty tile for ducts in the roof.
                        self.map.tiles.insert(
                            *position + Vector3i::UP * 3 + Vector3i::new(x, y, 0),
                            Tile::new_vacuume(),
                        );

                        corridor_tiles.insert(*position + Vector3i::new(x, y, 0));
                    } else if let Some(tile) =
                        self.map.tiles.get(&(*position + Vector3i::new(x, y, 0)))
                    {
                        if !tile.passable {
                            corridor_tiles.insert(*position + Vector3i::new(x, y, 0));
                        }
                    }
                }
            }
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
                self.map
                    .tiles
                    .insert(*position + Vector3i::UP, hull.clone());

                corridor.nodes.push(*position);
            }
        }
    }

    pub fn build_room(&mut self, room: &mut Room) -> bool {
        let hull =
            crate::tile_blueprints::get_tile("hull").unwrap_or_else(|| Tile::new_empty_stp());

        let breathable_atmosphere = crate::tile_blueprints::get_tile("breathable_atmosphere")
            .unwrap_or_else(|| Tile::new_empty_stp());

        let vacuume =
            crate::tile_blueprints::get_tile("vacuume").unwrap_or_else(|| Tile::new_vacuume());

        //Pick side to expand into
        let neighbours = get_cardinal_neighbours(room.centre);

        let mut open_tile = room.centre;
        let corridor_connection = room.centre;

        let half_size_x = (room.size.x / 2) as i32;
        let half_size_y = (room.size.y / 2) as i32;
        let half_size_z = (room.size.z / 2) as i32;

        for neighbour in neighbours.iter() {
            let mut valid_tile = true;

            for z in -half_size_z..=half_size_z {
                if let Some(_) = self.map.tiles.get(&(*neighbour + Vector3i::new(0, 0, z))) {
                    valid_tile = false;
                    break;
                }
            }

            if valid_tile {
                open_tile = *neighbour;
            }
        }

        if open_tile == room.centre {
            return false;
        }

        //Add open tile to the rooms doors
        room.nodes.push(open_tile);

        let direction = (open_tile - room.centre).normalize_delta();
        let room_centre =
            room.centre + Vector3i::new(half_size_x + 1, half_size_y + 1, 0) * direction;

        //Expand out in the direction of the open tile
        let mut area_tiles = Vec::new();

        //Check if space is open
        for x in -half_size_x..=half_size_x {
            for y in -half_size_y..=half_size_y {
                for z in -half_size_z..=half_size_z {
                    let current_position = room_centre + Vector3i::new(x, y, z);

                    if let Some(_) = self.map.tiles.get(&current_position) {
                        return false;
                    }
                }
                area_tiles.push(room_centre + Vector3i::new(x, y, 0));
            }
        }

        //Add tiles
        for tile_position in area_tiles.iter() {
            self.map
                .tiles
                .insert(*tile_position + Vector3i::DOWN, hull.clone());
            self.map
                .tiles
                .insert(*tile_position, breathable_atmosphere.clone());
            self.map
                .tiles
                .insert(*tile_position + Vector3i::UP, breathable_atmosphere.clone());
            self.map
                .tiles
                .insert(*tile_position + Vector3i::UP * 2, hull.clone());

            //Add empty tile for ducts in the roof.
            self.map
                .tiles
                .insert(*tile_position + Vector3i::UP * 3, vacuume.clone());

            //If the tile is at the edge of the room, make it a wall
            let mut neighbours = 0;

            for neighbour in get_cardinal_neighbours(*tile_position).iter() {
                if area_tiles.contains(neighbour) {
                    neighbours += 1;
                }
            }

            if neighbours < 4 {
                self.map.tiles.insert(*tile_position, hull.clone());
                self.map
                    .tiles
                    .insert(*tile_position + Vector3i::UP, hull.clone());
            }
        }
        //Clear door
        self.map
            .tiles
            .insert(corridor_connection, breathable_atmosphere.clone());

        for tile_position in room.nodes.iter() {
            self.map
                .tiles
                .insert(*tile_position, breathable_atmosphere.clone());
        }

        room.centre = room_centre;
        true
    }

    pub fn populate_area(ecs: &mut World, area: &mut Box<dyn Area>) {
        let nodes = area.get_nodes().clone();
        let mut connections = Vec::new();
        let mut entity_positions = HashSet::new();

        if nodes.len() == 0 {
            return;
        }

        let adjacent_to = rng::range(0, nodes.len() as i32) as usize;
        let offset;

        let left_right = if rng::range(0, 2) == 0 { -1 } else { 1 };

        let side = nodes
            .get(adjacent_to)
            .unwrap_or(&Vector3i::new_equi(0).clone())
            .clone();

        let direction = (*area.get_area_position() - side).normalize_delta();

        if direction == Vector3i::N {
            offset = Vector3i::new(left_right, 1, 0);
        } else if direction == Vector3i::E {
            offset = Vector3i::new(-1, left_right, 0);
        } else if direction == Vector3i::S {
            offset = Vector3i::new(left_right, -1, 0);
        } else {
            offset = Vector3i::new(1, left_right, 0);
        }

        if area.get_area_type() != AreaType::Corridor {
            let breaker_position = side + offset + (direction * 2);
            spawner::breaker_box(ecs, breaker_position);
            area.set_breaker_pos(breaker_position);
            entity_positions.insert(breaker_position);
        }
        let ceiling_lamp_position = *area.get_area_position() + Vector3i::UP;
        spawner::ceiling_lamp(
            ecs,
            ceiling_lamp_position,
            1.0,
            if area.get_area_type() == AreaType::GeneratorRoom {
                RGB::named(rltk::RED).to_rgba(1.0)
            } else {
                RGB::named(rltk::WHITE).to_rgba(1.0)
            },
            true,
        );
        connections.push(ceiling_lamp_position);
        entity_positions.insert(ceiling_lamp_position);

        if area.get_area_type() == AreaType::GeneratorRoom {
            let generator_position = area.get_area_position();

            spawner::power_source(ecs, *generator_position, true, 1000.0);
            connections.push(*generator_position);
        }

        if area.get_area_type() != AreaType::Corridor {
            // Doors
            for node in area.get_nodes().iter() {
                spawner::door(
                    ecs,
                    *node,
                    false,
                    RGB::named(rltk::GRAY).to_rgba(1.0),
                    char_to_glyph('/'),
                    char_to_glyph('+'),
                );
            }

            // Add storage cabinets
            let cabinets = range(1, 5);

            for _ in 0..cabinets {
                let mut placed = false;
                let mut attempts = 0;

                while !placed && attempts < 10 {
                    if let Some(cabinet_position) = get_wall_adjacent_position(area.as_ref()) {
                        if !entity_positions.contains(&cabinet_position)
                            && nodes
                                .iter()
                                .all(|node| node.distance_to(cabinet_position) > 1.0)
                        {
                            placed = true;
                            let cabinet = spawner::storage_cabinet(ecs, cabinet_position);
                            let item = spawner::test_item(ecs, Vector3i::new_equi(0));

                            spawner::put_item_in_container(ecs, item, cabinet);
                            entity_positions.insert(cabinet_position);
                        }
                    }
                    attempts += 1;
                }
            }
        }

        //Add devices to list of places to wire
        area.update_power_connections().append(&mut connections);
    }
}

const MIN_AREA_SIZE: i32 = 5;

impl MapBuilder for SmallCargoShipMapBuilder {
    fn build_map(&mut self) {
        let mut back_bone = Corridor::new(
            self.start_position,
            self.start_position + Vector3i::new(40, 0, 0),
            7,
            "Main corridor".to_string(),
            AreaType::Corridor,
            true,
        );
        self.build_corridor(&mut back_bone);

        let mut open_nodes: Vec<Vector3i> = Vec::new();

        for node in back_bone.nodes.iter() {
            open_nodes.push(node.clone());
        }

        self.areas.push(Box::new(back_bone));

        let sterm_room_position = self.start_position + Vector3i::new(40 + 3, 0, 0);
        let mut stern_room = Room::new(
            sterm_room_position,
            Vector3i::new(7, 9, 4),
            "cockpit".to_string(),
            AreaType::Cockpit,
            true,
        );

        self.build_room(&mut stern_room);

        self.areas.push(Box::new(stern_room.clone()));

        let aft_room_position = self.start_position + Vector3i::new(-3, 0, 0);

        let mut aft_room = Room::new(
            aft_room_position,
            Vector3i::new(7, 9, 4),
            "engineering".to_string(),
            AreaType::GeneratorRoom,
            true,
        );

        self.build_room(&mut aft_room);

        self.areas.push(Box::new(aft_room.clone()));

        open_nodes.sort();
        let mut shuffled_nodes: BTreeMap<i32, Vector3i> = BTreeMap::new();

        for node in open_nodes.into_iter() {
            let random_index = rng::random_int();

            shuffled_nodes.insert(random_index, node);
        }

        for (_, node) in shuffled_nodes.iter() {
            let mut dimension = rng::range(MIN_AREA_SIZE, 10) | 1;

            while dimension >= MIN_AREA_SIZE {
                let area_size = Vector3i::new(dimension, dimension, 4);
                let mut room = Room::new(
                    *node,
                    area_size,
                    "generic".to_string(),
                    AreaType::GenericRoom,
                    true,
                );
                let mut mirrored_room = Room::new(
                    *node * Vector3i::new(1, -1, 1),
                    area_size,
                    "generic".to_string(),
                    AreaType::GenericRoom,
                    true,
                );
                if self.build_room(&mut room) && self.build_room(&mut mirrored_room) {
                    self.areas.push(Box::new(room));
                    self.areas.push(Box::new(mirrored_room));
                    break;
                } else {
                    dimension -= 1;
                }
            }
        }
    }

    fn spawn_entities(&mut self, ecs: &mut specs::World) {
        let mut breaker_positions = HashSet::new();
        let mut area_positions = Vec::new();
        let mut device_positions = Vec::new();

        //Pick a random room to be the power room
        let mut generator_breaker = Vector3i::new_equi(0);
        let mut generator_room_position = Vector3i::new_equi(0);

        for area in self.get_areas().iter_mut() {
            SmallCargoShipMapBuilder::populate_area(ecs, area);
        }

        for area in self.get_areas().iter_mut() {
            if let Some(breaker_position) = area.get_breaker_pos() {
                breaker_positions.insert(breaker_position.clone());

                if area.get_area_type() == AreaType::GenericRoom {
                    area_positions.push((
                        area.get_area_position().clone() + Vector3i::UP * 3,
                        (area.get_area_position().clone() + Vector3i::UP * 3)
                            * Vector3i::new(1, 0, 1),
                    ));
                } else if area.get_area_type() == AreaType::Cockpit {
                    area_positions.push((
                        area.get_area_position().clone() + Vector3i::UP * 3,
                        Vector3i::new_equi(MAX),
                    ));
                }

                if area.get_area_type() == AreaType::GeneratorRoom {
                    generator_breaker = breaker_position.clone();
                    generator_room_position = *area.get_area_position();
                }

                for device in area.get_power_connections().iter() {
                    device_positions.push((breaker_position.clone(), device.clone()));
                }
            }
        }

        //Add opening to roof of generator room
        self.map
            .tiles
            .remove(&(generator_room_position + Vector3i::UP * 2));

        for area_position in area_positions.iter_mut() {
            if area_position.1.x == MAX {
                area_position.1 = generator_room_position.clone() + Vector3i::UP * 3;
            }
            //Duct
            spawner::lay_ducting(
                ecs,
                self.get_map(),
                area_position.0,
                area_position.1 + (Vector3i::S * area_position.0.normalize_delta()),
            );

            self.map
                .tiles
                .insert(area_position.0 + Vector3i::DOWN, Tile::new_empty_stp());
        }

        for position in breaker_positions.iter() {
            let color_hex = format!(
                "#{:X}{:X}{:X}",
                rng::range(0, 256),
                rng::range(0, 256),
                rng::range(0, 256),
            );

            let color = RGB::from_hex(color_hex.clone()).unwrap_or(RGB::named(rltk::RED));
            lay_wiring(
                ecs,
                self.get_map(),
                generator_breaker.clone(),
                *position,
                &breaker_positions,
                RGB::named(rltk::RED).to_rgba(1.0),
                "RED".to_string(),
                true,
                false,
            );

            for (breaker_position, device_position) in device_positions
                .iter()
                .filter(|(breaker, _)| breaker == position)
            {
                lay_wiring(
                    ecs,
                    self.get_map(),
                    *device_position,
                    *breaker_position,
                    &breaker_positions,
                    color.to_rgba(1.0),
                    color_hex.clone(),
                    true,
                    true,
                );
            }
        }
    }

    fn get_map(&mut self) -> Map {
        self.map.clone()
    }

    fn get_start_position(&mut self) -> crate::vectors::Vector3i {
        self.start_position
    }

    fn get_areas(&mut self) -> &mut Vec<Box<dyn Area>> {
        &mut self.areas
    }
}

fn get_wall_adjacent_position(area: &dyn Area) -> Option<Vector3i> {
    let area_pos = area.get_area_position();
    let size_x = area.get_size().x as i32;
    let size_y = area.get_size().y as i32;

    // Ensure the room is big enough for cabinet placement
    if size_x < 3 || size_y < 3 {
        return None;
    }

    let half_x = size_x / 2;
    let half_y = size_y / 2;

    let (x, y) = if rng::range(0, 2) == 0 {
        // Place along X-axis walls
        (
            if rng::range(0, 2) == 0 {
                -half_x + 1
            } else {
                half_x - 1
            },
            rng::range(-half_y + 2, half_y - 1),
        )
    } else {
        // Place along Y-axis walls
        (
            rng::range(-half_x + 2, half_x - 1),
            if rng::range(0, 2) == 0 {
                -half_y + 1
            } else {
                half_y - 1
            },
        )
    };

    Some(*area_pos + Vector3i::new(x, y, 0))
}
