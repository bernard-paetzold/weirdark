use empty_plain::EmptyPlainMapBuilder;
use system_test_map::SystemTestMapBuilder;

use crate::vectors::Vector3i;

use super::Map;

mod empty_plain;
mod system_test_map;

trait MapBuilder {
    fn build(map_size: Vector3i) -> Map;
}

pub fn build_emply_plain_map(map_size: Vector3i) -> Map {
    EmptyPlainMapBuilder::build(map_size)
}

pub fn build_system_test_map(map_size: Vector3i) -> Map {
    SystemTestMapBuilder::build(map_size)
}