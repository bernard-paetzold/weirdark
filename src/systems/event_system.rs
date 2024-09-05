use std::{cmp::Ordering, collections::BinaryHeap};

use specs::{
    prelude::*,
    shred::{Fetch, FetchMut},
    storage::{GenericReadStorage, MaskedStorage},
};

use crate::{
    entities::{
        intents::{
            DropIntent, Initiative, Intent, InteractIntent, Interactable, MoveIntent, OpenIntent,
            PickUpIntent,
        },
        power_components::ControlPanel,
    },
    gamelog::GameLog,
    states::RunState,
    update_camera_position,
    vectors::Vector3i,
    Blocker, Camera, Container, Door, Illuminant, InContainer, Installed, Item, Map, Name,
    Photometry, PowerNode, PowerSwitch, Viewshed,
};

pub struct EventSystem {}

const TIME_PER_TURN: f32 = 1.0;

impl<'a> System<'a> for EventSystem {
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, Map>,
        WriteExpect<'a, RunState>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, Entity>,
        WriteExpect<'a, Vector3i>,
        WriteStorage<'a, InteractIntent>,
        WriteStorage<'a, MoveIntent>,
        WriteStorage<'a, PowerSwitch>,
        WriteStorage<'a, PowerNode>,
        WriteStorage<'a, Initiative>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Door>,
        WriteStorage<'a, Vector3i>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Photometry>,
        WriteStorage<'a, Illuminant>,
        WriteStorage<'a, ControlPanel>,
        ReadStorage<'a, Blocker>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, PickUpIntent>,
        WriteStorage<'a, InContainer>,
        WriteStorage<'a, Container>,
        WriteStorage<'a, OpenIntent>,
        WriteStorage<'a, DropIntent>,
        ReadStorage<'a, Item>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            map,
            mut run_state,
            mut game_log,
            player,
            mut stored_player_position,
            mut interact_intents,
            mut move_intents,
            mut power_switches,
            mut power_nodes,
            initiatives,
            names,
            mut doors,
            mut positions,
            mut viewsheds,
            mut photometria,
            mut illuminants,
            breakers,
            blockers,
            cameras,
            mut pick_up_intents,
            mut in_container,
            mut containers,
            mut open_intents,
            mut drop_intents,
            items,
        ) = data;

        let mut next_state: Option<RunState> = None;
        //Handle movement intents
        let mut entities_to_handle = BinaryHeap::new();

        //Collect all entities in order of intent
        for (entity, initiative) in (&entities, &initiatives).join() {
            entities_to_handle.push(IntentState::new(entity, initiative.current));
        }

        let mut queue_empty = true;

        for intent_state in entities_to_handle.iter() {
            let entity = intent_state.entity;

            let is_player = entity == *player;

            //Handle possible interactions
            {
                if let Some(interact_intent) = interact_intents.get_mut(entity) {
                    interact_intent.update_remaining_cost(-TIME_PER_TURN);

                    macro_rules! handle_interaction_intent {
                        ( $( $x:expr ),* ) => {
                            {
                                $(
                                    if let Some(component) = $x.get_mut(interact_intent.target) {
                                        if component.interaction_id == interact_intent.interaction_id {
                                            component.interact();

                                            if let Some(name) = names.get(interact_intent.target) {
                                                game_log.entries.push(format!("{}: {}", name.name, interact_intent.interaction_description.clone()));
                                            }
                                            else {
                                                game_log.entries.push("Invalid intent".to_string());
                                            }
                                            $x.remove(entity);
                                        }
                                    }

                                    //If the interactable is powered, rebuild power state
                                    if let Some(power_node) = power_nodes.get_mut(interact_intent.target) {
                                        power_node.dirty = true;
                                    }

                                    //If the interactable is a breaker box, rebuild power state of all connected wires
                                    if let Some(_) = breakers.get(interact_intent.target) {
                                        if let Some(breaker_position) = positions.get(interact_intent.target) {
                                            for (power_node, _) in (&mut power_nodes, &positions).join().filter(|(_, x)| *x == breaker_position) {
                                               power_node.dirty = true;
                                            }
                                        }
                                    }

                                    //If the interaction is a container, open its inventory
                                    if let Some(_) = containers.get(interact_intent.target) {

                                    }


                                )*
                            }
                        };
                    }

                    //If the interaction is finished execute it
                    if interact_intent.get_remaining_cost() <= 0.0 {
                        handle_interaction_intent!(power_switches, doors);
                        interact_intents.remove(entity);
                    }

                    if is_player {
                        queue_empty = false;
                    }
                }
            }

            //Handle possible movement
            {
                if let Some(move_intent) = move_intents.get_mut(entity) {
                    move_intent.update_remaining_cost(-TIME_PER_TURN);

                    if move_intent.get_remaining_cost() <= 0.0 {
                        let current_position = move_intent.current_position;
                        let delta = move_intent.delta;
                        let mut target_position = Vector3i::new_equi(0);

                        let mut movement_possible = true;

                        let tile = map.tiles.get(&(current_position + delta));

                        //Check tile blockers
                        match tile {
                            Some(tile) => {
                                //TODO: Add exceptions here for if a player might need to move through solid tiles
                                if !tile.passable {
                                    movement_possible = false;
                                }
                            }
                            _ => {
                                movement_possible = false;
                            }
                        }

                        if movement_possible {
                            target_position = current_position + delta;
                        }

                        //If the movement is diagonal, blocks of four entites must be checked since the player passes through all four
                        if check_entity_blocking(
                            &blockers,
                            &positions,
                            current_position,
                            target_position,
                        ) {
                            movement_possible = false;
                        }

                        if movement_possible {
                            game_log.entries.push(target_position.to_string());

                            //Update position
                            if let Some(new_position) = positions.get_mut(entity) {
                                *new_position = target_position;
                            }

                            if let Some(viewshed) = viewsheds.get_mut(*player) {
                                viewshed.dirty = true;
                            }

                            if let Some(photometry) = photometria.get_mut(*player) {
                                photometry.dirty = true;
                            }

                            if let Some(illuminant) = illuminants.get_mut(*player) {
                                illuminant.dirty = true;
                            }

                            //Update player position tracker
                            *stored_player_position = target_position;

                            update_camera_position(delta, &cameras, &mut positions);
                        }
                        move_intents.remove(entity);
                    }

                    if is_player {
                        queue_empty = false;
                    }
                }
            }

            //Handle pick up events
            {
                if let Some(pick_up_event) = pick_up_intents.get_mut(entity) {
                    let mut pick_up_valid = true;
                    let item = items.get(pick_up_event.target).unwrap();

                    //TODO: Check item weight etc
                    match containers.get(entity) {
                        Some(container) => {
                            if container.remaining_volume - item.volume < 0.0 {
                                //Item is too large
                                pick_up_valid = false;
                                game_log.entries.push("Item is too large".to_string());
                            }
                        }
                        None => {
                            //Entity has no place to put item
                            //TODO: Change this when clothes etc have storage
                            pick_up_valid = false;
                            game_log.entries.push("No place to store item".to_string());
                        }
                    }

                    pick_up_event.update_remaining_cost(-TIME_PER_TURN);

                    if pick_up_event.get_remaining_cost() <= 0.0 && pick_up_valid {
                        let item_entity = pick_up_event.target.clone();
                        let container = containers.get_mut(pick_up_event.initiator);
                        //Add to container

                        if let Some(container) = container {
                            if pick_up_valid {
                                let _ = in_container
                                    .insert(pick_up_event.target, InContainer::new(container.id));

                                //Remove position
                                positions.remove(item_entity);

                                //Remove intent
                                pick_up_intents.remove(entity);

                                //Update container
                                container.try_insert_item(item.volume);
                            }
                        }
                    }

                    if !pick_up_valid {
                        pick_up_intents.remove(entity);
                    }

                    if is_player {
                        queue_empty = false;
                    }
                }
            }

            //Handle drop intents
            {
                if let Some(drop_event) = drop_intents.get_mut(entity) {
                    let item = items.get(drop_event.target).unwrap();

                    drop_event.update_remaining_cost(-TIME_PER_TURN);

                    if drop_event.get_remaining_cost() <= 0.0 {
                        let item_entity = drop_event.target.clone();
                        let container = containers.get_mut(drop_event.initiator);
                        //Add to container

                        if let Some(container) = container {
                            //Add position
                            if let Some(position) = positions.get(drop_event.initiator) {
                                let _ = positions.insert(item_entity, position.clone());
                            }

                            //Remove in container
                            in_container.remove(item_entity);

                            //Remove intent
                            drop_intents.remove(entity);

                            //Update container
                            container.remove_item(item.volume);
                        }
                    }

                    if is_player {
                        queue_empty = false;
                    }
                }
            }

            //Handle open container events
            {
                if let Some(open_event) = open_intents.get_mut(entity) {
                    open_event.update_remaining_cost(-TIME_PER_TURN);

                    if open_event.get_remaining_cost() <= 0.0 {
                        next_state = Some(RunState::ShowInventory {
                            id: open_event.interaction_id,
                            selected_item: None,
                        });
                        //Remove intent
                        open_intents.remove(entity);
                    }

                    if is_player {
                        queue_empty = false;
                    }
                }
            }

            //If all intents are handled, return to input state
            if let Some(state) = next_state.clone() {
                *run_state = state;
            } else if queue_empty {
                *run_state = RunState::AwaitingInput;
            } else {
                *run_state = RunState::Ticking;
            }
        }
    }
}
#[derive(Clone, Copy)]
pub enum InteractionType {
    Component,
    PickUp,
    Open,
    Drop,
}

