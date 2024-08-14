use crate::{vectors::Vector3i, Tile, MAP_SCREEN_HEIGHT, MAP_SCREEN_WIDTH};
use bimap::BiMap;
use serde::{Deserialize, Serialize};
use specs::Entity;
use std::collections::HashMap;

pub mod pathfinding;
pub mod tile_blueprints;

#[serde_with::serde_as]
#[derive(Default, Clone, Serialize, Deserialize)]
pub struct Map {
    #[serde_as(as = "HashMap<serde_with::json::JsonString, _>")]
    pub tiles: HashMap<Vector3i, Tile>,

    #[serde(skip)]
    pub entities: BiMap<Vector3i, Entity>,
}

impl Map {
    pub fn new() -> Map {
        Map {
            tiles: HashMap::new(),
            entities: BiMap::new(),
        }
    }

    pub fn clear_map_entities(&mut self) {
        self.entities.clear();
    }
}

pub mod components;

pub fn mouse_to_map(mouse_position: (i32, i32), viewport_position: Vector3i) -> Vector3i {
    Vector3i::new(mouse_position.0  - (MAP_SCREEN_WIDTH / 2) + viewport_position.x, mouse_position.1  - (MAP_SCREEN_HEIGHT / 2) + viewport_position.y, viewport_position.z)
}