use common::Area;
use std::collections::HashMap;
//use room_build_tester::RoomTestMapBuilder;
use small_cargo_ship::SmallCargoShipMapBuilder;
use specs::World;

use crate::vectors::Vector3i;

use super::Map;

mod common;
//mod room_build_tester;
mod small_cargo_ship;

#[allow(dead_code)]
pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&mut self) -> Map;
    fn get_start_position(&mut self) -> Vector3i;
    fn get_areas(&mut self) -> &mut HashMap<Vector3i, Box<dyn Area>>;
}

/*pub fn build_emply_plain_map(map_size: Vector3i) -> Map {
    EmptyPlainMapBuilder::build(map_size)
}*/

pub fn build_small_cargo_ship_map(
    _map_size: Vector3i,
    start_position: Vector3i,
) -> Box<dyn MapBuilder> {
    Box::new(SmallCargoShipMapBuilder::new(start_position))
}

/*pub fn build_room_test_map(_map_size: Vector3i, start_position: Vector3i) -> Box<dyn MapBuilder> {
    Box::new(RoomTestMapBuilder::new(start_position))
}*/
