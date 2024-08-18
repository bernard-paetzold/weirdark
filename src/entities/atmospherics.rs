use std::collections::HashMap;
use std::fmt::Display;

use serde::Deserialize;
use serde::Serialize;
use specs::prelude::*;
use specs_derive::*;

pub const R: f32 =  8.31446261815324;
pub const K: f32 = 273.15;


#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Hash, Debug, Copy)]
pub enum Gas {
    Oxygen,
    Nitrogen,
    CarbonDioxide,
}

impl Display for Gas {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gas_name;
        match self {
            Gas::Oxygen => gas_name = "Oxygen",
            Gas::Nitrogen => gas_name = "Nitrogen",
            Gas::CarbonDioxide => gas_name = "CarbonDioxide",
        }
        write!(f, "{}", gas_name)
    }
}

#[derive(Component, Default, Serialize, Deserialize, Clone, Debug)]
pub struct Atmosphere {
    pub pressure: f32,
    pub temperature: f32,
    pub gasses: HashMap<Gas, f32>,
    pub space_id: usize,
    pub dirty: bool,
}

#[allow(dead_code)]
impl Atmosphere {
    pub fn new(pressure: f32, temperature: f32, gasses: HashMap<Gas, f32>, dirty: bool) -> Self {
        Self { 
            pressure,
            temperature,
            gasses,
            space_id: crate::rng::random_int() as usize,
            dirty,
        }
    }
    pub fn new_vacuume() -> Self {
        Self { 
            pressure: 0.0,
            temperature: 2.7,
            gasses: HashMap::new(),
            space_id: crate::rng::random_int() as usize,
            dirty: false,
        }
    }
    pub fn new_stp() -> Self {
        let mut gasses = HashMap::new();
        gasses.insert(Gas::Oxygen,  9.3043588);
        gasses.insert(Gas::Nitrogen, 32.9881812);

        let mut atmosphere =
        Self { 
            pressure: 0.0,
            temperature: 288.15,
            gasses,
            space_id: crate::rng::random_int() as usize,
            dirty: true,
        };

        atmosphere.recalculate_pressure();

        atmosphere
    }
    pub fn get_pressure(&self) -> f32 {
        self.pressure
    }
    pub fn get_pressure_kpa(&self) -> f32 {
        self.pressure * 0.001
    }
    pub fn get_temperature(&self) -> f32 {
        self.temperature
    }
    pub fn get_celcius_temperature(&self) -> f32 {
        self.temperature - K
    }
    pub fn update_gas(&mut self, gasses: &HashMap<Gas, f32>, incoming_temperature: f32) {
        for (gas, delta_mols) in gasses.iter() {
            //Recalculate temperature
            self.temperature = ((self.get_total_mols() * self.temperature) + (delta_mols * incoming_temperature)) / (self.get_total_mols() + delta_mols);

            //Apply new gas
            if let Some(current_mols) = self.gasses.get(&gas) {
                if current_mols + delta_mols > 0.0 {
                    self.gasses.insert(gas.clone(), current_mols + delta_mols);
                }
                else {
                    self.gasses.remove(gas);
                }
            }
            else if *delta_mols > 0.0 {
                self.gasses.insert(gas.clone(), *delta_mols);
            }
        }

        //Recalculate pressure
        self.recalculate_pressure();
        self.dirty = true;
    }
    pub fn set_gasses(&mut self, gasses: &HashMap<Gas, f32>, incoming_temperature: f32) {
        //Apply new gas
        self.gasses = gasses.clone();

        //Recalculate pressure
        self.temperature = incoming_temperature;
        self.recalculate_pressure();
        self.dirty = true;
    }
    pub fn set_gas(&mut self, gas: &Gas, mols: f32,  incoming_temperature: f32) {
        //Apply new gas
        self.gasses.insert(*gas, mols);

        //Recalculate pressure
        self.temperature = incoming_temperature;
        self.recalculate_pressure();
        self.dirty = true;
    }
    pub fn transfer_gas(&self, other: &mut Self, delta_mols: f32) {
        let mut delta_gasses = HashMap::new();
        let mut total_transferred = 0.0;
        let gases: Vec<_> = self.gasses.keys().cloned().collect();
        
        for (i, gas) in gases.iter().enumerate() {
            let ratio = self.get_gas_ratio(*gas);
            let transfer = if i == gases.len() - 1 {
                // For the last gas, transfer the remaining amount
                delta_mols - total_transferred
            } else {
                delta_mols * ratio
            };
            
            delta_gasses.insert(*gas, transfer);
            total_transferred += transfer;
        }
    
        other.update_gas(&delta_gasses, self.temperature);
        other.dirty = true;
    }
    pub fn transfer_gas_except(&self, other: &mut Self, delta_mols: f32, exclude_gas: Gas) {
        let mut delta_gasses = HashMap::new();
        let mut total_transferred = 0.0;
        let gases: Vec<_> = self.gasses.keys().cloned().filter(|&g| g != exclude_gas).collect();
        
        for (i, gas) in gases.iter().enumerate() {
            let ratio = self.get_gas_ratio_except(*gas, exclude_gas);
            let transfer = if i == gases.len() - 1 {
                // For the last gas, transfer the remaining amount
                delta_mols - total_transferred
            } else {
                delta_mols * ratio
            };
            
            delta_gasses.insert(*gas, transfer);
            total_transferred += transfer;
        }
    
        other.update_gas(&delta_gasses, self.temperature);
        other.dirty = true;
    }
    
