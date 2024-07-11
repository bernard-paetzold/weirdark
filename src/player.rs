use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::Component;
use crate::{vectors::Vector3i, Camera, State};


#[derive(Component, Debug)]
pub struct Player {}

pub fn try_move_player(delta: Vector3i, ecs: &mut World) -> Vector3i {
    let mut positions = ecs.write_storage::<Vector3i>();
    let mut players = ecs.write_storage::<Player>();

    let mut player_position = Vector3i::new_equi(0);

    for (_player, position) in (&mut players, &mut positions).join() {
        position.x = position.x + delta.x;
        position.y = position.y + delta.y;
        position.z = position.z + delta.z;

        player_position = *position;
    }

    player_position
}

pub fn update_camera_position(new_position: Vector3i, ecs: &mut World) {
    let cameras = ecs.read_storage::<Camera>();
    let mut camera_positions = ecs.write_storage::<Vector3i>();

    for (position, camera) in (&mut camera_positions, &cameras).join() {
        if camera.is_active {
            position.x = new_position.x;
            position.y = new_position.y;
            position.z = new_position.z;
        }
    }
}

pub fn player_input(game_state: &mut State, ctx: &mut Rltk) {
    // Player movement

    let mut delta = Vector3i::new_equi(0);

    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Period => delta = Vector3i::new(0, 0, -1),
            VirtualKeyCode::Comma => delta = Vector3i::new(0, 0, 1),
            VirtualKeyCode::Left => delta = Vector3i::new(-1, 0, 0),
            VirtualKeyCode::Right => delta = Vector3i::new(1, 0, 0),
            VirtualKeyCode::Up => delta = Vector3i::new(0, -1, 0),
            VirtualKeyCode::Down => delta = Vector3i::new(0, 1, 0),
            _ => {}
        },
    }

    if delta.x != 0 || delta.y != 0 || delta.z != 0 {
        let new_camera_position = try_move_player(delta, &mut game_state.ecs);

        //TODO: Change this to allow camera freelook
        update_camera_position(new_camera_position, &mut game_state.ecs);
    }
}