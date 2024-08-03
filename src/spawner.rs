use std::f32::consts::PI;

use rltk::RGB;
use serde::Serialize;
use specs::{prelude::*, saveload::{MarkedBuilder, SimpleMarker}};

use crate::{vectors::Vector3i, Illuminant, Name, Photometry, Player, Renderable, SerializeThis, Viewshed};

pub fn player(ecs: &mut World, player_position: Vector3i) -> Entity {
    //Add player camera
    {
        crate::add_camera(player_position, ecs, true);
    }
    ecs.create_entity()
        .with(player_position)
        .with(Renderable::new(
            rltk::to_cp437('@'),
            rltk::to_cp437('@'),
            RGB::named(rltk::YELLOW).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(1.0),
        ))
        .with(Player::new())
        .with(Viewshed::new(30, 3, 0.9))
        .with(Photometry::new())
        .with(Illuminant::new(
            1.0,
            5,
            RGB::named(rltk::RED).to_rgba(1.0),
            PI * 2.0,
            false,
        ))
        .with(Name::new("Player".to_string()))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}
