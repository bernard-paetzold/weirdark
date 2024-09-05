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

#[derive(Default, Debug, Serialize, PartialEq, Hash, Eq, Deserialize, Clone)]
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
    pub airtight: bool,
}

#[allow(dead_code)]
impl Blocker {
    pub fn new(sides: Vec<Direction>, airtight: bool) -> Blocker {
        Blocker { sides, airtight }
    }
    pub fn new_cardinal_sides(airtight: bool) -> Blocker {
        Blocker {
            sides: vec![Direction::N, Direction::E, Direction::S, Direction::W],
            airtight,
        }
    }
    pub fn new_n_s(airtight: bool) -> Blocker {
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
            airtight,
        }
    }
    pub fn new_e_w(airtight: bool) -> Blocker {
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
            airtight,
        }
    }
    pub fn new_all_sides(airtight: bool) -> Blocker {
        Blocker {
            sides: vec![
                Direction::N,
                Direction::W,
                Direction::S,
                Direction::E,
                Direction::UP,
                Direction::DOWN,
            ],
            airtight,
        }
    }

    pub fn remove_side(&mut self, side_to_remove: Direction) {
        let mut index = -1;
        let mut count = 0;

        for side in self.sides.iter() {
            if *side == side_to_remove {
                index = count;
                break;
            }
            count += 1;
        }

        if index >= 0 {
            self.sides.remove(index as usize);
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

    pub fn remove_side(&mut self, side_to_remove: Direction) {
        let mut index = -1;
        let mut count = 0;

        for side in self.sides.iter() {
            if *side == side_to_remove {
                index = count;
                break;
            }
            count += 1;
        }

        if index >= 0 {
            self.sides.remove(index as usize);
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
pub struct Item {
    pub volume: f32,
    pub weight: f32,
}

impl Item {
    pub fn new(volume: f32, weight: f32) -> Self {
        Self { volume, weight }
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

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Container {
    pub volume: f32,
    pub remaining_volume: f32,
    pub interaction_description: String,
    pub id: u32,
    pub open: bool,
    pub cost: f32,
}

impl Container {
    pub fn new(volume: f32) -> Self {
        Self {
            volume,
            remaining_volume: volume,
            interaction_description: "Closed".to_string(),
            id: crate::rng::random_int() as u32,
            open: false,
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

    pub fn try_insert_item(&mut self, volume: f32) -> bool {
        if self.remaining_volume - volume >= 0.0 {
            self.remaining_volume -= volume;
            true
        } else {
            false
        }
    }

    pub fn remove_item(&mut self, volume: f32) {
        self.remaining_volume = (self.remaining_volume + volume).min(self.volume);
    }
}
