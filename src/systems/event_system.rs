use std::{cmp::Ordering, collections::BinaryHeap};

use specs::{
    prelude::*,
    shred::{Fetch, FetchMut},
    storage::MaskedStorage,
};

use crate::{
    entities::{
        intents::{Initiative, Intent, InteractIntent, Interactable, MoveIntent},
        power_components::BreakerBox,
    },
    gamelog::GameLog,
    states::RunState,
    update_camera_position,
    vectors::Vector3i,
    Blocker, Camera, Door, Illuminant, Map, Name, Photometry, PowerNode, PowerSwitch, Viewshed,
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
        WriteStorage<'a, BreakerBox>,
        ReadStorage<'a, Blocker>,
        ReadStorage<'a, Camera>,
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
        ) = data;

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
                                        if component.interaction_id == component.interaction_id {
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


                                )*
                            }
                        };
                    }

                    //If the interaction is finished execute it
                    if interact_intent.get_remaining_cost() <= 0.0 {
                        handle_interaction_intent!(power_switches, doors);
                        interact_intents.remove(entity);
                    } else if is_player {
                        queue_empty = false;
                    }
                }
            }

            //Handle possible movement
            {
                //let mut cleared_intents = Vec::new();

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
                    } else if is_player {
                        queue_empty = false;
                    }
                }
            }
        }

        //If all intents ae handled, return to input state
        if queue_empty {
            *run_state = RunState::AwaitingInput;
        }
    }
}

pub fn get_entity_interactions(ecs: &World, entity: Entity) -> Vec<(usize, String, u32, u32, f32)> {
    let names = ecs.read_storage::<Name>();

    let mut interactables = Vec::new();

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
                            entity.id(),
                            entity.id(),
                            interactable.get_cost()
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

fn check_entity_blocking(
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
