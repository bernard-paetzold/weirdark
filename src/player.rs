use std::usize;

use crate::entities::biology::Breather;
use crate::entities::intents::{InteractIntent, MoveIntent};
use crate::graphics::get_viewport_position;
use crate::systems::event_system::InteractionInformation;
use crate::{
    gamelog::GameLog, vectors::Vector3i, Illuminant, Photometry, RunState, State, Viewshed,
};
use crate::{
    mouse_to_map, set_camera_position, update_camera_position, Camera, Container, TERMINAL_WIDTH,
};
use rltk::{Rltk, VirtualKeyCode};
use specs::storage::GenericReadStorage;
use specs::{prelude::*, shred::Fetch, storage::MaskedStorage, world::EntitiesRes};
use specs_derive::Component;

use serde::Deserialize;
use serde::Serialize;

#[derive(Component, Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub power_overlay: bool,
    pub gas_overlay: bool,
}

impl Player {
    pub fn new() -> Player {
        Player {
            power_overlay: false,
            gas_overlay: false,
        }
    }
}

pub fn try_move_player(delta: Vector3i, ecs: &mut World) {
    let mut move_intents = ecs.write_storage::<MoveIntent>();
    let player = ecs.write_resource::<Entity>();
    let player_position = ecs.write_resource::<Vector3i>();

    move_intents
        .insert(*player, MoveIntent::new(*player_position, delta))
        .expect("Player move intent error");
}

pub fn player_input(game_state: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    let mut delta = Vector3i::new_equi(0);
    let mut delta_camera = Vector3i::new_equi(0);

    let viewport_position = get_viewport_position(&game_state.ecs);

    let player_pos = *game_state.ecs.fetch::<Vector3i>();
    let player_entity = *game_state.ecs.fetch::<Entity>();

    let mut reset_camera = false;

    match ctx.key {
        //If there is no input, set runstate to paused
        None => return RunState::AwaitingInput,
        Some(key) => match key {
            VirtualKeyCode::Period => delta = Vector3i::DOWN,
            VirtualKeyCode::Comma => delta = Vector3i::UP,
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => delta = Vector3i::N,
            VirtualKeyCode::Numpad9 => delta = Vector3i::NE,
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => delta = Vector3i::E,
            VirtualKeyCode::Numpad3 => delta = Vector3i::SE,
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => delta = Vector3i::S,
            VirtualKeyCode::Numpad1 => delta = Vector3i::SW,
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => delta = Vector3i::W,
            VirtualKeyCode::Numpad7 => delta = Vector3i::NW,

            //Pass turn
            VirtualKeyCode::Space | VirtualKeyCode::Numpad5 => {
                let game_log = game_state.ecs.fetch_mut::<GameLog>();
                return skip_turn(game_log);
            }

            VirtualKeyCode::B => {
                let mut breathers = game_state.ecs.write_storage::<Breather>();
                let players = game_state.ecs.read_storage::<Player>();

                for (_player, breather) in (&players, &mut breathers).join() {
                    breather.trigger_breath = true;
                }
            }

            //Look gui
            VirtualKeyCode::K => {
                return RunState::InteractGUI {
                    range: TERMINAL_WIDTH as usize,
                    target: player_pos,
                    source: player_pos,
                    prev_mouse_position: mouse_to_map(ctx.mouse_pos(), viewport_position),
                    selected_entity: None,
                }
            }

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

                for (_player, illuminant, viewshed, photometry) in
                    (&players, &mut illuminants, &mut viewsheds, &mut photometria).join()
                {
                    illuminant.on = !illuminant.on;
                    illuminant.dirty = true;
                    viewshed.dirty = true;
                    photometry.dirty = true;

                    let mut log = game_state.ecs.fetch_mut::<GameLog>();

                    match illuminant.on {
                        true => {
                            log.entries.push("Light: On".to_string());
                        }
                        false => {
                            log.entries.push("Light: Off".to_string());
                        }
                    }
                }
            }
            //If there is no valid input, set runstate to paused
            _ => {
                return RunState::HandleOtherInput {
                    next_runstate: std::sync::Arc::new(RunState::AwaitingInput),
                    key,
                }
            }
        },
    }

    if delta.x != 0 || delta.y != 0 || delta.z != 0 {
        try_move_player(delta, &mut game_state.ecs);
    }

    if reset_camera {
        let player_position;
        {
            player_position = *game_state.ecs.read_resource::<Vector3i>();
        }

        set_camera_position(player_position, &mut game_state.ecs);
    } else if delta_camera.x != 0 || delta_camera.y != 0 || delta_camera.z != 0 {
        let cameras = game_state.ecs.read_storage::<Camera>();
        let mut positions = game_state.ecs.write_storage::<Vector3i>();

        update_camera_position(delta_camera, &cameras, &mut positions);
    }
    //Set run state to player turn
    RunState::Ticking
}

pub fn get_player_entity(
    entities: &Read<EntitiesRes>,
    players: &Storage<Player, Fetch<MaskedStorage<Player>>>,
) -> Option<Entity> {
    (entities, players).join().next().map(|(entity, _)| entity)
}

fn skip_turn(mut game_log: specs::shred::FetchMut<GameLog>) -> RunState {
    //TODO: Add functionality to heal while waiting etc here.
    game_log.entries.push("Waiting...".to_string());
    RunState::Ticking
}

pub fn toggle_power_overlay(ecs: &mut World) {
    let mut players = ecs.write_storage::<Player>();
    let player_positions = ecs.read_storage::<Vector3i>();

    for (_, player) in (&player_positions, &mut players).join() {
        player.power_overlay = !player.power_overlay;
    }
}

pub fn toggle_gas_overlay(ecs: &mut World) {
    let mut players = ecs.write_storage::<Player>();
    let player_positions = ecs.read_storage::<Vector3i>();

    for (_, player) in (&player_positions, &mut players).join() {
        player.gas_overlay = !player.gas_overlay;
    }
}

pub fn handle_other_input(
    ecs: &mut World,
    key: VirtualKeyCode,
    sending_state: RunState,
) -> RunState {
    let container_id = {
        let player = ecs.fetch::<Entity>();
        let containers = ecs.read_storage::<Container>();
        containers.get(*player).map(|c| c.id)
    };

    if let Some(id) = container_id {
        match key {
            //Main menu
            VirtualKeyCode::Escape => return RunState::SaveGame,
            VirtualKeyCode::I => {
                return RunState::ShowInventory {
                    id,
                    selected_item: None,
                }
            }

            //Enable power overlay
            VirtualKeyCode::P => {
                toggle_power_overlay(ecs);
                return sending_state;
            }
            VirtualKeyCode::G => {
                toggle_gas_overlay(ecs);
                return sending_state;
            }
            _ => return sending_state,
        }
    }
    sending_state
}