#[derive(Clone)]
pub struct InteractionInformation {
    pub id: u32,
    pub description: String,
    pub entity_id: u32,
    pub cost: f32,
    pub interaction_type: InteractionType,
}

impl InteractionInformation {
    pub fn new(
        interaction_id: u32,
        interaction_description: String,
        entity_id: u32,
        interaction_cost: f32,
        interaction_type: InteractionType,
    ) -> Self {
        Self {
            id: interaction_id,
            description: interaction_description,
            entity_id,
            cost: interaction_cost,
            interaction_type,
        }
    }
}

pub fn get_entity_interactions(ecs: &World, entity: Entity) -> Vec<InteractionInformation> {
    let names = ecs.read_storage::<Name>();

    let mut interactables = Vec::new();
    let mut name = "{unknown}".to_string();

    if let Some(entity_name) = names.get(entity) {
        name = entity_name.name.clone()
    }

    macro_rules! check_for_interactable {
        ($($typ:ty), *) => {
            {
                $(
                    let storage = ecs.read_storage::<$typ>();

                    if let Some(interactable) = storage.get(entity) {
                        interactables.push(InteractionInformation::new(
                            interactable.interaction_id,
                            format!("{} ({}): {}", name, interactable.state_description(), interactable.interaction_description),
                            entity.id(),
                            interactable.get_cost(),
                            InteractionType::Component,
                        ));
                    }
                )*
            }
        };
    }

    //TODO: Add any other interactable components
    check_for_interactable!(PowerSwitch, Door);

    interactables
}

