use specs::World;
use system_test_map::SystemTestMapBuilder;

use crate::vectors::Vector3i;

use super::Map;

mod system_test_map;

#[allow(dead_code)]
pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&mut self) -> Map;
    fn get_start_position(&mut self) -> Vector3i;
}

/*pub fn build_emply_plain_map(map_size: Vector3i) -> Map {
    EmptyPlainMapBuilder::build(map_size)
}*/

pub fn build_system_test_map(_map_size: Vector3i, start_position: Vector3i) -> Box<dyn MapBuilder> {
    Box::new(SystemTestMapBuilder::new(start_position))
}