use std::collections::HashSet;

use rltk::RandomNumberGenerator;
use specs::prelude::*;

use crate::{
    entities::power_components::BreakerBox, vectors::{utils::get_cardinal_neighbours_with_z, Vector3i}, Illuminant, Map, Photometry, PowerNode, PowerSource, PowerSwitch, PoweredState, Wire
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
        WriteStorage<'a, BreakerBox>,
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
            breaker_boxes,
            entities
        ) = data;

        let mut random = RandomNumberGenerator::new();

        let mut dirty_networks = HashSet::new();
        for (node, _) in (&mut nodes, &entities).join().filter(|(node, _)| node.dirty) {
            dirty_networks.insert(node.network_id);
            //Reset dirty
            node.dirty = false;
        }

        for network_id in dirty_networks.iter() {
            //Rebuild wire network
            let mut start_position = Vector3i::new_equi(0);
            let mut start_wire = Wire::new(rltk::RGB::named(rltk::WHITE).to_rgba(1.0), "invalid".to_string());

            {
                if let Some((wire, _, position)) = (&wires, &nodes, &positions).join()
                .filter(|(_, node, _)| node.network_id == *network_id).next() {
                    start_position = *position;
                    start_wire = wire.clone();
                }
            }

            if start_wire.color_name == "invalid" {
                continue;
            }

            let mut unchanged_wires = vec!(start_position);

            let mut visited_wire_positions = HashSet::new();

            while let Some(wire_position) = unchanged_wires.pop() {

                //Add all wires on the current position
                for (_, current_position) in (&mut wires, &positions).join()
                .filter(|(_, x)| *x == &wire_position) {
                    visited_wire_positions.insert(current_position);
                }

                let neighbours = get_cardinal_neighbours_with_z(wire_position);

                for neighbour in neighbours.into_iter() {
                    if !visited_wire_positions.contains(&neighbour) {
                        for (_, _, _) in (&wires, &positions, &nodes).join().filter(|(_, x, _)| **x == neighbour) {                   
                            unchanged_wires.push(neighbour);
                        }    
                    }
                }
                
            }

            //Set all wires found in the network
            for (_, _, node) in (&wires, &positions, &mut nodes).join()
                .filter(|(_, x, _)| visited_wire_positions.contains(x)) {
                    node.network_id = *network_id;
            }

            //Set the network id of every connected node to that of the network
            for (_, node, entity) in (&positions, &mut nodes, &entities).join()
            .filter(|(x, _, _)| visited_wire_positions.contains(x)) {
                if let None = wires.get(entity) {
                    node.network_id = *network_id;
                }
            }


            //Get all wires no longer in the network
            for (_, _, node) in (&wires, &positions, &mut nodes).join()
            .filter(|(_, x, node)| node.network_id == *network_id && !visited_wire_positions.contains(*x)) {
                node.network_id = random.rand();
                node.dirty = true;
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

            for (wire, _) in (&mut wires, &nodes).join()
            .filter(|(_, node)| node.network_id == *network_id) {
                wire.power_load = 0.0;
                wire.available_wattage = 0.0;
            }

            let mut load_wires = std::collections::HashMap::new();

            //For every power state, check if they have a wire attached to them
            for (power_state, position, _) in (&mut power_states, &positions, &nodes).join()
            .filter(|(_, _, node)| node.network_id == *network_id) {
                for (wire, _, entity) in (&mut wires, &positions, &entities).join()
                .filter(|(_, x, _)| *x == position)  {
                    if power_state.on {
                        wire.power_load += power_state.wattage;
                        load_wires.insert(entity, position);
                    }
                }
            }

            let mut visited_power_wires = HashSet::new();
            //Calculate power loads
            for (start_wire_entity, position) in load_wires.iter() {                
                if let Some(start_wire) = wires.get(*start_wire_entity).cloned() {
                    let mut unchanged_wires = vec!(**position);
                    let mut wire_entites = vec!(*start_wire_entity);
                
                    let mut total_draw = 0.0;

                    let mut prev_colors = HashSet::new();
                    prev_colors.insert(start_wire.color_name.clone());
            
                    while let Some(wire_position) = unchanged_wires.pop() {
                        for (current_wire, current_position, _) in (&wires, &positions, &entities).join()
                        .filter(|(wire, x, entity)| **x == wire_position && prev_colors.contains(&wire.color_name)
                        && wire_entites.contains(entity)) {
                            total_draw += current_wire.power_load;
                            visited_power_wires.insert(current_position);
                        }
        
                        let neighbours = get_cardinal_neighbours_with_z(wire_position);
        
                        for neighbour in neighbours.into_iter() {
                            if !visited_power_wires.contains(&neighbour) {
    
                                for (wire, position, entity) in (&wires, &positions, &entities).join().filter(|(_, x, _)| **x == neighbour) {
                                    if let Some((_, switch, _)) = (&breaker_boxes, &power_switches, &positions).join().filter(|(_, _, x)| *x == position).next() {
                                        if switch.on {
                                            unchanged_wires.push(neighbour);
                                            wire_entites.push(entity);
                                            prev_colors.insert(wire.color_name.clone());
                                        }
                                    }
                                    else if prev_colors.contains(&wire.color_name) {
                                        unchanged_wires.push(neighbour);
                                        wire_entites.push(entity);
                                        prev_colors.insert(wire.color_name.clone());
                                    }
                                }    
                            }
                        }                        
                    }
    
                    for (wire, _) in (&mut wires, &positions).join()
                        .filter(|(wire, x)| visited_power_wires.contains(x) && prev_colors.contains(&wire.color_name)) {
                        wire.power_load = total_draw;
                    }
                }
               
            }

            let mut visited_power_wires = HashSet::new();

            //Align powered state with power sources
            for (power_source, position, _) in (&mut power_sources, &positions, &nodes).join()
            .filter(|(_, _, node)| node.network_id == *network_id) {
                if power_source.on {
                    if let Some((start_wire, position, entity)) = (&wires, &positions, &entities).join().filter(|(_, x, _)| *x == position).next() {
                        let mut unchanged_wires = vec!(*position);
                        let mut wire_entites = vec!(entity);

                        let mut prev_colors = HashSet::new();
                        prev_colors.insert(start_wire.color_name.clone());
                
                        while let Some(wire_position) = unchanged_wires.pop() {
                            for (_, current_position, _) in (&wires, &positions, &entities).join()
                            .filter(|(wire, x, entity)| **x == wire_position && prev_colors.contains(&wire.color_name)
                            && wire_entites.contains(entity)) {
                                visited_power_wires.insert(current_position);
                            }
            
                            let neighbours = get_cardinal_neighbours_with_z(wire_position);
            
                            for neighbour in neighbours.into_iter() {
                                for (wire, position, entity) in (&wires, &positions, &entities).join().filter(|(_, x, _)| **x == neighbour) {
                                    if !visited_power_wires.contains(&neighbour) || !wire_entites.contains(&entity) {
                                        if let Some((_, switch, _)) = (&breaker_boxes, &power_switches, &positions).join().filter(|(_, _, x)| *x == position).next() {
                                            if switch.on {
                                                unchanged_wires.push(neighbour);
                                                wire_entites.push(entity);
                                                prev_colors.insert(wire.color_name.clone());
                                            }
                                        }
                                        else if prev_colors.contains(&wire.color_name) {
                                            unchanged_wires.push(neighbour);
                                            wire_entites.push(entity);
                                            prev_colors.insert(wire.color_name.clone());
                                        }
                                    }    
                                }
                            }                                    
                        }
                        power_source.available_wattage = power_source.available_wattage - start_wire.power_load;

                        for (wire, _) in (&mut wires, &positions).join()
                        .filter(|(wire, x)| visited_power_wires.contains(x) && prev_colors.contains(&wire.color_name)) {
                            wire.available_wattage += power_source.available_wattage;
                        }
                    }
                }
            }

            for (power_state, position, _) in (&mut power_states, &positions, &nodes).join()
            .filter(|(_, _, node)| node.network_id == *network_id) {
                for (wire, _) in (&mut wires, &positions).join()
                .filter(|(_, x)| *x == position || *x == position)  {
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

pub fn get_devices_on_network(ecs: &World, network_entity: Entity) -> Vec<(usize, String, u32, u32)> {
    let names = ecs.read_storage::<crate::Name>();
    let nodes = ecs.read_storage::<crate::PowerNode>();
    let entities = ecs.entities();

    let mut interactables = Vec::new();

    if let Some(network_node) = nodes.get(network_entity) {
        for (entity, _) in (&entities, &nodes)
        .join().filter(|(entity, node)| node.network_id == network_node.network_id && *entity != network_entity) {
            macro_rules! check_for_interactable {
                ($($typ:ty), *) => {
                    {
                        $(
                            let storage = ecs.read_storage::<$typ>();
        
                            if let Some(interactable) = storage.get(entity) {
                                let mut name = "{unknown}".to_string();
                        
                                if let Some(entity_name) = names.get(entity) { name = entity_name.name.clone()}
                        
                                interactables.push((
                                    interactable.interaction_id,
                                    format!("{} ({}): {}", name, interactable.state_description(), interactable.interaction_description),
                                    network_entity.id(),
                                    entity.id()
                                ));
                            }
                        )*
                    }
                };
            }
        
            //TODO: Add any other interactable components
            check_for_interactable!(PowerSwitch);
        }
    }
    interactables
}
