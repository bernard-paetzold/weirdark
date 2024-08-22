use std::f32::consts::PI;

use rltk::{RGB, RGBA};
use specs::{
    prelude::*,
    saveload::{MarkedBuilder, SimpleMarker},
};

use crate::{
    entities::{
        biology::Breather,
        intents::Initiative,
        power_components::{BreakerBox, ElectronicHeater},
    },
    graphics::char_to_glyph,
    pathfinding::{find_walkable_path, wall_climb_path},
    vectors::Vector3i,
    Blocker, Direction, Door, Duct, EntityDirection, Illuminant, Map, Name, Photometry, Player,
    PowerNode, PowerSource, PowerSwitch, PoweredState, Renderable, SerializeThis, Viewshed,
    VisionBlocker, Wire,
};

pub fn player(ecs: &mut World, player_position: Vector3i) -> Entity {
    //Add player camera
    {
        crate::add_camera(player_position, ecs, true);
    }
    ecs.create_entity()
        .with(player_position)
        .with(Renderable::new(
            char_to_glyph('@'),
            char_to_glyph('@'),
            RGB::named(rltk::YELLOW).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(1.0),
        ))
        .with(Player::new())
        .with(Viewshed::new(60, 3, 0.9))
        .with(Photometry::new())
        .with(Illuminant::new(
            1.0,
            10,
            RGB::named(rltk::WHITE).to_rgba(1.0),
            PI * 2.0,
            false,
        ))
        .with(Name::new("Player".to_string()))
        .with(Breather::new_humanlike())
        .with(Initiative::new(0.0))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

#[allow(dead_code)]
pub fn standing_lamp(
    ecs: &mut World,
    name: String,
    position: Vector3i,
    intensity: f32,
    color: RGBA,
    on: bool,
) -> Entity {
    ecs.create_entity()
        .with(position)
        .with(Renderable::new(
            char_to_glyph('î'),
            char_to_glyph('î'),
            RGB::named(rltk::ANTIQUEWHITE4).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Viewshed::new(10, 3, 1.0))
        .with(Photometry::new())
        .with(Illuminant::new(intensity, 10, color, PI * 2.0, false))
        .with(Name::new(name.to_string()))
        .with(PoweredState::new(true, 10.0))
        .with(PowerSwitch::new(on))
        .with(PowerNode::new())
        .with(Blocker::new_all_sides())
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub const CEILING_LAMP_RANGE: usize = 30;

pub fn ceiling_lamp(
    ecs: &mut World,
    position: Vector3i,
    intensity: f32,
    color: RGBA,
    on: bool,
) -> Entity {
    ecs.create_entity()
        .with(position)
        .with(Renderable::new(
            char_to_glyph('☼'),
            char_to_glyph('☼'),
            RGB::named(rltk::ANTIQUEWHITE4).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Viewshed::new(CEILING_LAMP_RANGE, 3, 1.0))
        .with(Photometry::new())
        .with(Illuminant::new(
            intensity,
            CEILING_LAMP_RANGE,
            color,
            PI * 2.0,
            false,
        ))
        .with(Name::new("Ceiling lamp".to_string()))
        .with(PoweredState::new(true, 10.0))
        .with(PowerSwitch::new(on))
        .with(PowerNode::new())
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub fn door(
    ecs: &mut World,
    position: Vector3i,
    open: bool,
    color: RGBA,
    open_glyph: u16,
    closed_glyph: u16,
) -> Entity {
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
            .with(Name::new("Door".to_string()))
            .marked::<SimpleMarker<SerializeThis>>()
            .build()
    } else {
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
            .with(Name::new("Door".to_string()))
            .with(Blocker::new_all_sides())
            .with(VisionBlocker::new_all_sides())
            .marked::<SimpleMarker<SerializeThis>>()
            .build()
    }
}

pub fn power_source(ecs: &mut World, position: Vector3i, on: bool, power: f32) {
    ecs.create_entity()
        .with(position)
        .with(Renderable::new(
            char_to_glyph('◘'),
            char_to_glyph('◘'),
            RGB::named(rltk::WHITE).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Photometry::new())
        .with(Name::new("Power source".to_string()))
        .with(PowerSource::new(on, power))
        .with(PowerSwitch::new(true))
        .with(PowerNode::new())
        .marked::<SimpleMarker<SerializeThis>>()
        .build();
}

#[allow(dead_code)]
pub fn lay_ducting(ecs: &mut World, map: Map, start_position: Vector3i, end_position: Vector3i) {
    let mut path: Vec<Vector3i> = Vec::new();
    if start_position.z == end_position.z {
        path = find_walkable_path(map, start_position, end_position);
    }

    ecs.create_entity()
        .with(start_position + Vector3i::new(-1, 0, 0))
        .with(Renderable::new(
            char_to_glyph('═'),
            char_to_glyph('═'),
            RGB::named(rltk::BLACK).to_rgba(1.0),
            RGB::named(rltk::GRAY).to_rgba(1.0),
        ))
        .with(Photometry::new())
        .with(Name::new("Duct".to_string()))
        .with(Duct::new())
        .with(EntityDirection::new(Direction::E))
        .with(Blocker::new(vec![
            Direction::N,
            Direction::S,
            Direction::W,
            Direction::DOWN,
        ]))
        .with(VisionBlocker::new(vec![
            Direction::N,
            Direction::S,
            Direction::W,
            Direction::DOWN,
        ]))
        .with(Name::new(format!("Duct ({:?})", Direction::E)))
        .marked::<SimpleMarker<SerializeThis>>()
        .build();

    ecs.create_entity()
        .with(start_position + Vector3i::new(-1, 0, 1))
        .with(Renderable::new(
            char_to_glyph('■'),
            char_to_glyph('■'),
            RGB::named(rltk::BLACK).to_rgba(1.0),
            RGB::named(rltk::GRAY).to_rgba(1.0),
        ))
        .with(Photometry::new())
        .with(Name::new("Duct".to_string()))
        .with(Duct::new())
        .with(Name::new(format!("Duct ({:?})", Direction::UP)))
        .with(Blocker::new_cardinal_sides())
        .with(VisionBlocker::new_cardinal_sides())
        .marked::<SimpleMarker<SerializeThis>>()
        .build();

    let mut prev_position = start_position;
    let mut prev_direction = Direction::DOWN;

    let mut count = 0;

    for position in path.iter() {
        let mut next_vec_direction = *position - prev_position;

        let mut vec_direction = *position - prev_position;
        let mut direction = Direction::NW;

        let mut char = '║';

        let next_position = path.get(count + 1);

        if let Some(next_postion) = next_position {
            next_vec_direction = *next_postion - *position;

            if prev_position == *position {
                vec_direction = next_vec_direction;
            }
        }

        let mut sides: Vec<Direction> = Vec::new();

        if vec_direction == Vector3i::N {
            direction = Direction::N;
            char = '║';

            if next_vec_direction != Vector3i::N {
                sides.push(Direction::N);
            }

            if next_vec_direction == Vector3i::E {
                char = '╔';
            } else {
                sides.push(Direction::E);
            }

            if next_vec_direction == Vector3i::W {
                char = '╗';
            } else {
                sides.push(Direction::W);
            }

            if next_vec_direction != Vector3i::UP
                && (prev_direction != Direction::DOWN || prev_direction != Direction::UP)
            {
                sides.push(Direction::UP);
            }

            if next_vec_direction != Vector3i::DOWN {
                sides.push(Direction::DOWN);
            }
        } else if vec_direction == Vector3i::W {
            direction = Direction::W;
            char = '═';

            if next_vec_direction != Vector3i::W {
                sides.push(Direction::W);
            }

            if next_vec_direction == Vector3i::N {
                char = '╝';
            } else {
                sides.push(Direction::N);
            }

            if next_vec_direction == Vector3i::S {
                char = '╗';
            } else {
                sides.push(Direction::S);
            }

            if next_vec_direction != Vector3i::UP
                && !(prev_direction == Direction::DOWN || prev_direction == Direction::UP)
            {
                sides.push(Direction::UP);
            }

            if next_vec_direction == Vector3i::DOWN {
                sides.push(Direction::DOWN);
            }
        } else if vec_direction == Vector3i::S {
            direction = Direction::S;
            char = '║';

            if next_vec_direction != Vector3i::S {
                sides.push(Direction::S);
            }

            if next_vec_direction == Vector3i::E {
                char = '╔';
            } else {
                sides.push(Direction::E);
            }

            if next_vec_direction == Vector3i::W {
                char = '╗';
            } else {
                sides.push(Direction::W);
            }

            if next_vec_direction != Vector3i::UP
                && !(prev_direction == Direction::DOWN || prev_direction == Direction::UP)
            {
                sides.push(Direction::UP);
            }

            if next_vec_direction != Vector3i::DOWN {
                sides.push(Direction::DOWN);
            }
        } else if vec_direction == Vector3i::E {
            direction = Direction::E;
            char = '═';

            if next_vec_direction != Vector3i::E {
                sides.push(Direction::E);
            }

            if next_vec_direction == Vector3i::N {
                char = '╝';
            } else {
                sides.push(Direction::N);
            }

            if next_vec_direction == Vector3i::S {
                char = '╗';
            } else {
                sides.push(Direction::S);
            }

            if next_vec_direction != Vector3i::UP
                && !(prev_direction == Direction::DOWN || prev_direction == Direction::UP)
            {
                sides.push(Direction::UP);
            }

            if next_vec_direction != Vector3i::DOWN {
                sides.push(Direction::DOWN);
            }
        } else if vec_direction == Vector3i::UP {
            direction = Direction::UP;

            if next_vec_direction != Vector3i::UP {
                sides.push(Direction::UP);
            }

            if next_vec_direction == Vector3i::N {
                char = '□';
            } else {
                sides.push(Direction::N);
            }

            if next_vec_direction == Vector3i::S {
                char = '□';
            } else {
                sides.push(Direction::S);
            }

            if next_vec_direction == Vector3i::E {
                char = '□';
            } else {
                sides.push(Direction::E);
            }

            if next_vec_direction == Vector3i::W {
                char = '□';
            } else {
                sides.push(Direction::W);
            }
        } else if vec_direction == Vector3i::DOWN {
            direction = Direction::DOWN;

            if next_vec_direction != Vector3i::UP {
                sides.push(Direction::UP);
            }

            if next_vec_direction == Vector3i::N {
                char = '□';
            } else {
                sides.push(Direction::N);
            }

            if next_vec_direction == Vector3i::S {
                char = '□';
            } else {
                sides.push(Direction::S);
            }

            if next_vec_direction == Vector3i::E {
                char = '□';
            } else {
                sides.push(Direction::E);
            }

            if next_vec_direction == Vector3i::W {
                char = '□';
            } else {
                sides.push(Direction::W);
            }
        }

        ecs.create_entity()
            .with(*position)
            .with(Renderable::new(
                char_to_glyph(char),
                char_to_glyph(char),
                RGB::named(rltk::BLACK).to_rgba(1.0),
                RGB::named(rltk::GRAY).to_rgba(1.0),
            ))
            .with(Photometry::new())
            .with(Name::new("Duct".to_string()))
            .with(Duct::new())
            .with(EntityDirection::new(direction.clone()))
            .with(Blocker::new(sides.clone()))
            .with(VisionBlocker::new(sides))
            .with(Name::new(format!("Duct ({:?})", direction)))
            .with(PowerNode::new())
            .marked::<SimpleMarker<SerializeThis>>()
            .build();

        prev_position = *position;
        prev_direction = direction;

        count += 1;
    }
}

pub fn breaker_box(ecs: &mut World, position: Vector3i) {
    ecs.create_entity()
        .with(position)
        .with(Renderable::new(
            char_to_glyph('b'),
            char_to_glyph('b'),
            RGB::named(rltk::BLACK).to_rgba(1.0),
            RGB::named(rltk::GRAY).to_rgba(1.0),
        ))
        .with(Photometry::new())
        .with(BreakerBox {})
        .with(Name::new("Breaker box".to_string()))
        .with(PowerSwitch::new(true))
        .with(PowerNode::new())
        .marked::<SimpleMarker<SerializeThis>>()
        .build();
}

pub fn lay_wiring(
    ecs: &mut World,
    map: Map,
    start_position: Vector3i,
    end_position: Vector3i,
    color: RGBA,
    color_name: String,
    roof_preferred: bool,
) {
    let path;
    if start_position.z == end_position.z {
        path = find_walkable_path(map, start_position, end_position);
    } else {
        path = wall_climb_path(map, start_position, end_position, roof_preferred);
    }

    let mut prev_position = start_position;

    for position in path.iter() {
        let vec_direction = *position - prev_position;
        let mut direction = Direction::NW;
        let mut char = '.';

        let mut wire_present = false;
        //If the wire meets an existing wire, do not add a second wire
        {
            let positions = ecs.read_storage::<Vector3i>();
            let wires = ecs.read_storage::<Wire>();

            for (_, _) in (&wires, &positions)
                .join()
                .filter(|(wire, x)| wire.color_name == color_name && *x == position)
            {
                wire_present = true;
            }
        }

        if wire_present {
            continue;
        }

        if vec_direction == Vector3i::N {
            direction = Direction::N;
            char = '|';
        } else if vec_direction == Vector3i::W {
            direction = Direction::W;
            char = '-';
        } else if vec_direction == Vector3i::S {
            direction = Direction::S;
            char = '|';
        } else if vec_direction == Vector3i::E {
            direction = Direction::E;
            char = '-';
        } else if vec_direction == Vector3i::UP {
            direction = Direction::UP;
            char = '.';
        } else if vec_direction == Vector3i::DOWN {
            direction = Direction::DOWN;
            char = '.';
        }

        ecs.create_entity()
            .with(*position)
            .with(Renderable::new(
                char_to_glyph(char),
                char_to_glyph(char),
                color,
                RGB::named(rltk::BLACK).to_rgba(0.0),
            ))
            .with(Photometry::new())
            .with(Name::new(format!("Wire ({})", color_name.clone())))
            .with(Wire::new(color, color_name.clone()))
            .with(EntityDirection::new(direction))
            .with(PowerNode::new())
            .marked::<SimpleMarker<SerializeThis>>()
            .build();

        prev_position = *position;
    }
}

#[allow(dead_code)]
pub fn heater(ecs: &mut World, position: Vector3i, target_temperature: f32, on: bool) -> Entity {
    ecs.create_entity()
        .with(position)
        .with(Renderable::new(
            char_to_glyph('H'),
            char_to_glyph('H'),
            RGB::named(rltk::BLACK).to_rgba(1.0),
            RGB::named(rltk::GRAY).to_rgba(1.0),
        ))
        .with(Viewshed::new(2, 1, 1.0))
        .with(Photometry::new())
        .with(Illuminant::new(
            0.5,
            2,
            RGB::named(rltk::DARK_ORANGE).to_rgba(1.0),
            PI * 2.0,
            true,
        ))
        .with(Name::new("Heater".to_string()))
        .with(PoweredState::new(true, 10.0))
        .with(PowerSwitch::new(on))
        .with(PowerNode::new())
        .with(ElectronicHeater::new(target_temperature, on))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}
