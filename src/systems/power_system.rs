use std::collections::HashSet;

use specs::{join, prelude::*};

use crate::{
    vectors::{utils::{get_cardinal_neighbours, get_cardinal_neighbours_with_z}, Vector3i}, Illuminant, Map, Photometry, PowerNode, PowerSource, PowerSwitch, PoweredState, Wire
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
        WriteStorage<'a, PowerNode>,
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
            mut nodes,
            entities
        ) = data;

        let mut dirty_networks = HashSet::new();
        for (node, _) in (&mut nodes, &entities).join().filter(|(node, _)| node.dirty) {
            dirty_networks.insert(node.network_id);
            //Reset dirty
            node.dirty = false;
        }

        for network_id in dirty_networks.iter() {
            //Rebuild wire network
            if let Some((start_wire, node, start_position)) = (&mut wires, &nodes, &positions).join()
            .filter(|(_, node, _)| node.network_id == *network_id).next() {

                let mut unchanged_wires = vec!(*start_position);
                let mut visited_positons = HashSet::new();

                while let Some(wire_position) = unchanged_wires.pop() {

                    for (wire, current_position) in (&mut wires, &positions).join()
                    .filter(|(_, x)| *x == &wire_position) {
                        visited_positons.insert(current_position);
                    }

                    let neighbours = get_cardinal_neighbours_with_z(wire_position);

                    for neighbour in neighbours.into_iter() {
                        if (&wires, &positions).join().any(|(_, x)| *x == neighbour) && 
                        !visited_positons.contains(&neighbour) {
                            unchanged_wires.push(neighbour);
                        }
                    }
                }

                //Get all nodes found in the network
                for (_, node) in (&positions, &mut nodes).join()
                    .filter(|(x, _)| visited_positons.contains(x)) {
                        node.network_id = *network_id;
                }
            }

            //Align powered on state with switches
            for (power_switch, entity, _) in (&power_switches, &entities, &nodes).join()
            .filter(|(_, _, node)| node.network_id == *network_id) {
                if let Some(power_state) = power_states.get_mut(entity) {
                    power_state.on = power_switch.on;
                }

                if let Some(power_source) = power_sources.get_mut(entity) {
                    power_source.on = power_switch.on;
                }
            }

            //Calculate power draw of wire segments
            //Reset power draw
            for (power_source, _) in (&mut power_sources, &nodes).join()
            .filter(|(_, node)| node.network_id == *network_id) {
                power_source.available_wattage = power_source.max_wattage;
            }

            for wire in (&mut wires).join() {
                wire.power_load = 0.0;
                wire.available_wattage = 0.0;
            }

            //For every power state, check if they have a wire attached to them
            for (power_state, position, node) in (&mut power_states, &positions, &nodes).join()
            .filter(|(_, _, node)| node.network_id == *network_id) {
                for (wire, _) in (&mut wires, &positions).join()
                .filter(|(_, x)| *x == position || *x == &(*position + Vector3i::UP))  {
                    if power_state.on {
                        wire.power_load += power_state.wattage;
                    }
                }
            }

            //Align powered state with power sources
            for (power_source, position, _) in (&mut power_sources, &positions, &nodes).join()
            .filter(|(_, _, node)| node.network_id == *network_id) {
                if power_source.on {
                    if (&wires, &positions).join().any(|(_, x)| x == position) {
                        let mut unchanged_wires = vec!(*position);
                        let mut visited_positons = HashSet::new();
                        let mut total_draw:f32 = 0.0;

                        while let Some(wire_position) = unchanged_wires.pop() {

                            for (wire, current_position) in (&mut wires, &positions).join()
                            .filter(|(_, x)| *x == &wire_position) {
                                total_draw += wire.power_load;
                                visited_positons.insert(current_position);
                            }
        
                            let neighbours = get_cardinal_neighbours_with_z(wire_position);
        
                            for neighbour in neighbours.into_iter() {
                                if (&wires, &positions).join().any(|(_, x)| *x == neighbour) && 
                                !visited_positons.contains(&neighbour) {
                                    unchanged_wires.push(neighbour);
                                }
                            }
                        }
                        power_source.available_wattage = power_source.available_wattage - total_draw;

                        for (wire, _) in (&mut wires, &positions).join()
                            .filter(|(_, x)| visited_positons.contains(x)) {
                                wire.available_wattage = power_source.available_wattage;
                                wire.power_load = total_draw;
                        }
                    }
                }
            }

            for (power_state, position, _) in (&mut power_states, &positions, &nodes).join()
            .filter(|(_, _, node)| node.network_id == *network_id) {
                for (wire, _) in (&mut wires, &positions).join()
                .filter(|(_, x)| *x == position || *x == &(*position + Vector3i::UP))  {
                    power_state.available_wattage = wire.available_wattage;
                }
            }

            //Align powered components with powered state
            for (power, entity, _) in (&mut power_states, &entities, &nodes).join()
            .filter(|(_, _, node)| node.network_id == *network_id) {

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
}