pub fn get_default_interactions(ecs: &World, entity: Entity) -> Vec<InteractionInformation> {
    let mut interactions = Vec::new();
    let installed = ecs.read_storage::<Installed>();
    let containers = ecs.read_storage::<Container>();
    let in_containers = ecs.read_storage::<InContainer>();

    //Item pickup
    {
        //If the item is not installed, allow pickup
        if installed.get(entity).is_none() {
            interactions.push(InteractionInformation::new(
                entity.id(),
                format!("{}", "Pick up".to_string()),
                entity.id(),
                1.0,
                InteractionType::PickUp,
            ));
        }
    }

    //If item is in a container, give an option to drop it
    {
        if in_containers.get(entity).is_some() {
            interactions.push(InteractionInformation::new(
                entity.id(),
                format!("{}", "Drop".to_string()),
                entity.id(),
                1.0,
                InteractionType::Drop,
            ));
        }
    }

    //Open container
    {
        if let Some(container) = containers.get(entity) {
            interactions.push(InteractionInformation::new(
                container.id,
                format!("{}", "Open".to_string()),
                entity.id(),
                1.0,
                InteractionType::Open,
            ));
        }
    }

    interactions
}

//#[derive(Serialize, Deserialize, Clone)]
struct IntentState {
    entity: Entity,
    initiative: f32,
}

impl PartialEq for IntentState {
    fn eq(&self, other: &Self) -> bool {
        self.initiative == other.initiative
    }
}

impl Eq for IntentState {}

impl IntentState {
    fn new(entity: Entity, initiative: f32) -> Self {
        Self { entity, initiative }
    }
}

impl Ord for IntentState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.initiative
            .partial_cmp(&other.initiative)
            .unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for IntentState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn check_entity_blocking(
    blockers: &Storage<Blocker, Fetch<MaskedStorage<Blocker>>>,
    positions: &Storage<Vector3i, FetchMut<MaskedStorage<Vector3i>>>,
    player_position: Vector3i,
    target_position: Vector3i,
) -> bool {
    let delta = (target_position - player_position).normalize_delta();

    if is_entity_blocked(&blockers, &positions, player_position, target_position) {
        return true;
    }

    if delta == Vector3i::NW {
        //Check two additional tiles player moves through
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position + Vector3i::N,
            target_position,
        ) {
            return true;
        }
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position + Vector3i::W,
            target_position,
        ) {
            return true;
        }

        if is_entity_blocked(
            &blockers,
            &positions,
            player_position,
            player_position + Vector3i::N,
        ) {
            return true;
        }
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position,
            player_position + Vector3i::W,
        ) {
            return true;
        }
    } else if delta == Vector3i::SW {
        //Check two additional tiles player moves through
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position + Vector3i::S,
            target_position,
        ) {
            return true;
        }
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position + Vector3i::W,
            target_position,
        ) {
            return true;
        }

        if is_entity_blocked(
            &blockers,
            &positions,
            player_position,
            player_position + Vector3i::S,
        ) {
            return true;
        }
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position,
            player_position + Vector3i::W,
        ) {
            return true;
        }
    } else if delta == Vector3i::SE {
        //Check two additional tiles player moves through
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position + Vector3i::S,
            target_position,
        ) {
            return true;
        }
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position + Vector3i::E,
            target_position,
        ) {
            return true;
        }

        if is_entity_blocked(
            &blockers,
            &positions,
            player_position,
            player_position + Vector3i::S,
        ) {
            return true;
        }
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position,
            player_position + Vector3i::E,
        ) {
            return true;
        }
    } else if delta == Vector3i::NE {
        //Check two additional tiles player moves through
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position + Vector3i::N,
            target_position,
        ) {
            return true;
        }
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position + Vector3i::E,
            target_position,
        ) {
            return true;
        }

        if is_entity_blocked(
            &blockers,
            &positions,
            player_position,
            player_position + Vector3i::N,
        ) {
            return true;
        }
        if is_entity_blocked(
            &blockers,
            &positions,
            player_position,
            player_position + Vector3i::E,
        ) {
            return true;
        }
    }
    false
}

