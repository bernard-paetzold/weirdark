use std::f32::consts::PI;

use rltk::{to_cp437, RGB, RGBA};
use specs::{prelude::*, saveload::{MarkedBuilder, SimpleMarker}};

use crate::{pathfinding::find_walkable_path, vectors::Vector3i, Blocker, Direction, Door, Illuminant, Map, Name, Photometry, Player, PowerSource, PowerSwitch, PoweredState, Renderable, SerializeThis, Viewshed, VisionBlocker, Wire};

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

pub fn standing_lamp(ecs: &mut World, name: String, position: Vector3i, intensity: f32, color: RGBA, on: bool) -> Entity {
    ecs.create_entity()
        .with(position)
        .with(Renderable::new(
            rltk::to_cp437('î'),
            rltk::to_cp437('î'),
            RGB::named(rltk::ANTIQUEWHITE4).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Viewshed::new(10, 3, 1.0))
        .with(Photometry::new())
        .with(Illuminant::new(
            intensity,
            10,
            color,
            PI * 2.0,
            false,
        ))
        .with(Name::new(name.to_string()))
        .with(PoweredState::new(true, 10.0))
        .with(PowerSwitch::new(on))
        .with(Blocker::new())
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub fn ceiling_lamp(ecs: &mut World, name: String, position: Vector3i, intensity: f32, color: RGBA, on: bool) -> Entity {
    ecs.create_entity()
        .with(position)
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
            false,
        ))
        .with(Name::new(name.to_string()))
        .with(PoweredState::new(true, 10.0))
        .with(PowerSwitch::new(on))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub fn door(ecs: &mut World, name: String, position: Vector3i, open: bool, color: RGBA, open_glyph: rltk::FontCharType, closed_glyph: rltk::FontCharType) -> Entity {
    if open {
        ecs.create_entity()
        .with(position)
        .with(Door::new(open, open_glyph, closed_glyph))
        .with(Renderable::new(
            open_glyph,
            open_glyph,
            color,
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Photometry::new())
        .with(Name::new(name.to_string()))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
    }
    else {
        ecs.create_entity()
        .with(position)
        .with(Door::new(open, open_glyph, closed_glyph))
        .with(Renderable::new(
            closed_glyph,
            closed_glyph,
            color,
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Photometry::new())
        .with(Name::new(name.to_string()))
        .with(Blocker::new())
        .with(VisionBlocker::new())
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
    }
}

pub fn power_source(ecs: &mut World, position: Vector3i, on: bool, power: f32) {
    ecs.create_entity()
    .with(position)
    .with(Renderable::new(
        to_cp437('◘'),
        to_cp437('◘'),
        RGB::named(rltk::WHITE).to_rgba(1.0),
        RGB::named(rltk::BLACK).to_rgba(0.0),
    ))
    .with(Photometry::new())
    .with(Name::new("Power source".to_string()))
    .with(PowerSource::new(on, power))
    .with(PowerSwitch::new(true))
    .marked::<SimpleMarker<SerializeThis>>()
    .build();
}

pub fn lay_wire(ecs: &mut World, map: Map, start_position: Vector3i, end_position: Vector3i) {
    let path = find_walkable_path(ecs, map, start_position, end_position);
    let mut prev_position = start_position;

    for position in path.iter() {
        let vec_direction = *position - prev_position;
        let mut direction = Direction::NW;
        let mut char = '.';

        
        if vec_direction == Vector3i::TOP {
            direction = Direction::N;
            char = '|';
        }
        else if vec_direction == Vector3i::LEFT {
            direction = Direction::W;
            char = '-';
        }
        else if vec_direction == Vector3i::BOTTOM {
            direction = Direction::S;
            char = '|';
        }
        else if vec_direction == Vector3i::RIGHT {
            direction = Direction::E;
            char = '-';
        }


        ecs.create_entity()
        .with(*position)
        .with(Renderable::new(
            to_cp437(char),
            to_cp437(char),
            RGB::named(rltk::RED).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Photometry::new())
        .with(Name::new("Wire".to_string()))
        .with(PoweredState::new(true, 0.0))
        .with(Wire::new(direction))
        .marked::<SimpleMarker<SerializeThis>>()
        .build();

        prev_position = *position;
    }

}