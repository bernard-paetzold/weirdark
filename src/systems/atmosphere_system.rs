use std::collections::{HashMap, HashSet};

use specs::prelude::*;

use crate::{
    entities::atmospherics::{Gas, R},
    vectors::{
        utils::{get_neighbours, get_neighbours_with_z},
        Vector3i,
    },
    Map,
};

const DISSIPATION_THRESHOLD: f32 = 0.00000001;

pub struct AtmosphereSystem {}

impl<'a> System<'a> for AtmosphereSystem {
    type SystemData = (WriteExpect<'a, Map>,);

    fn run(&mut self, data: Self::SystemData) {
        let now = std::time::Instant::now();
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
            let neighbours = get_accessible_neighbours(&map, position).clone();

            let current_tile = map.tiles.get(position);
            let mut temperature = 0.0;
            let mut pressure = 0.0;

            let mut neighbour_mol_deltas: HashMap<Vector3i, HashMap<Gas, f32>> = HashMap::new();
            let mut total_gas_deltas: HashMap<Gas, f32> = HashMap::new();

            let mut neighbour_pressure_deltas: HashMap<Vector3i, f32> = HashMap::new();
            let mut total_delta = 0.0;
            let mut total_pressure = 0.0;

            let mut clean_tiles = Vec::new();

            let mut higher_pressure_neighbours = Vec::new();

            if let Some(current_tile) = current_tile {
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
                                                neighbour_mol_deltas
                                                    .entry(*neighbour)
                                                    .or_insert_with(HashMap::new)
                                                    .insert(*gas, delta);
            
                                            if delta > DISSIPATION_THRESHOLD {
                                                *total_gas_deltas.entry(*gas).or_insert(0.0) += delta;
                                            }
                                            else {
                                                clean_tiles.push(neighbour);
                                            }
                                        }
                                    }
                                    else if *mols > DISSIPATION_THRESHOLD {
                                        neighbour_mol_deltas
                                            .entry(*neighbour)
                                            .or_insert_with(HashMap::new)
                                            .insert(*gas, *mols);
    
                                        *total_gas_deltas.entry(*gas).or_insert(0.0) += *mols;                 
                                    }
                                    else {
                                        clean_tiles.push(neighbour);
                                    }
                                }
                            }
                            else {
                                let delta = pressure - neighbour_pressure;
                                neighbour_pressure_deltas.insert(*neighbour, delta);
                                total_delta += delta;
                                total_pressure += neighbour_pressure;
                            }
                        } else {
                            higher_pressure_neighbours.push(neighbour);
                        }

                        /*for (gas, mols) in current_tile.atmosphere.gasses.iter() {
                            if let Some(neighbour_mols) = neighbour_tile.atmosphere.gasses.get(gas)
                            {
                                let delta = mols - neighbour_mols;
                                    neighbour_mol_deltas
                                        .entry(*neighbour)
                                        .or_insert_with(HashMap::new)
                                        .insert(*gas, delta);

                                    *total_gas_deltas.entry(*gas).or_insert(0.0) += delta;
                            }
                        }*/
                    }
                }
            }

            let average_pressure =
                (total_pressure + pressure) / (neighbour_pressure_deltas.len() + 1) as f32;
            let excess_pressure = pressure - average_pressure;
            //Distribute excess between neighbours proportionally to the difference from own
            for (neighbour, delta) in neighbour_pressure_deltas.iter() {
                let proportion: f32 = *delta / total_delta;
                let mols_to_transfer = (excess_pressure * proportion) / (temperature * R);

                if mols_to_transfer < DISSIPATION_THRESHOLD {
                    clean_tiles.push(neighbour);
                    continue;
                }

                //Add gas to neighbouring tile
                let mut current_tile = None;
                if let Some(tile) = map.tiles.get(&position) {
                    current_tile = Some(tile.clone());
                }

                if let Some(current_tile) = current_tile {
                    if let Some(neighbour_tile) = map.tiles.get_mut(&neighbour) {
                        current_tile
                            .atmosphere
                            .transfer_gas(&mut neighbour_tile.atmosphere, mols_to_transfer);
                        //neighbour_tile.atmosphere.dirty = true;
                    }
                }

                //Remove gas from current tile
                if let Some(current_tile) = map.tiles.get_mut(&position) {
                    current_tile.atmosphere.remove_gas(mols_to_transfer);
                    //current_tile.atmosphere.dirty = true;
                }
            }

            //If neighbours are missing gasses current tile has, share them
            for (neighbour, gas_deltas) in neighbour_mol_deltas.iter() {
                for (gas, delta) in gas_deltas.iter() {
                    let proportion: f32;

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
                        clean_tiles.push(neighbour);
                        continue;
                    }

                    let mut total_mols: f32 = neighbour_mol_deltas.values()
                    .filter_map(|deltas| deltas.get(gas))
                    .sum::<f32>() + current_mols;

                    total_mols += current_mols;

                    let count = neighbour_mol_deltas.values()
                    .filter(|deltas| deltas.contains_key(gas))
                    .count() + 1;

                    let average_mols = total_mols / count as f32;

                    let excess_mols = (current_mols - average_mols).max(0.0);

                    if excess_mols == 0.0 {
                        clean_tiles.push(neighbour);
                        continue;
                    }

                    let total_delta: f32 = neighbour_mol_deltas.values()
                        .filter_map(|deltas| deltas.get(gas))
                        .sum();


                    proportion = *delta / total_delta;

                    let mols_to_swap = excess_mols * proportion;

                    if mols_to_swap < DISSIPATION_THRESHOLD {
                        clean_tiles.push(neighbour);
                        continue;
                    }

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
                            //neighbour_tile.atmosphere.dirty = true;
                        }
                    }

                    //Remove gas from current tile
                    if let Some(current_tile) = map.tiles.get_mut(&position) {
                        current_tile
                            .atmosphere
                            .remove_single_gas(*gas, mols_to_swap);
                        current_tile.atmosphere.dirty = true;
                    }
                }
            }

            //Set clean tiles
            for position in clean_tiles.iter() {
                if let Some(tile) = map.tiles.get_mut(&position) {
                    tile.atmosphere.dirty = false;
                }
            }
        }
    }
}

pub fn get_accessible_neighbours(map: &Map, position: &Vector3i) -> Vec<Vector3i> {
    let mut neighbours = get_neighbours(*position);
    let mut accessible_neighbours = Vec::new();

    while neighbours.len() > 0 {
        let neighbour = neighbours.pop();

        if let Some(tile_position) = neighbour {
            if let Some(tile) = map.tiles.get(&tile_position) {
                if tile.passable {
                    accessible_neighbours.push(tile_position);
                }
            }
        }
    }
    accessible_neighbours
}
