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

pub fn add_camera(position: Vector3i, ecs: &mut World, is_active: bool) {
    ecs.create_entity()
                .with(position)
                .with(Camera { is_active })
                /*.with(Renderable::new(
                rltk::char_to_glyph('K'),
                rltk::char_to_glyph('K'),
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK)))*/
                .marked::<SimpleMarker<SerializeThis>>()
                .build();
            }
