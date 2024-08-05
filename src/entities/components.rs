use std::collections::HashSet;

use crate::map;
use crate::vectors::Vector3i;
use rltk::{RandomNumberGenerator, RGB, RGBA};
use serde::Deserialize;
use serde::Serialize;
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

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

#[derive(Component, Clone)]
pub struct InteractIntent {
    pub initiator: Entity,
    pub target: Entity,
    pub interaction_id: String,
    pub interaction_description: String,
}

impl InteractIntent {
    pub fn new(initiator: Entity, target: Entity, interaction_id: String, interaction_description: String) -> InteractIntent {
        InteractIntent {
            initiator,
            target,
            interaction_id,
            interaction_description,
        }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Power {
    pub powered: bool,
    pub on: bool,
}

pub trait Interactable {
    fn interact(&mut self) -> bool;
}

impl Power {
    pub fn new(powered: bool, on: bool) -> Power {
        Power { powered, on }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct PowerSwitch {
    pub on: bool,
    pub interaction_description: String,
    pub interaction_id: String,
}

impl PowerSwitch {
    pub fn new(on: bool) -> PowerSwitch {
        let mut rng = RandomNumberGenerator::new();

        PowerSwitch {
            on,
            interaction_description: "Toggle power switch".to_string(),
            interaction_id: rng.next_u64().to_string(),
        }
    }

    pub fn toggle(&mut self) {
        self.on = !self.on;
    }
}

impl Interactable for PowerSwitch {
    fn interact(&mut self) -> bool {
        self.toggle();
        true
    }
}
