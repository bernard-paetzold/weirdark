use std::collections::HashSet;

use crate::map;
use crate::vectors::Vector3i;
use rltk::{RGB, RGBA};
use serde::Deserialize;
use serde::Serialize;
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

use super::intents::Interactable;

#[derive(Default, Debug, Serialize, PartialEq, Deserialize, Clone)]
pub enum Direction {
    #[default]
    N,
    NW,
    W,
    SW,
    S,
    SE,
    E,
    NE,
    UP,
    DOWN,
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Viewshed {
    pub visible_tiles: HashSet<Vector3i>,
    pub discovered_tiles: HashSet<Vector3i>,
    pub z_range: usize,
    pub view_distance: usize,
    pub dark_vision: f32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(view_distance: usize, z_range: usize, dark_vision: f32) -> Viewshed {
        Viewshed {
            visible_tiles: HashSet::new(),
            discovered_tiles: HashSet::new(),
            z_range,
            view_distance,
            dark_vision,
            dirty: true,
        }
    }
}

#[allow(dead_code)]
#[derive(Component, Clone, Debug)]
pub struct Blocker {
    pub sides: Vec<Direction>,
}

#[allow(dead_code)]
impl Blocker {
    pub fn new(sides: Vec<Direction>) -> Blocker {
        Blocker { sides }
    }
    pub fn new_cardinal_sides() -> Blocker {
        Blocker {
            sides: vec![Direction::N, Direction::E, Direction::S, Direction::W],
        }
    }
    pub fn new_n_s() -> Blocker {
        Blocker {
            sides: vec![
                Direction::N,
                Direction::NW,
                Direction::W,
                Direction::SW,
                Direction::S,
                Direction::SE,
                Direction::E,
                Direction::UP,
                Direction::DOWN,
            ],
        }
    }
    pub fn new_e_w() -> Blocker {
        Blocker {
            sides: vec![
                Direction::N,
                Direction::NW,
                Direction::W,
                Direction::SW,
                Direction::S,
                Direction::SE,
                Direction::E,
                Direction::UP,
                Direction::DOWN,
            ],
        }
    }
    pub fn new_all_sides() -> Blocker {
        Blocker {
            sides: vec![
                Direction::N,
                Direction::W,
                Direction::S,
                Direction::E,
                Direction::UP,
                Direction::DOWN,
            ],
        }
    }
}

#[derive(Component, Clone)]
pub struct VisionBlocker {
    pub sides: Vec<Direction>,
}

#[allow(dead_code)]
impl VisionBlocker {
    pub fn new(sides: Vec<Direction>) -> VisionBlocker {
        VisionBlocker { sides }
    }
    pub fn new_cardinal_sides() -> VisionBlocker {
        VisionBlocker {
            sides: vec![Direction::N, Direction::E, Direction::S, Direction::W],
        }
    }
    pub fn new_n_s() -> VisionBlocker {
        VisionBlocker {
            sides: vec![
                Direction::N,
                Direction::NW,
                Direction::W,
                Direction::SW,
                Direction::S,
                Direction::SE,
                Direction::E,
                Direction::UP,
                Direction::DOWN,
            ],
        }
    }
    pub fn new_e_w() -> VisionBlocker {
        VisionBlocker {
            sides: vec![
                Direction::N,
                Direction::NW,
                Direction::W,
                Direction::SW,
                Direction::S,
                Direction::SE,
                Direction::E,
                Direction::UP,
                Direction::DOWN,
            ],
        }
    }
    pub fn new_all_sides() -> VisionBlocker {
        VisionBlocker {
            sides: vec![
                Direction::N,
                Direction::W,
                Direction::S,
                Direction::E,
                Direction::UP,
                Direction::DOWN,
            ],
        }
    }
}

#[derive(Component, ConvertSaveload, Clone)]
pub struct Illuminant {
    pub intensity: f32,
    pub range: usize,
    pub color: RGBA,
    pub beam_angle: f32,
    pub on: bool,
    pub dirty: bool,
}

impl Illuminant {
    pub fn new(intensity: f32, range: usize, color: RGBA, beam_angle: f32, on: bool) -> Illuminant {
        Illuminant {
            intensity,
            range,
            color,
            beam_angle,
            on,
            dirty: true,
        }
    }

    pub fn set_state(&mut self, on: bool) {
        let previous_state = self.on;
        self.on = on;

        if self.on != previous_state {
            self.dirty = true;
        }
    }
}

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct Photometry {
    pub light_level: f32,
    pub light_color: RGBA,
    pub dirty: bool,
}

//TODO: Once lighting is calculated set initial light level to 0.0
impl Photometry {
    pub fn new() -> Photometry {
        Photometry {
            light_level: 1.0,
            light_color: RGB::named(rltk::WHITE).to_rgba(1.0),
            dirty: true,
        }
    }
}

#[derive(Component, ConvertSaveload, Clone, Debug)]
pub struct Name {
    pub name: String,
}

impl Name {
    pub fn new(name: String) -> Name {
        Name { name }
    }
}

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct SerializeThis;

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct SerializationHelper {
    pub map: map::Map,
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Door {
    pub open: bool,
    pub interaction_description: String,
    pub interaction_id: u32,
    pub open_glyph: u16,
    pub closed_glyph: u16,
    pub cost: f32,
}

impl Door {
    pub fn new(open: bool, open_glyph: u16, closed_glyph: u16) -> Door {
        let description: String;
        if open {
            description = "Close".to_string()
        } else {
            description = "Open".to_string()
        }

        Door {
            open,
            interaction_description: description.to_string(),
            interaction_id: crate::rng::random_int() as u32,
            open_glyph,
            closed_glyph,
            cost: 1.0,
        }
    }

    pub fn open_close(&mut self) {
        self.open = !self.open;

        if self.open {
            self.interaction_description = "Close".to_string()
        } else {
            self.interaction_description = "Open".to_string()
        }
    }

    pub fn state_description(&self) -> String {
        if self.open {
            "open".to_string()
        } else {
            "closed".to_string()
        }
    }
}

impl Interactable for Door {
    fn get_cost(&self) -> f32 {
        self.cost
    }
    fn interact(&mut self) {
        self.open_close();
    }

    fn interaction_id(&self) -> u32 {
        self.interaction_id
    }

    fn interaction_description(&self) -> String {
        self.interaction_description.clone()
    }

    fn state_description(&self) -> String {
        self.state_description()
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct EntityDirection {
    pub direction: Direction,
}

impl EntityDirection {
    pub fn new(direction: Direction) -> EntityDirection {
        EntityDirection { direction }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Duct {}

#[allow(dead_code)]
impl Duct {
    pub fn new() -> Duct {
        Duct {}
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Prop {}

impl Prop {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Item {}

impl Item {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct InContainer {
    pub owner: u32,
}

impl InContainer {
    pub fn new(entity_id: u32) -> Self {
        Self { owner: entity_id }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Installed {}

impl Installed {
    pub fn new() -> Self {
        Self {}
    }
}
