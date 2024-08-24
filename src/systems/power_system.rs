use std::collections::{HashMap, HashSet};

use fnv::FnvHashSet;
use rltk::RandomNumberGenerator;
use specs::{
    prelude::*,
    storage::{GenericReadStorage, GenericWriteStorage},
};

use crate::{
    entities::{
        atmospherics::R,
        power_components::{BreakerBox, ElectronicHeater},
    },
    vectors::{utils::get_cardinal_neighbours_with_z, Vector3i},
    Illuminant, Map, Photometry, PowerNode, PowerSource, PowerSwitch, PoweredState, Wire,
};

use crate::entities::intents::Interactable;

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
        WriteStorage<'a, ElectronicHeater>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            mut power_states,
            mut power_sources,
            power_switches,
            mut illuminants,
            mut photometrics,
            positions,
            mut wires,
            mut nodes,
            breaker_boxes,
            mut electronic_heaters,
            entities,
        ) = data;

        let mut random = RandomNumberGenerator::new();

        //Align powered on state with switches
        for (power_switch, entity, _) in (&power_switches, &entities, &nodes).join() {
            if let Some(power_state) = power_states.get_mut(entity) {
                power_state.on = power_switch.on;
            }

            if let Some(power_source) = power_sources.get_mut(entity) {
                power_source.on = power_switch.on;
            }
        }

        //Check if heaters must turn on
        /* for (electronic_heater, power_state, position, node) in (
            &mut electronic_heaters,
            &mut power_states,
            &positions,
            &mut nodes,
        )
            .join()
        {
            if let Some(tile) = map.tiles.get_mut(position) {
                let on = electronic_heater.check_status(tile.atmosphere.temperature);

                let prev_state = power_state.on;

                if on {
                    //Produce heat

                    power_state.on = true;
                    tile.atmosphere.update_temperature(
                        (2.0 * power_state.wattage) / (3.0 * tile.atmosphere.get_total_mols() * R),
                    );
                } else {
                    power_state.on = false;
                }

                if power_state.on != prev_state {
                    node.dirty = true;
                }
            }
        }*/
        let dirty_networks: HashSet<usize> = (&mut nodes, &entities)
            .join()
            .filter_map(|(node, _)| {
                if node.dirty {
                    node.dirty = false;
                    Some(node.network_id)
                } else {
                    None
                }
            })
            .collect();

        for network_id in dirty_networks.iter() {
            //Rebuild wire network
            let mut start_position = Vector3i::new_equi(0);
            let mut start_wire = Wire::new(
                rltk::RGB::named(rltk::WHITE).to_rgba(1.0),
                "invalid".to_string(),
                false,
            );

            {
                if let Some((wire, _, position)) = (&wires, &nodes, &positions)
                    .join()
                    .filter(|(_, node, _)| node.network_id == *network_id)
                    .next()
                {
                    start_position = *position;
                    start_wire = wire.clone();
                }
            }

            if start_wire.color_name == "invalid" {
                continue;
            }

            let mut unchanged_wires = vec![start_position];

            let mut load_wires = std::collections::HashMap::new();
            let mut prev_colors = HashSet::new();
            prev_colors.insert(start_wire.color_name);

            let mut network_wires = FnvHashSet::default();

            while let Some(wire_position) = unchanged_wires.pop() {
                //For every node, set the id
                let mut is_load_wire = false;
                for (position, node, entity) in (&positions, &mut nodes, &entities)
                    .join()
                    .filter(|(x, _, _)| **x == wire_position)
                {
                    node.network_id = *network_id;

                    if let Some(power_state) = power_states.get(entity) {
                        is_load_wire = true;
                    }
                }

                //Add all wires on the current position
                for (wire, current_position, entity) in (&mut wires, &positions, &entities)
                    .join()
                    .filter(|(wire, x, _)| {
                        *x == &wire_position && prev_colors.contains(&wire.color_name.clone())
                    })
                {
                    network_wires.insert(entity.id());

                    if is_load_wire {
                        load_wires.insert(entity, wire_position);
                    }
                }

                let neighbours = get_cardinal_neighbours_with_z(wire_position);

                for neighbour in neighbours.into_iter() {
                    for (wire, position, _, entity) in (&wires, &positions, &nodes, &entities)
                        .join()
                        .filter(|(_, x, _, _)| **x == neighbour)
                    {
                        if !network_wires.contains(&entity.id()) {
                            if let Some((_, switch, _)) =
                                (&breaker_boxes, &power_switches, &positions)
                                    .join()
                                    .filter(|(_, _, x)| *x == position)
                                    .next()
                            {
                                // if switch.on {
                                unchanged_wires.push(neighbour);
                                prev_colors.insert(wire.color_name.clone());
                                //}
                            } else if prev_colors.contains(&wire.color_name) {
                                unchanged_wires.push(neighbour);
                            }
                        }
                    }
                }
            }

            //Get all wires no longer in the network
            for (_, _, node) in (&wires, &entities, &mut nodes)
                .join()
                .filter(|(_, x, node)| {
                    node.network_id == *network_id && !network_wires.contains(&x.id())
                })
            {
                node.network_id = random.rand();
                node.dirty = true;
            }

            //Calculate power draw of wire segments
            //Reset power draw
            for (power_source, _) in (&mut power_sources, &nodes)
                .join()
                .filter(|(_, node)| node.network_id == *network_id)
            {
                power_source.available_wattage = power_source.max_wattage;
            }

            for wire in network_wires.iter() {
                if let Some(wire) = wires.get_mut(entities.entity(*wire)) {
                    wire.power_load = 0.0;
                    wire.available_wattage = 0.0;
                }
            }

            //let mut visited_power_wires = HashSet::new();
            let now = std::time::Instant::now();
            //Calculate power loads

            for (start_wire_entity, position) in load_wires.iter() {
                if let Some(start_wire) = wires.get(*start_wire_entity).cloned() {
                    let mut unchanged_wires = vec![*position];
                    let mut wire_entities: FnvHashSet<u32> = HashSet::default();

                    let mut total_draw = 0.0;

                    let mut prev_colors = HashSet::new();
                    prev_colors.insert(start_wire.color_name.clone());

                    while let Some(wire_position) = unchanged_wires.pop() {
                        for (current_wire, current_position, entity) in
                            (&wires, &positions, &entities)
                                .join()
                                .filter(|(wire, x, entity)| {
                                    **x == wire_position && prev_colors.contains(&wire.color_name)
                                })
                        {
                            if !wire_entities.contains(&entity.id()) {
                                for (power_state, position, node) in
                                    (&mut power_states, &positions, &mut nodes)
                                        .join()
                                        .filter(|(_, x, node)| *x == current_position)
                                {
                                    total_draw += power_state.wattage;
                                    node.network_id = *network_id;
                                }
                                wire_entities.insert(entity.id());
                            }
                        }

                        let neighbours = get_cardinal_neighbours_with_z(wire_position);

                        for neighbour in neighbours.into_iter() {
                            for (wire, position, entity) in (&wires, &positions, &entities)
                                .join()
                                .filter(|(_, x, _)| **x == neighbour)
                            {
                                if !wire_entities.contains(&entity.id()) {
                                    if let Some((_, switch, _)) =
                                        (&breaker_boxes, &power_switches, &positions)
                                            .join()
                                            .filter(|(_, _, x)| *x == position)
                                            .next()
                                    {
                                        if switch.on {
                                            unchanged_wires.push(neighbour);
                                            prev_colors.insert(wire.color_name.clone());
                                        }
                                    } else if prev_colors.contains(&wire.color_name) {
                                        unchanged_wires.push(neighbour);
                                    }
                                }
                            }
                        }
                    }

                    for (wire, _, _) in (&mut wires, &positions, &entities)
                        .join()
                        .filter(|(wire, _, entity)| wire_entities.contains(&entity.id()))
                    {
                        wire.power_load = total_draw;
                    }
                }
            }
            println!("{:?}", now.elapsed());

            //let mut visited_power_wires = HashSet::new();

            //Align powered state with power sources
            for (power_source, position, _) in (&mut power_sources, &positions, &nodes)
                .join()
                .filter(|(_, _, node)| node.network_id == *network_id)
            {
                if power_source.on {
                    if let Some((start_wire, position, entity)) = (&wires, &positions, &entities)
                        .join()
                        .filter(|(_, x, _)| *x == position)
                        .next()
                    {
                        let mut unchanged_wires = vec![*position];
                        let mut wire_entities = HashSet::new();

                        let mut prev_colors = HashSet::new();
                        prev_colors.insert(start_wire.color_name.clone());

                        while let Some(wire_position) = unchanged_wires.pop() {
                            for (current_wire, current_position, entity) in (
                                &wires, &positions, &entities,
                            )
                                .join()
                                .filter(|(wire, x, entity)| {
                                    **x == wire_position && prev_colors.contains(&wire.color_name)
                                })
                            {
                                if !wire_entities.contains(&entity) {
                                    wire_entities.insert(entity);
                                }
                            }

                            let neighbours = get_cardinal_neighbours_with_z(wire_position);

                            for neighbour in neighbours.into_iter() {
                                for (wire, position, entity) in (&wires, &positions, &entities)
                                    .join()
                                    .filter(|(_, x, _)| **x == neighbour)
                                {
                                    if !wire_entities.contains(&entity) {
                                        if let Some((_, switch, _)) =
                                            (&breaker_boxes, &power_switches, &positions)
                                                .join()
                                                .filter(|(_, _, x)| *x == position)
                                                .next()
                                        {
                                            if switch.on {
                                                unchanged_wires.push(neighbour);
                                                prev_colors.insert(wire.color_name.clone());
                                            }
                                        } else if prev_colors.contains(&wire.color_name) {
                                            unchanged_wires.push(neighbour);
                                        }
                                    }
                                }
                            }
                        }
                        power_source.available_wattage =
                            power_source.available_wattage - start_wire.power_load;

                        for (wire, _) in (&mut wires, &entities)
                            .join()
                            .filter(|(wire, x)| wire_entities.contains(x))
                        {
                            wire.available_wattage += power_source.available_wattage;
                        }
                    }
                }
            }

            for (power_state, position, node) in (&mut power_states, &positions, &mut nodes)
                .join()
                .filter(|(_, _, node)| node.network_id == *network_id)
            {
                for (wire, _) in (&mut wires, &positions)
                    .join()
                    .filter(|(_, x)| *x == position)
                {
                    power_state.available_wattage = wire.available_wattage;
                }
            }

            //Align powered components with powered state
            for (power, entity, _, _) in (&mut power_states, &entities, &nodes, &positions)
                .join()
                .filter(|(_, _, node, _)| node.network_id == *network_id)
            {
                let power_state = power.on && (power.available_wattage > 0.0);
                //Illuminant
                if let Some(illuminant) = illuminants.get_mut(entity) {
                    illuminant.set_state(power_state);

                    if let Some(photometry) = photometrics.get_mut(entity) {
                        photometry.dirty = true;
                    }
                }

                if let Some(electronic_heater) = electronic_heaters.get_mut(entity) {
                    electronic_heater.set_state(power_state);
                }
                //TODO: Add any other powered systems here
            }
        }
    }
}