    pub fn get_gas_ratio_except(&self, gas: Gas, exclude_gas: Gas) -> f32 {
        let total_mols_except = self.get_total_mols_except(exclude_gas);
        if total_mols_except == 0.0 {
            return 0.0;
        }
        self.gasses.get(&gas).unwrap_or(&0.0) / total_mols_except
    }
    
    pub fn get_total_mols_except(&self, exclude_gas: Gas) -> f32 {
        self.gasses.iter()
            .filter(|&(&g, _)| g != exclude_gas)
            .map(|(_, &mols)| mols)
            .sum()
    }
    pub fn transfer_single_gas(&self, other: &mut Self, gas: Gas, delta_mols: f32) {
        let mut delta_gasses = HashMap::new();

        delta_gasses.insert(gas, delta_mols);

        other.update_gas(&delta_gasses, self.temperature);
        other.dirty = true;
    }
    pub fn remove_gas(&mut self, delta_mols: f32) {
        let mut delta_gasses = HashMap::new();

        for (gas, _) in &self.gasses {
            delta_gasses.insert(gas.clone(), - delta_mols.clone() * self.get_gas_ratio(*gas));
        }

        self.update_gas(&delta_gasses, self.temperature);
        self.dirty = true;
    }
    pub fn remove_gas_except(&mut self, delta_mols: f32, exclude_gas: Gas) {
        let mut delta_gasses = HashMap::new();
        let total_mols_except = self.get_total_mols_except(exclude_gas);
    
        if total_mols_except > 0.0 {
            for (gas, mols) in &self.gasses {
                if *gas != exclude_gas {
                    let ratio = *mols / total_mols_except;
                    delta_gasses.insert(*gas, -delta_mols * ratio);
                }
            }
    
            self.update_gas(&delta_gasses, self.temperature);
            self.dirty = true;
        }
    }
    pub fn remove_single_gas(&mut self, gas: Gas, delta_mols: f32) {
        let mut delta_gasses = HashMap::new();

        delta_gasses.insert(gas.clone(), - delta_mols.clone());
        
        self.update_gas(&delta_gasses, self.temperature);
        self.dirty = true;
    }
    pub fn update_temperature(&mut self, delta_t: f32) {
        //Apply new temperature
        self.temperature += delta_t;

        self.dirty = true;
        //Recalculate pressure
        self.recalculate_pressure();
    }
    pub fn get_total_mols(&self) -> f32 {
        let mut total_mols = 0.0;
        let _ = self.gasses.iter().for_each(|(_, x)| total_mols += x);

        total_mols
    }
    pub fn recalculate_pressure(&mut self) {
        self.pressure = self.get_total_mols() * R * self.temperature;
        self.dirty = true;
    }
    pub fn recalculate_temperature(&mut self) {
        self.temperature = self.pressure / (self.get_total_mols() * R);
        self.dirty = true;
    }
    pub fn get_gas_ratio(&self, gas: Gas) -> f32 {
        self.gasses.get(&gas).unwrap_or(&0.0) / self.get_total_mols()
    }

}