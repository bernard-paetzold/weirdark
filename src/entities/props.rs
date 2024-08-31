use serde::Deserialize;
use serde::Serialize;
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Cabinet {}

impl Cabinet {
    pub fn new() -> Self {
        Self {}
    }
}