/*pub fn get_devices_on_network(
    ecs: &World,
    network_entity: Entity,
) -> Vec<(usize, String, u32, u32, f32)> {
    let names = ecs.read_storage::<crate::Name>();
    let nodes = ecs.read_storage::<crate::PowerNode>();
    let entities = ecs.entities();

    let mut interactables = Vec::new();

    if let Some(network_node) = nodes.get(network_entity) {
        for (entity, _) in (&entities, &nodes).join().filter(|(entity, node)| {
            node.network_id == network_node.network_id && *entity != network_entity
        }) {
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
                                    entity.id(),
                                    interactable.get_cost()
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
}*/

pub fn get_devices_on_subnetwork(
    ecs: &World,
    network_entity: Entity,
) -> Vec<(usize, String, u32, u32, f32)> {
    let names = ecs.read_storage::<crate::Name>();
    let nodes = ecs.read_storage::<crate::PowerNode>();
    let positions = ecs.read_storage::<crate::Vector3i>();
    let wires = ecs.read_storage::<crate::Wire>();
    let entities = ecs.entities();

    let mut interactables = Vec::new();

    let mut start_position = Vector3i::new_equi(0);

    if let Some(position) = positions.get(network_entity) {
        start_position = *position;
    }
    let mut unchanged_wires = vec![start_position];

    let mut start_wire = Wire::new(
        rltk::RGB::named(rltk::WHITE).to_rgba(1.0),
        "invalid".to_string(),
        false,
    );

    for (wire, _, node) in (&wires, &positions, &nodes)
        .join()
        .filter(|(_, x, _)| **x == start_position)
    {
        start_wire = wire.clone();
    }

    let mut prev_colors = HashSet::new();
    prev_colors.insert(start_wire.color_name);

    let mut network_wires = FnvHashSet::default();

    while let Some(wire_position) = unchanged_wires.pop() {
        //Add all wires on the current position
        for (_, _, entity) in (&wires, &positions, &entities)
            .join()
            .filter(|(wire, x, _)| {
                *x == &wire_position && prev_colors.contains(&wire.color_name.clone())
            })
        {
            network_wires.insert(entity.id());
        }

        let neighbours = get_cardinal_neighbours_with_z(wire_position);

        for neighbour in neighbours.into_iter() {
            for (wire, _, _, entity) in (&wires, &positions, &nodes, &entities)
                .join()
                .filter(|(wire, x, _, _)| wire.data && **x == neighbour)
            {
                if !network_wires.contains(&entity.id()) {
                    if prev_colors.contains(&wire.color_name) {
                        unchanged_wires.push(neighbour);
                    }
                }
            }
        }
    }
    for entity_id in network_wires {
        let entity = entities.entity(entity_id);
        let mut current_position = Vector3i::new_equi(0);

        if let Some(position) = positions.get(entity) {
            current_position = position.clone();
        }

        for (entity, _) in (&entities, &positions)
            .join()
            .filter(|(entity, x)| **x == current_position && *entity != network_entity)
        {
            if let Some(_) = nodes.get(entity) {
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
                                            entity.id(),
                                            interactable.get_cost()
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
    }
    interactables
}
