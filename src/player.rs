use std::usize;

use rltk::{Rltk, VirtualKeyCode};
use specs::{prelude::*, shred::Fetch, storage::MaskedStorage, world::EntitiesRes};
use specs_derive::Component;
use crate::{Blocker, TERMINAL_WIDTH};
use crate::{gamelog::GameLog, vectors::Vector3i, Camera, Illuminant, Map, Photometry, RunState, State, Viewshed};

use serde::Serialize;
use serde::Deserialize;


#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Player {
}

impl Player {
    pub fn new() -> Player {
        Player { }
    }
}

pub fn try_move_player(delta: Vector3i, ecs: &mut World) -> Option<Vector3i> {
    let mut positions = ecs.write_storage::<Vector3i>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let entities = ecs.entities();
    let mut photometria = ecs.write_storage::<Photometry>();
    let mut illuminants = ecs.write_storage::<Illuminant>();
    let blockers = ecs.read_storage::<Blocker>();

    let mut target_position = Vector3i::new_equi(0);

    //let mut log = ecs.fetch_mut::<GameLog>();

    for (_player, position, _entity) in (&mut players, &mut positions, &entities).join() {
        let map = ecs.fetch::<Map>();
        let tile = map.tiles.get(&(*position + delta));

        //Check tile blockers
        match tile {
            Some(tile) => {
                //TODO: Add exceptions here for if a player might need to move through solid tiles
                if !tile.passable {
                    return None;
                }
            },
            _ => {
                return None
            }
        }

        target_position = *position + delta;
    }

    //Check entity blockers
    if (&blockers, &positions).join().filter(|x| *x.1 == target_position).next().is_some() {
        return None;
    }

    //log.entries.push(target_position.to_string());

    let player = ecs.write_resource::<Entity>();

    //Update player position
    for (_player, position) in (&mut players, &mut positions).join() {
        *position = target_position;
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
    let mut stored_player_position = ecs.write_resource::<Vector3i>();     
    stored_player_position.x = target_position.x;
    stored_player_position.y = target_position.y;
    stored_player_position.z = target_position.z;

    return Some(target_position)
}

pub fn update_camera_position(delta: Vector3i, ecs: &mut World) -> Option<&Camera> {
    let cameras = ecs.read_storage::<Camera>();
    let mut camera_positions = ecs.write_storage::<Vector3i>();

    for (position, camera) in (&mut camera_positions, &cameras).join() {
        if camera.is_active {
            *position += delta;
        }
    }
    None
}

pub fn set_camera_position(new_position: Vector3i, ecs: &mut World) {
    let cameras = ecs.read_storage::<Camera>();
    let mut camera_positions = ecs.write_storage::<Vector3i>();

    for (position, camera) in (&mut camera_positions, &cameras).join() {
        if camera.is_active {
            *position = new_position;
        }
    }
}

pub fn reset_camera_position(ecs: &mut World) {
    let cameras = ecs.read_storage::<Camera>();
    let mut camera_positions = ecs.write_storage::<Vector3i>();

    let player_pos = ecs.fetch::<Vector3i>();

    for (position, camera) in (&mut camera_positions, &cameras).join() {
        if camera.is_active {
            *position = *player_pos;
        }
    }
}

pub fn player_input(game_state: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    let mut delta = Vector3i::new_equi(0);
    let mut delta_camera = Vector3i::new_equi(0);

    let player_pos = *game_state.ecs.fetch::<Vector3i>();
    
    let mut reset_camera = false;

    match ctx.key {
        //If there is no input, set runstate to paused
        None => { return RunState::AwaitingInput }
        Some(key) => match key {
            VirtualKeyCode::Period => delta = Vector3i::new(0, 0, -1),
            VirtualKeyCode::Comma => delta = Vector3i::new(0, 0, 1),
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => delta = Vector3i::new(0, -1, 0),
            VirtualKeyCode::Numpad9 => delta = Vector3i::new(1, -1, 0),
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => delta = Vector3i::new(1, 0, 0),
            VirtualKeyCode::Numpad3 => delta = Vector3i::new(1, 1, 0),
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => delta = Vector3i::new(0, 1, 0),
            VirtualKeyCode::Numpad1 => delta = Vector3i::new(-1, 1, 0),
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => delta = Vector3i::new(-1, 0, 0),
            VirtualKeyCode::Numpad7 => delta = Vector3i::new(-1, -1, 0),
            
            //Pass turn
            
            VirtualKeyCode::Space | VirtualKeyCode::Numpad5 => {
                let game_log = game_state.ecs.fetch_mut::<GameLog>();
                return skip_turn(game_log)
            },

            //Main menu
            VirtualKeyCode::Escape =>  return RunState::SaveGame,

            //Look gui
            VirtualKeyCode::K =>  return RunState::InteractGUI { range: TERMINAL_WIDTH as usize, target: player_pos, source: player_pos },

            //Interaction gui
            VirtualKeyCode::I =>  return RunState::InteractGUI { range: 1, target: player_pos, source: player_pos },

            //Camera freelook
            VirtualKeyCode::Q => delta_camera = Vector3i::new(0, 0, 1),
            VirtualKeyCode::E => delta_camera = Vector3i::new(0, 0, -1),
            VirtualKeyCode::A => delta_camera = Vector3i::new(-1, 0, 0),
            VirtualKeyCode::D => delta_camera = Vector3i::new(1, 0, 0),
            VirtualKeyCode::W => delta_camera = Vector3i::new(0, -1, 0),
            VirtualKeyCode::S => delta_camera = Vector3i::new(0, 1, 0),
            VirtualKeyCode::R => reset_camera = true,
            VirtualKeyCode::L => {
                let mut illuminants = game_state.ecs.write_storage::<Illuminant>();
                let players = game_state.ecs.read_storage::<Player>();
                let mut viewsheds = game_state.ecs.write_storage::<Viewshed>();
                let mut photometria = game_state.ecs.write_storage::<Photometry>();

                for (_player, illuminant, viewshed, photometry) in (&players, &mut illuminants, &mut viewsheds, &mut photometria).join() {
                    illuminant.on = !illuminant.on;
                    illuminant.dirty = true;
                    viewshed.dirty = true;
                    photometry.dirty = true;

                    let mut log = game_state.ecs.fetch_mut::<GameLog>();

                    match illuminant.on {
                        true => {
                            log.entries.push("Light: On".to_string());
                        },
                        false => {
                            log.entries.push("Light: Off".to_string());
                        }
                    }
                }
            },
            //If there is no valid input, set runstate to paused
            _ => { return RunState::AwaitingInput }
        },
    }

    if delta.x != 0 || delta.y != 0 || delta.z != 0 {

        let new_player_position = try_move_player(delta, &mut game_state.ecs);

        match new_player_position {
            Some(_) => {
                //TODO: Change this to prevent camera always moving with player
                update_camera_position(delta, &mut game_state.ecs);
            },
            None => {}
        }
    }

    if reset_camera {
        let new_player_position = try_move_player(Vector3i::new_equi(0), &mut game_state.ecs);

        match new_player_position {
            Some(new_position) => {
                set_camera_position(new_position, &mut game_state.ecs);
            },
            None => {}
        }  
    }
    else if delta_camera.x != 0 || delta_camera.y != 0 || delta_camera.z != 0 {
        update_camera_position(delta_camera, &mut game_state.ecs);
    }
    //Set run state to running
    RunState::PlayerTurn
}

pub fn get_player_entity(entities: &Read<EntitiesRes>, players: &Storage<Player, Fetch<MaskedStorage<Player>>>) -> Option<Entity> {
    (entities, players).join().next().map(|(entity, _)| entity)
}

fn skip_turn(mut game_log: specs::shred::FetchMut<GameLog>) -> RunState {
    //TODO: Add functionality to heal while waiting etc here.
    game_log.entries.push("Waiting...".to_string());
    RunState::PlayerTurn
}