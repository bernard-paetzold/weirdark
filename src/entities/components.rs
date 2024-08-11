use std::collections::HashSet;
use std::path::Display;

use crate::map;
use crate::vectors::Vector3i;
use rltk::{RGB, RGBA};
use serde::Deserialize;
use serde::Serialize;
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

#[derive(Default, Debug, Serialize, PartialEq, Deserialize, Clone)]
pub enum Direction {
    #[default] N,
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

#[derive(Component, Clone, Debug)]
pub struct Blocker {
    pub sides: Vec<Direction>,
}

impl Blocker {
    pub fn new(sides: Vec<Direction>) -> Blocker {
        Blocker {
            sides,
        }
    }
    pub fn new_cardinal_sides() -> Blocker {
        Blocker {
            sides: vec![Direction::N, Direction::E, Direction::S, Direction::W]
        }
    }
    pub fn new_n_s() -> Blocker {
        Blocker {
            sides: vec![Direction::N, Direction::NW, Direction::W,
            Direction::SW, Direction::S, Direction::SE, 
            Direction::E, Direction::UP, Direction::DOWN],
        }
    }
    pub fn new_e_w() -> Blocker {
        Blocker {
            sides: vec![Direction::N, Direction::NW, Direction::W,
            Direction::SW, Direction::S, Direction::SE, 
            Direction::E, Direction::UP, Direction::DOWN],
        }
    }
    pub fn new_all_sides() -> Blocker {
        Blocker {
            sides: vec![Direction::N, Direction::W,
                        Direction::S, Direction::E, 
                        Direction::UP, Direction::DOWN],
        }
    }
}

#[derive(Component, Clone)]
pub struct VisionBlocker {
    pub sides: Vec<Direction>,
}

impl VisionBlocker {
    pub fn new(sides: Vec<Direction>) -> VisionBlocker {
        VisionBlocker {
            sides,
        }
    }
    pub fn new_cardinal_sides() -> VisionBlocker {
        VisionBlocker {
            sides: vec![Direction::N, Direction::E, Direction::S, Direction::W]
        }
    }
    pub fn new_n_s() -> VisionBlocker {
        VisionBlocker {
            sides: vec![Direction::N, Direction::NW, Direction::W,
            Direction::SW, Direction::S, Direction::SE, 
            Direction::E, Direction::UP, Direction::DOWN],
        }
    }
    pub fn new_e_w() -> VisionBlocker {
        VisionBlocker {
            sides: vec![Direction::N, Direction::NW, Direction::W,
            Direction::SW, Direction::S, Direction::SE, 
            Direction::E, Direction::UP, Direction::DOWN],
        }
    }
    pub fn new_all_sides() -> VisionBlocker {
        VisionBlocker {
            sides: vec![Direction::N, Direction::W,
                        Direction::S, Direction::E, 
                        Direction::UP, Direction::DOWN],
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

#[allow(dead_code)]
#[derive(Component, Clone)]
pub struct InteractIntent {
    pub initiator: Entity,
    pub target: Entity,
    pub interaction_id: usize,
    pub interaction_description: String,
}

impl InteractIntent {
    pub fn new(initiator: Entity, target: Entity, interaction_id: usize, interaction_description: String) -> InteractIntent {
        InteractIntent {
            initiator,
            target,
            interaction_id,
            interaction_description,
        }
    }
}

#[allow(dead_code)]
pub trait Interactable {
    fn interaction_id(&self) -> usize;
    fn interaction_description(&self) -> String;
    fn state_description(&self) -> String;
    fn interact(&mut self);
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct PoweredState {
    pub on: bool,
    pub wattage: f32,
    pub available_wattage: f32
}

impl PoweredState {
    pub fn new(on: bool, wattage: f32) -> PoweredState {
        PoweredState { on, wattage, available_wattage: 0.0 }
    }

    pub fn state_description(&self) -> String {
        if self.on && (self.available_wattage >= self.wattage) { 
            "on, powered".to_string() 
        } 
        else if self.on && (self.available_wattage < self.wattage) { 
            "on, not powered".to_string() 
        }
        else if !self.on && (self.available_wattage >= self.wattage) { 
            "off, powered".to_string() 
        }
        else { 
            "off, not powered".to_string() 
        }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct PowerSwitch {
    pub on: bool,
    pub interaction_description: String,
    pub interaction_id: usize,
}

impl PowerSwitch {
    pub fn new(on: bool) -> PowerSwitch {

        let description: String;
        if on { description = "Turn off".to_string() } else { description = "Turn on".to_string() }

        PowerSwitch {
            on,
            interaction_description: description.to_string(),
            interaction_id: crate::rng::random_int() as usize,
        }
    }

    pub fn toggle(&mut self) {
        self.on = !self.on;

        if self.on { self.interaction_description = "Turn off".to_string() } 
        else { self.interaction_description = "Turn on".to_string() }
    }

    pub fn state_description(&self) -> String {
        if self.on { "on".to_string() } else { "off".to_string() }
    }
}

impl Interactable for PowerSwitch {
    fn interact(&mut self) {
        self.toggle();
    }
        
    fn interaction_id(&self) -> usize {
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
pub struct Door {
    pub open: bool,
    pub interaction_description: String,
    pub interaction_id: usize,
    pub open_glyph: rltk::FontCharType,
    pub closed_glyph: rltk::FontCharType,
}

impl Door {
    pub fn new(open: bool, open_glyph: rltk::FontCharType, closed_glyph: rltk::FontCharType) -> Door {
        let description: String;
        if open { description = "Close".to_string() } else { description = "Open".to_string() }

        Door {
            open,
            interaction_description: description.to_string(),
            interaction_id: crate::rng::random_int() as usize,
            open_glyph,
            closed_glyph
        }
    }

    pub fn open_close(&mut self) {
        self.open = !self.open;

        if self.open { self.interaction_description = "Close".to_string() } 
        else { self.interaction_description = "Open".to_string() }
    }

    pub fn state_description(&self) -> String {
        if self.open { "open".to_string() } else { "closed".to_string() }
    }
}

impl Interactable for Door {
    fn interact(&mut self) {
        self.open_close();
    }
    
    fn interaction_id(&self) -> usize {
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
pub struct Power {
    pub wattage: f32,
}

#[allow(dead_code)]
impl Power {
    pub fn new(wattage: f32) -> Power {
        Power {
            wattage,
        }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Wire {
    pub power_load: f32,
    pub available_wattage: f32,
}

impl Wire {
    pub fn new() -> Wire {
        Wire {
            power_load: 0.0,
            available_wattage: 0.0,
        }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct EntityDirection {
    pub direction: Direction,
}

impl EntityDirection {
    pub fn new(direction: Direction) -> EntityDirection {
        EntityDirection {
            direction,
        }
    }
}


#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct PowerSource {
    pub on: bool,
    pub max_wattage: f32,
    pub available_wattage: f32,
}

impl PowerSource {
    pub fn new(on: bool, max_wattage: f32) -> PowerSource {
        PowerSource {
            on,
            max_wattage,
            available_wattage: max_wattage,
        }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Duct {}

impl Duct {
    pub fn new() -> Duct {
        Duct { }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct PowerNode {
    pub network_id: usize,
    pub dirty: bool,
}

impl PowerNode {
    pub fn new() -> Self {
        Self {
            network_id: crate::rng::random_int() as usize,
            dirty: true,
        }
    }
}

