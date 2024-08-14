use specs::prelude::*;
use specs_derive::{Component, ConvertSaveload};
use specs::error::NoError;
use specs::saveload::{ConvertSaveload, MarkedBuilder, Marker, SimpleMarker};
use serde::Serialize;
use serde::Deserialize;

use crate::vectors::Vector3i;
use crate::SerializeThis;

#[derive(Component, Debug, ConvertSaveload, Clone)]
pub struct Camera {
    pub is_active: bool,
}

impl Camera {
    pub fn new(is_active: bool) -> Self {
        Self {
            is_active,
        }
    }
}

pub fn add_camera(position: Vector3i, ecs: &mut World, is_active: bool) {
    ecs.create_entity()
                .with(position)
                .with(Camera::new(is_active))
                /*.with(Renderable::new(
                rltk::char_to_glyph('K'),
                rltk::char_to_glyph('K'),
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK)))*/
                .marked::<SimpleMarker<SerializeThis>>()
                .build();
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

pub fn set_camera_z(new_z: i32, ecs: &mut World) {
    let cameras = ecs.read_storage::<Camera>();
    let mut camera_positions = ecs.write_storage::<Vector3i>();

    for (position, camera) in (&mut camera_positions, &cameras).join() {
        if camera.is_active {
            position.z = new_z;
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
