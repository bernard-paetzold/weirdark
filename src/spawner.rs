use std::f32::consts::PI;

use rltk::RGB;
use specs::{prelude::*, saveload::{MarkedBuilder, SimpleMarker}};

use crate::{vectors::Vector3i, Illuminant, Name, Photometry, Player, Power, PowerSwitch, Renderable, SerializeThis, Viewshed};

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
            10,
            RGB::named(rltk::WHITE).to_rgba(1.0),
            PI * 2.0,
            false,
        ))
        .with(Name::new("Player".to_string()))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub fn standing_lamp(ecs: &mut World, light_position: Vector3i) -> Entity {
    ecs.create_entity()
        .with(light_position)
        .with(Renderable::new(
            rltk::to_cp437('☼'),
            rltk::to_cp437('î'),
            RGB::named(rltk::ANTIQUEWHITE4).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Viewshed::new(20, 3, 1.0))
        .with(Photometry::new())
        .with(Illuminant::new(
            1.0,
            20,
            RGB::named(rltk::ANTIQUEWHITE1).to_rgba(1.0),
            PI * 2.0,
            true,
        ))
        .with(Name::new("Standing lamp".to_string()))
        .with(Power::new(true, false))
        .with(PowerSwitch::new(false))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}
