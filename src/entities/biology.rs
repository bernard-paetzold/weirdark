use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;
use specs::prelude::*;
use specs_derive::*;

use super::atmospherics::Atmosphere;
use super::atmospherics::Gas;

#[derive(Component, Default, Serialize, Deserialize, Clone)]
pub struct Breather {
    //Gas, ideal mols, needed ratio to reach ideal
    pub in_gasses: HashMap<Gas, (f32, f32)>,
    pub out_gasses: HashMap<Gas, f32>,
    pub trigger_breath: bool,
    pub temperature: f32,    
}

impl Breather {
    pub fn new_humanlike() -> Self {
        let mut in_gasses = HashMap::new();
        in_gasses.insert(Gas::Oxygen, (0.0011, 0.21));

        let mut out_gasses = HashMap::new();
        //out_gasses.insert(Gas::CarbonDioxide, 0.0012);        
        out_gasses.insert(Gas::CarbonDioxide, 0.01);        
        out_gasses.insert(Gas::Oxygen, 0.01);        
        out_gasses.insert(Gas::Nitrogen, 0.02);        

        Self {
            in_gasses,
            out_gasses,
            trigger_breath: true,
            temperature: 37.0 + super::atmospherics::K,
        }
    }
    pub fn breath(&mut self, atmosphere: &mut Atmosphere) {
        let mut gas_changes = Vec::new();
                    
        for (atmosphere_gas, _) in atmosphere.gasses.iter().filter(|(gas, _)| self.in_gasses.contains_key(gas)) {
            if let Some((consumption, ideal_ratio)) = self.in_gasses.get(atmosphere_gas) {
                let amount = consumption * (ideal_ratio * atmosphere.get_gas_ratio(*atmosphere_gas));
                gas_changes.push((*atmosphere_gas, amount));
            }
        } 
        // Then, apply all the changes
        for (gas, amount) in gas_changes {
            atmosphere.remove_single_gas(gas, amount);
        }  

        //TODO: Make the output gasses dependent on the linked input gas
        atmosphere.update_gas(&self.out_gasses, self.temperature);
        atmosphere.dirty = true;
    }
}