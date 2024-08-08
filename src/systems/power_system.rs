use std::collections::{HashMap, HashSet};

use specs::{prelude::*, rayon::iter::Positions};

use crate::{
    vectors::{utils::get_cardinal_neighbours, Vector3i}, Illuminant, Map, Photometry, Power, PowerSource, PowerSwitch, PoweredState, Wire
};

pub struct PowerSystem {}

impl<'a> System<'a> for PowerSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, PoweredState>,
        WriteStorage<'a, PowerSource>,
        ReadStorage<'a, PowerSwitch>,
        WriteStorage<'a, Illuminant>,
        WriteStorage<'a, Photometry>,
        ReadStorage<'a, Vector3i>,
        WriteStorage<'a, Wire>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            _map,
            mut power_states,
            mut power_sources,
            power_switches,
            mut illuminants,
            mut photometrics,
            positions,
            mut wires,
            entities
        ) = data;

        //Align powered on state with switches
        for (power_switch, entity) in (&power_switches, &entities).join() {
            if let Some(power_state) = power_states.get_mut(entity) {
                power_state.on = power_switch.on;
            }

            if let Some(power_source) = power_sources.get_mut(entity) {
                power_source.on = power_switch.on;
            }
        }

        //Calculate power draw of wire segments
        //Reset power draw
        for power_source in (&mut power_sources).join() {
            power_source.available_wattage = power_source.max_wattage;
        }

        for wire in (&mut wires).join() {
            wire.power_load = 0.0;
            wire.available_wattage = 0.0;
        }

        //For every power state, check if they have a wire attached to them
        for (power_state, position) in (&mut power_states, &positions).join() {
            for (wire, _) in (&mut wires, &positions).join()
            .filter(|(_, x)| *x == position || *x == &(*position + Vector3i::TOP))  {
                if power_state.on {
                    wire.power_load += power_state.wattage;
                }
            }
        }

        //Align powered state with power sources
        for (power_source, position) in (&mut power_sources, &positions).join() {
            if power_source.on {
                if (&wires, &positions).join().any(|(_, x)| x == position) {
                    let mut unchanged_wires = vec!(*position);
                    let mut visited_positons = HashSet::new();
    
                    while let Some(wire_position) = unchanged_wires.pop() {
                        for (wire, current_position) in (&mut wires, &positions).join().filter(|(_, x)| *x == &wire_position) {
    
                            if power_source.available_wattage - wire.power_load > 0.0 {
                                power_source.available_wattage = power_source.available_wattage - wire.power_load;
                                wire.available_wattage = power_source.available_wattage;
                            }
                            visited_positons.insert(current_position);
                        }
    
                        let neighbours = get_cardinal_neighbours(wire_position);
    
                        for neighbour in neighbours.into_iter() {
                            if (&wires, &positions).join().any(|(_, x)| *x == neighbour) && !visited_positons.contains(&neighbour) {
                                unchanged_wires.push(neighbour);
                            }
                        }
                    }
                }
            }
        }

        for (power_state, position) in (&mut power_states, &positions).join() {
            for (wire, _) in (&mut wires, &positions).join()
            .filter(|(_, x)| *x == position || *x == &(*position + Vector3i::TOP))  {
                power_state.available_wattage = wire.available_wattage;
            }
        }

        //Align powered components with powered state
        for (power, entity) in (&mut power_states, &entities).join() {

            let power_state = power.on && (power.available_wattage >= power.wattage);
            //Illuminant
            if let Some(illuminant) = illuminants.get_mut(entity) {
                illuminant.set_state(power_state);

                if let Some(photometry) = photometrics.get_mut(entity) {
                    photometry.dirty = true;
                }
            }

            //TODO: Add any other powered systems here
        }
    }
}
