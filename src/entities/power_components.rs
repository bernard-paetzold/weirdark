use rltk::RGBA;
use serde::Deserialize;
use serde::Serialize;
use specs::prelude::*;
use specs_derive::*;

use super::intents::Interactable;

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
    pub available_wattage: f32,
}

impl PoweredState {
    pub fn new(on: bool, wattage: f32) -> PoweredState {
        PoweredState {
            on,
            wattage,
            available_wattage: 0.0,
        }
    }

    pub fn state_description(&self) -> String {
        if self.on && (self.available_wattage >= self.wattage) {
            "on, powered".to_string()
        } else if self.on && (self.available_wattage < self.wattage) {
            "on, not powered".to_string()
        } else if !self.on && (self.available_wattage >= self.wattage) {
            "off, powered".to_string()
        } else {
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
    pub interaction_id: u32,
    pub cost: f32,
}

impl PowerSwitch {
    pub fn new(on: bool) -> PowerSwitch {
        let description: String;
        if on {
            description = "Turn off".to_string()
        } else {
            description = "Turn on".to_string()
        }

        PowerSwitch {
            on,
            interaction_description: description.to_string(),
            interaction_id: crate::rng::random_int() as u32,
            cost: 0.5,
        }
    }

    pub fn toggle(&mut self) {
        self.on = !self.on;

        if self.on {
            self.interaction_description = "Turn off".to_string()
        } else {
            self.interaction_description = "Turn on".to_string()
        }
    }

    pub fn state_description(&self) -> String {
        if self.on {
            "on".to_string()
        } else {
            "off".to_string()
        }
    }
}

impl Interactable for PowerSwitch {
    fn get_cost(&self) -> f32 {
        self.cost
    }
    fn interact(&mut self) {
        self.toggle();
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
pub struct Wire {
    pub power_load: f32,
    pub available_wattage: f32,
    pub color: RGBA,
    pub color_name: String,
    pub data: bool,
}

impl Wire {
    pub fn new(color: RGBA, color_name: String, data: bool) -> Wire {
        Wire {
            power_load: 0.0,
            available_wattage: 0.0,
            color,
            color_name,
            data,
        }
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone, Debug)]
pub struct ElectronicHeater {
    pub target_temperature: f32,
    pub on: bool,
}

#[allow(dead_code)]
impl ElectronicHeater {
    pub fn new(target_temperature: f32, on: bool) -> Self {
        Self {
            target_temperature,
            on,
        }
    }

    pub fn check_status(&mut self, current_temperature: f32) -> bool {
        self.on = current_temperature < self.target_temperature;

        self.on
    }

    pub fn set_state(&mut self, on: bool) {
        self.on = on;
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone, Debug)]
pub struct Wiring {
    pub end_points: Vec<f32>,
}
