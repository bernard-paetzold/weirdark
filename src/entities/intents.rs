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

#[derive(Component, Clone)]
pub struct MoveIntent {
    current_position: Vector3i,
    target_position: Vector3i,
}

impl MoveIntent {
    pub fn new(current_position: Vector3i, target_position: Vector3i) -> Self {
        Self {
            current_position,
            target_position,
        }
    }
}

impl Intent for MoveIntent {
    fn execute(&mut self) {
        
    }
}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Initiative {
    pub current : f32,
}

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

pub trait Intent {
    //fn interaction_id(&self) -> usize;
    //fn interaction_description(&self) -> String;
    //fn state_description(&self) -> String;
    fn execute(&mut self);
}