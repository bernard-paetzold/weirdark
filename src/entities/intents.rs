use crate::vectors::Vector3i;
use serde::Deserialize;
use serde::Serialize;
use specs::error::NoError;
use specs::prelude::*;
use specs::saveload::{ConvertSaveload, Marker};
use specs_derive::*;

#[derive(Component, Clone)]
pub struct MoveIntent {
    pub current_position: Vector3i,
    pub delta: Vector3i,
    pub cost: f32,
    pub remaining_cost: f32,
}

impl MoveIntent {
    pub fn new(current_position: Vector3i, delta: Vector3i) -> Self {
        let cost;
        //Calculate time taken
        if delta == Vector3i::N || delta == Vector3i::E || delta == Vector3i::S || delta == Vector3i::W {
            cost = 1.0;
        }
        else {
            cost = std::f32::consts::SQRT_2;
        }

        Self {
            current_position,
            delta,
            cost,
            remaining_cost: cost,
        }
    }
}

impl Intent for MoveIntent {
    fn get_cost(&self) -> f32 {
        self.cost
    }
    fn get_remaining_cost(&self) -> f32 {
        self.remaining_cost
    }
    fn update_remaining_cost(&mut self, delta: f32) {
        self.remaining_cost += delta;
    }
    fn execute(&mut self) {
        
    }
}

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Initiative {
    pub current : f32,
}

#[allow(dead_code)]
impl Initiative {
    pub fn new(default: f32) -> Self {
        Self {
            current: default,
        }
    }
    pub fn adjust(&mut self, delta: f32) {
        self.current += delta;
    }
}

#[allow(dead_code)]
#[derive(Component, Clone)]
pub struct InteractIntent {
    pub initiator: Entity,
    pub target: Entity,
    pub interaction_id: usize,
    pub interaction_description: String,
    pub cost: f32,
    pub remaining_cost: f32,
}

impl InteractIntent {
    pub fn new(initiator: Entity, target: Entity, interaction_id: usize, interaction_description: String, cost: f32) -> InteractIntent {
        InteractIntent {
            initiator,
            target,
            interaction_id,
            interaction_description,
            cost,
            remaining_cost: cost,
        }
    }
}

impl Intent for InteractIntent {
    fn get_cost(&self) -> f32 {
        self.cost
    }
    fn get_remaining_cost(&self) -> f32 {
        self.remaining_cost
    }
    fn update_remaining_cost(&mut self, delta: f32) {
        self.remaining_cost += delta;
    }
    fn execute(&mut self) {
        
    }
}

#[allow(dead_code)]
pub trait Interactable {
    fn interaction_id(&self) -> usize;
    fn interaction_description(&self) -> String;
    fn state_description(&self) -> String;
    fn interact(&mut self);
    fn get_cost(&self) -> f32;
}

#[allow(dead_code)]
pub trait Intent {
    fn get_cost(&self) -> f32;
    fn get_remaining_cost(&self) -> f32;
    fn update_remaining_cost(&mut self, delta: f32);
    //fn interaction_id(&self) -> usize;
    //fn interaction_description(&self) -> String;
    //fn state_description(&self) -> String;
    fn execute(&mut self);
}