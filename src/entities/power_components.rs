use serde::Deserialize;
use serde::Serialize;
use specs::prelude::*;
use specs_derive::*;

use crate::Interactable;

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
pub struct BreakerBox {}

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