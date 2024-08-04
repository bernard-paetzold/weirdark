use crate::{vectors::Vector3i, Tile};
use bimap::BiMap;
use serde::{Deserialize, Serialize};
use specs::Entity;
use std::collections::HashMap;

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