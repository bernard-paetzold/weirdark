use std::f32::consts::PI;

use rltk::{RGB, RGBA};
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

pub fn standing_lamp(ecs: &mut World, name: String, light_position: Vector3i, intensity: f32, color: RGBA) -> Entity {
    ecs.create_entity()
        .with(light_position)
        .with(Renderable::new(
            rltk::to_cp437('î'),
            rltk::to_cp437('î'),
            RGB::named(rltk::ANTIQUEWHITE4).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Viewshed::new(30, 3, 1.0))
        .with(Photometry::new())
        .with(Illuminant::new(
            intensity,
            10,
            color,
            PI * 2.0,
            true,
        ))
        .with(Name::new(name.to_string()))
        .with(Power::new(true, true))
        .with(PowerSwitch::new(false))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub fn ceiling_lamp(ecs: &mut World, name: String, light_position: Vector3i, intensity: f32, color: RGBA) -> Entity {
    ecs.create_entity()
        .with(light_position)
        .with(Renderable::new(
            rltk::to_cp437('☼'),
            rltk::to_cp437('☼'),
            RGB::named(rltk::ANTIQUEWHITE4).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Viewshed::new(30, 3, 1.0))
        .with(Photometry::new())
        .with(Illuminant::new(
            intensity,
            30,
            color,
            PI * 2.0,
            true,
        ))
        .with(Name::new(name.to_string()))
        .with(Power::new(true, true))
        .with(PowerSwitch::new(true))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}
