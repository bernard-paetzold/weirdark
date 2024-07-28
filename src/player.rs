use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;
use crate::{vectors::Vector3i, Camera, Map, State, Viewshed};


#[derive(Component, Debug)]
pub struct Player {}

pub fn try_move_player(delta: Vector3i, ecs: &mut World) -> Option<Vector3i> {
    let mut positions = ecs.write_storage::<Vector3i>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();

    for (_player, position, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let map = ecs.fetch::<Map>();

        let tile = map.tiles.get(&(*position + delta));

        match tile {
            Some(tile) => {
                if tile.passable {
                    //TODO: Add exceptions here for if a player might need to move through solid tiles
                    *position += delta;
                    let new_position = *position;
                    println!("Player position: {}", position);
                    viewshed.dirty = true;

                    return Some(new_position)
                }
            },
            _ => {
                //TODO: This allows players to move where there is no tile
                *position += delta;
                    let new_position = *position;
                    println!("Player position: {}", position);
                    viewshed.dirty = true;

                    return Some(new_position)
            }
        }
    }
    None
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

pub fn set_camera_position(delta: Vector3i, ecs: &mut World) {
    let cameras = ecs.read_storage::<Camera>();
    let mut camera_positions = ecs.write_storage::<Vector3i>();

    for (position, camera) in (&mut camera_positions, &cameras).join() {
        if camera.is_active {
            *position = delta;
        }
    }
}

pub fn player_input(game_state: &mut State, ctx: &mut Rltk) {
    // Player movement

    let mut delta = Vector3i::new_equi(0);
    let mut delta_camera = Vector3i::new_equi(0);

    let mut reset_camera = false;

    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Period => delta = Vector3i::new(0, 0, -1),
            VirtualKeyCode::Comma => delta = Vector3i::new(0, 0, 1),
            VirtualKeyCode::Left => delta = Vector3i::new(-1, 0, 0),
            VirtualKeyCode::Right => delta = Vector3i::new(1, 0, 0),
            VirtualKeyCode::Up => delta = Vector3i::new(0, -1, 0),
            VirtualKeyCode::Down => delta = Vector3i::new(0, 1, 0),

            //Camera freelook
            VirtualKeyCode::Q => delta_camera = Vector3i::new(0, 0, -1),
            VirtualKeyCode::E => delta_camera = Vector3i::new(0, 0, 1),
            VirtualKeyCode::A => delta_camera = Vector3i::new(-1, 0, 0),
            VirtualKeyCode::D => delta_camera = Vector3i::new(1, 0, 0),
            VirtualKeyCode::W => delta_camera = Vector3i::new(0, -1, 0),
            VirtualKeyCode::S => delta_camera = Vector3i::new(0, 1, 0),
            VirtualKeyCode::R => reset_camera = true,
            _ => {}
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
}

