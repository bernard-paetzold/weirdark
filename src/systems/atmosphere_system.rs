use std::collections::{HashMap, HashSet};

use specs::prelude::*;

use crate::{
    entities::atmospherics::{Gas, R},
    vectors::{
        utils::get_neighbours,
        Vector3i,
    },
    Map,
};

const PRESSURE_THRESHOLD: f32 = 0.000001;
//const PRESSURE_THRESHOLD: f32 = 0.000001;
const DISSIPATION_THRESHOLD: f32 = 0.000001;
const DIFFUSION_SMOOTHING: f32 = 0.8;
const PRESSURE_SMOOTHING: f32 = 0.6;


pub struct AtmosphereSystem {}

impl<'a> System<'a> for AtmosphereSystem {
    type SystemData = (WriteExpect<'a, Map>,);

    fn run(&mut self, data: Self::SystemData) {
        //let now = std::time::Instant::now();
        let (mut map,) = data;

        let mut dirty_atmospheres = HashSet::new();

        for (position, tile) in map
            .tiles
            .iter_mut()
            .filter(|(_, tile)| tile.atmosphere.dirty)
        {
            dirty_atmospheres.insert(position.clone());
            //Reset dirty
            tile.atmosphere.recalculate_temperature();
            tile.atmosphere.recalculate_pressure();
            tile.atmosphere.dirty = false;
        }



        for position in dirty_atmospheres.iter() {
            println!("Updating atmos: {}", dirty_atmospheres.len());
            let neighbours = get_accessible_neighbours(&map, position).clone();

            let mut temperature = 0.0;
            let mut pressure = 0.0;

            let mut neighbour_mol_deltas: HashMap<Gas, HashMap<Vector3i, f32>> = HashMap::new();
            //let mut total_deltas_by_gas: HashMap<Gas, f32> = HashMap::new();
            let mut neighbour_count: HashMap<Gas, usize> = HashMap::new();
            let mut total_mols_by_gas: HashMap<Gas, f32> = HashMap::new();

            let mut neighbour_pressure_deltas: HashMap<Vector3i, f32> = HashMap::new();
            let mut total_delta = 0.0;
            let mut total_pressure = 0.0;

            let mut clean_tiles = HashMap::new();

            let mut higher_pressure_neighbours = Vec::new();

            if let Some(current_tile) = map.tiles.get(position) {

                temperature = current_tile.atmosphere.temperature;
                pressure = current_tile.atmosphere.pressure;

                //Get the mols of all the neighbours
                for neighbour in neighbours.iter() {
                    if let Some(neighbour_tile) = map.tiles.get(neighbour) {
                        let neighbour_pressure = neighbour_tile.atmosphere.pressure;

                        if neighbour_pressure <= pressure {
                            if neighbour_pressure == pressure {
                                for (gas, mols) in current_tile.atmosphere.gasses.iter() {
                                    if let Some(neighbour_mols) = neighbour_tile.atmosphere.gasses.get(gas) {
                                        if mols > neighbour_mols {
                                            let delta = mols - neighbour_mols;
                                            
                                            if delta > DISSIPATION_THRESHOLD {
                                                neighbour_mol_deltas
                                                .entry(*gas)
                                                .or_insert_with(HashMap::new)
                                                .insert(*neighbour, delta);
        
                                                *neighbour_count.entry(*gas).or_insert(0) += 1;
                                                *total_mols_by_gas.entry(*gas).or_insert(0.0) += delta;
                                            }
                                        }
                                    }
                                    else if *mols > DISSIPATION_THRESHOLD {
                                        neighbour_mol_deltas
                                            .entry(*gas)
                                            .or_insert_with(HashMap::new)
                                            .insert(*neighbour, *mols);
        
                                        *neighbour_count.entry(*gas).or_insert(0) += 1;
        
                                        //*total_mols_by_gas.entry(*gas).or_insert(0.0) += *mols;                 
                                    }
                                }
                            }
                            else if (pressure - neighbour_pressure) > PRESSURE_THRESHOLD {
                                let delta = pressure - neighbour_pressure;
                                neighbour_pressure_deltas.insert(*neighbour, delta);
                                total_delta += delta;
                                total_pressure += neighbour_pressure;
                            }
                            else {
                                clean_tiles.insert(neighbour, true);
                            }
                        } else {
                            higher_pressure_neighbours.push(neighbour);
                        }
                    }
                }
            }

            let average_pressure =
                (total_pressure + pressure) / (neighbour_pressure_deltas.len() + 1) as f32;
            let excess_pressure = pressure - average_pressure;
            //Distribute excess between neighbours proportionally to the difference from own
            for (neighbour, delta) in neighbour_pressure_deltas.iter() {
                let proportion: f32 = *delta / total_delta;
                let mols_to_transfer = ((excess_pressure * proportion) / (temperature * R)) * (delta / pressure) * PRESSURE_SMOOTHING;

                if mols_to_transfer < DISSIPATION_THRESHOLD {
                    continue;
                }

                clean_tiles.insert(neighbour, false);

                //Add gas to neighbouring tile
                let mut current_tile = None;
                if let Some(tile) = map.tiles.get(position) {
                    current_tile = Some(tile.clone());
                }

                if let Some(current_tile) = current_tile {
                    if let Some(neighbour_tile) = map.tiles.get_mut(&neighbour) {
                        current_tile
                            .atmosphere
                            .transfer_gas(&mut neighbour_tile.atmosphere, mols_to_transfer);
                    }
                }

                //Remove gas from current tile
                if let Some(current_tile) = map.tiles.get_mut(position) {
                    current_tile.atmosphere.remove_gas(mols_to_transfer);
                }
            }

            if let Some(current_tile) = map.tiles.get(position) {
                //Get the mols of all the neighbours
                let pressure = current_tile.atmosphere.pressure;

                for neighbour in neighbours.iter() {
                    if let Some(neighbour_tile) = map.tiles.get(neighbour) {
                        //if pressure - neighbour_tile.atmosphere.pressure <= PRESSURE_THRESHOLD {
                            for (gas, mols) in current_tile.atmosphere.gasses.iter() {
                                if let Some(neighbour_mols) = neighbour_tile.atmosphere.gasses.get(gas) {
                                    if mols > neighbour_mols {
                                        let delta = mols - neighbour_mols;
                                        
                                        if delta > DISSIPATION_THRESHOLD {
                                            neighbour_mol_deltas
                                            .entry(*gas)
                                            .or_insert_with(HashMap::new)
                                            .insert(*neighbour, delta);
    
                                            *neighbour_count.entry(*gas).or_insert(0) += 1;
                                            *total_mols_by_gas.entry(*gas).or_insert(0.0) += delta;
                                        }
                                    }
                                }
                                else if *mols > DISSIPATION_THRESHOLD {
                                    neighbour_mol_deltas
                                        .entry(*gas)
                                        .or_insert_with(HashMap::new)
                                        .insert(*neighbour, *mols);
    
                                    *neighbour_count.entry(*gas).or_insert(0) += 1;
    
                                    //*total_mols_by_gas.entry(*gas).or_insert(0.0) += *mols;                 
                                }
                            }
                        //}
                    }
                }
            }

            for (gas, gas_deltas) in neighbour_mol_deltas.iter() {
                for (neighbour, delta) in gas_deltas.iter() {
                    clean_tiles.insert(neighbour, true);
                }
            }

            //If neighbours are missing gasses current tile has, share them
            for (gas, gas_deltas) in neighbour_mol_deltas.iter() {
                    let current_tile = map.tiles.get(&position);

                    let mut current_mols = 0.0;

                    if let Some(current_tile) = current_tile {
                        current_mols = current_tile
                            .atmosphere
                            .gasses
                            .get(&gas)
                            .unwrap_or(&0.0)
                            .clone();
                    }

                    if current_mols == 0.0 {
                        continue;
                    }

                    let count = *neighbour_count.get(gas).unwrap_or(&0) + 1;

                    

                    let total_mols: f32 = *total_mols_by_gas.get(&gas).unwrap_or(&0.0) + current_mols;

                    let average_mols = total_mols / count as f32;

                    let excess_mols = current_mols - average_mols;

                    if excess_mols == 0.0 {
                        continue;
                    }

                    let total_delta: f32 = neighbour_mol_deltas.values()
                    .flat_map(|deltas: &HashMap<Vector3i, f32>| deltas.values())
                    .sum();

                for (neighbour, delta) in gas_deltas.iter() {
                    let proportion = *delta / total_delta;

                    let mols_to_swap = excess_mols * proportion * (delta / current_mols) * DIFFUSION_SMOOTHING;

                    if mols_to_swap < DISSIPATION_THRESHOLD {
                        continue;
                    }

                    clean_tiles.insert(neighbour, false);

                    //Add gas to neighbouring tile
                    let mut current_tile = None;
                    if let Some(tile) = map.tiles.get(&position) {
                        current_tile = Some(tile.clone());
                    }

                    if let Some(current_tile) = current_tile {
                        if let Some(neighbour_tile) = map.tiles.get_mut(&neighbour) {
                            current_tile.atmosphere.transfer_single_gas(
                                &mut neighbour_tile.atmosphere,
                                *gas,
                                mols_to_swap,
                            );
                        }
                    }
                    //println!("Swap: Gas {} delta {}, Proportion {}, total mols, {}, Swap mols {}", gas, delta, proportion, total_mols, mols_to_swap);
                    //Remove gas from current tile
                    let mut neighbour_tile = None;
                    if let Some(tile) = map.tiles.get(&neighbour) {
                        neighbour_tile = Some(tile.clone());
                    }


                    if let Some(neighbour_tile) = neighbour_tile {
                        if let Some(current_tile) = map.tiles.get_mut(&position) {
                            //neighbour_tile.atmosphere.transfer_gas_except(&mut current_tile.atmosphere, mols_to_swap, *gas);
                            current_tile
                                .atmosphere
                                .remove_single_gas(*gas, mols_to_swap);
                        }
                    }

                    //Remove from neighbour
                    /*if let Some(neighbour_tile) = map.tiles.get_mut(&neighbour) {
                        neighbour_tile.atmosphere.remove_gas_except(mols_to_swap, *gas);
                    }*/
                }
            }
            //Set clean tiles
            for (position, clean) in clean_tiles.iter() {
                if !clean {
                    continue;
                }

                if let Some(tile) = map.tiles.get_mut(&position) {
                    tile.atmosphere.dirty = false;
                }
            }
        }

        /*let mut total_gasses: HashMap<Gas, f32> = HashMap::new();
        for (_, tile) in map.tiles.iter() {
            for (gas, mols) in tile.atmosphere.gasses.iter() {
                *total_gasses.entry(*gas).or_insert(0.0) += mols;
            }     
        }
        for (gas, mols) in total_gasses.iter() {
            println!("{}: {}", gas, mols);
        }*/
    }
}

pub fn get_accessible_neighbours(map: &Map, position: &Vector3i) -> Vec<Vector3i> {
    let mut neighbours = get_neighbours(*position);
    let mut accessible_neighbours = Vec::new();

    while let Some(neighbour) = neighbours.pop() {
        if let Some(tile) = map.tiles.get(&neighbour) {
            if tile.passable {
                accessible_neighbours.push(neighbour);
            }
        }   
    }
    accessible_neighbours
}
