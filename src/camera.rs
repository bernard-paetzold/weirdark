use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

use crate::{vectors::Vector3i, Renderable};

#[derive(Component, Debug)]
pub struct Camera {
    pub is_active: bool,
}

pub fn add_camera(position: Vector3i, ecs: &mut World, is_active: bool) {
    ecs.create_entity()
                .with(position)
                .with(Camera { is_active })
                /*.with(Renderable::new(
                rltk::to_cp437('K'),
                rltk::to_cp437('K'),
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK)))*/
                .build();
            }