fn is_entity_blocked(
    blockers: &Storage<Blocker, Fetch<MaskedStorage<Blocker>>>,
    positions: &Storage<Vector3i, FetchMut<MaskedStorage<Vector3i>>>,
    player_position: Vector3i,
    target_position: Vector3i,
) -> bool {
    //Check tile entity is in
    for (blocker, _) in (blockers, positions)
        .join()
        .filter(|x| *x.1 == player_position)
    {
        let delta = (target_position - player_position).normalize_delta();

        if delta == Vector3i::N && blocker.sides.contains(&crate::Direction::N) {
            return true;
        } else if delta == Vector3i::NW
            && (blocker.sides.contains(&crate::Direction::N)
                || blocker.sides.contains(&crate::Direction::W))
        {
            return true;
        } else if delta == Vector3i::W && blocker.sides.contains(&crate::Direction::W) {
            return true;
        } else if delta == Vector3i::SW
            && (blocker.sides.contains(&crate::Direction::S)
                || blocker.sides.contains(&crate::Direction::W))
        {
            return true;
        } else if delta == Vector3i::S && blocker.sides.contains(&crate::Direction::S) {
            return true;
        } else if delta == Vector3i::SE
            && (blocker.sides.contains(&crate::Direction::S)
                || blocker.sides.contains(&crate::Direction::E))
        {
            return true;
        } else if delta == Vector3i::E && blocker.sides.contains(&crate::Direction::E) {
            return true;
        } else if delta == Vector3i::NE
            && (blocker.sides.contains(&crate::Direction::N)
                || blocker.sides.contains(&crate::Direction::E))
        {
            return true;
        } else if delta == Vector3i::UP && blocker.sides.contains(&crate::Direction::UP) {
            return true;
        } else if delta == Vector3i::DOWN && blocker.sides.contains(&crate::Direction::DOWN) {
            return true;
        }
    }

    //Check tile entity is going to
    for (blocker, _) in (blockers, positions)
        .join()
        .filter(|x| *x.1 == target_position)
    {
        let delta = (player_position - target_position).normalize_delta();

        if delta == Vector3i::N && blocker.sides.contains(&crate::Direction::N) {
            return true;
        } else if delta == Vector3i::NW
            && (blocker.sides.contains(&crate::Direction::N)
                || blocker.sides.contains(&crate::Direction::W))
        {
            return true;
        } else if delta == Vector3i::W && blocker.sides.contains(&crate::Direction::W) {
            return true;
        } else if delta == Vector3i::SW
            && (blocker.sides.contains(&crate::Direction::S)
                || blocker.sides.contains(&crate::Direction::W))
        {
            return true;
        } else if delta == Vector3i::S && blocker.sides.contains(&crate::Direction::S) {
            return true;
        } else if delta == Vector3i::SE
            && (blocker.sides.contains(&crate::Direction::S)
                || blocker.sides.contains(&crate::Direction::E))
        {
            return true;
        } else if delta == Vector3i::E && blocker.sides.contains(&crate::Direction::E) {
            return true;
        } else if delta == Vector3i::NE
            && (blocker.sides.contains(&crate::Direction::N)
                || blocker.sides.contains(&crate::Direction::E))
        {
            return true;
        } else if delta == Vector3i::UP && blocker.sides.contains(&crate::Direction::UP) {
            return true;
        } else if delta == Vector3i::DOWN && blocker.sides.contains(&crate::Direction::DOWN) {
            return true;
        }
    }
    return false;
}
