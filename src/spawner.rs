use std::{collections::HashSet, f32::consts::PI};

use rltk::{RGB, RGBA};
use specs::{
    prelude::*,
    saveload::{MarkedBuilder, SimpleMarker},
    storage::GenericReadStorage,
};

use crate::{
    entities::{
        biology::Breather,
        intents::Initiative,
        power_components::{ControlPanel, ElectronicHeater},
        props::Cabinet,
    },
    graphics::char_to_glyph,
    pathfinding::{find_walkable_path, wall_climb_path},
    vectors::{
        utils::{get_cardinal_neighbours, get_cardinal_neighbours_with_z},
        Vector3i,
    },
    Blocker, Container, Direction, Door, Duct, EntityDirection, Illuminant, InContainer, Installed,
    Item, Map, Name, Photometry, Player, PowerNode, PowerSource, PowerSwitch, PoweredState, Prop,
    Renderable, SerializeThis, Viewshed, VisionBlocker, Wire,
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
        .with(Container::new(2.0))
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
        .with(Prop::new())
        .with(Installed::new())
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
        .with(Prop::new())
        .with(Installed::new())
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
            .with(Installed::new())
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
            .with(Prop::new())
            .with(Installed::new())
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
        .with(Prop::new())
        .with(Installed::new())
        .marked::<SimpleMarker<SerializeThis>>()
        .build();
}

#[allow(dead_code)]
pub fn lay_ducting(ecs: &mut World, map: Map, start_position: Vector3i, end_position: Vector3i) {
    let path: Vec<Vector3i>;
    if start_position.z == end_position.z {
        path = find_walkable_path(map, start_position, end_position);
    } else {
        path = wall_climb_path(map, start_position, end_position, &HashSet::new(), true)
    };

    ecs.create_entity()
        .with(start_position + Vector3i::DOWN)
        .with(Renderable::new(
            char_to_glyph('■'),
            char_to_glyph('■'),
            RGB::named(rltk::BLACK).to_rgba(1.0),
            RGB::named(rltk::GRAY).to_rgba(1.0),
        ))
        .with(Photometry::new())
        .with(Name::new("Duct".to_string()))
        .with(Duct::new())
        .with(EntityDirection::new(Direction::UP))
        .with(Blocker::new(vec![
            Direction::N,
            Direction::S,
            Direction::W,
            Direction::E,
        ]))
        .with(VisionBlocker::new(vec![
            Direction::N,
            Direction::S,
            Direction::W,
            Direction::E,
        ]))
        .with(Name::new(format!("Duct ({:?})", Direction::UP)))
        .with(Installed::new())
        .marked::<SimpleMarker<SerializeThis>>()
        .build();

    let mut prev_position = start_position;
    let mut prev_direction = Direction::UP;

    let mut count = 0;

    for position in path.iter() {
        let mut next_vec_direction = *position - prev_position;

        let mut vec_direction = *position - prev_position;
        let mut direction = Direction::NW;

        let next_position = path.get(count + 1);

        let mut duct_prevent = false;

        //If the duct meets an existing one, do not add a second
        {
            let positions = ecs.read_storage::<Vector3i>();
            let ducts = ecs.read_storage::<Duct>();

            for (_, _) in (&ducts, &positions).join().filter(|(_, x)| *x == position) {
                duct_prevent = true;
            }
        }

        if duct_prevent {
            continue;
        }

        if let Some(next_postion) = next_position {
            next_vec_direction = *next_postion - *position;

            if prev_position == *position {
                vec_direction = next_vec_direction;
            }
        }

        let mut sides: HashSet<Direction> = HashSet::new();

        if vec_direction == Vector3i::N {
            direction = Direction::N;

            if next_vec_direction != Vector3i::N {
                sides.insert(Direction::N);
            }

            if next_vec_direction == Vector3i::E {
            } else {
                sides.insert(Direction::E);
            }

            if next_vec_direction == Vector3i::W {
            } else {
                sides.insert(Direction::W);
            }

            if next_vec_direction != Vector3i::UP
                && (prev_direction != Direction::DOWN || prev_direction != Direction::UP)
            {
                sides.insert(Direction::UP);
            }

            if next_vec_direction != Vector3i::DOWN {
                sides.insert(Direction::DOWN);
            }
        } else if vec_direction == Vector3i::W {
            direction = Direction::W;

            if next_vec_direction != Vector3i::W {
                sides.insert(Direction::W);
            }

            if next_vec_direction == Vector3i::N {
            } else {
                sides.insert(Direction::N);
            }

            if next_vec_direction == Vector3i::S {
            } else {
                sides.insert(Direction::S);
            }

            if next_vec_direction != Vector3i::UP
                && !(prev_direction == Direction::DOWN || prev_direction == Direction::UP)
            {
                sides.insert(Direction::UP);
            }

            if next_vec_direction == Vector3i::DOWN {
                sides.insert(Direction::DOWN);
            }
        } else if vec_direction == Vector3i::S {
            direction = Direction::S;

            if next_vec_direction != Vector3i::S {
                sides.insert(Direction::S);
            }

            if next_vec_direction == Vector3i::E {
            } else {
                sides.insert(Direction::E);
            }

            if next_vec_direction == Vector3i::W {
            } else {
                sides.insert(Direction::W);
            }

            if next_vec_direction != Vector3i::UP
                && !(prev_direction == Direction::DOWN || prev_direction == Direction::UP)
            {
                sides.insert(Direction::UP);
            }

            if next_vec_direction != Vector3i::DOWN {
                sides.insert(Direction::DOWN);
            }
        } else if vec_direction == Vector3i::E {
            direction = Direction::E;

            if next_vec_direction != Vector3i::E {
                sides.insert(Direction::E);
            }

            if next_vec_direction == Vector3i::N {
            } else {
                sides.insert(Direction::N);
            }

            if next_vec_direction == Vector3i::S {
            } else {
                sides.insert(Direction::S);
            }

            if next_vec_direction != Vector3i::UP
                && !(prev_direction == Direction::DOWN || prev_direction == Direction::UP)
            {
                sides.insert(Direction::UP);
            }

            if next_vec_direction != Vector3i::DOWN {
                sides.insert(Direction::DOWN);
            }
        } else if vec_direction == Vector3i::UP {
            direction = Direction::UP;

            if next_vec_direction != Vector3i::UP {
                sides.insert(Direction::UP);
            }

            if next_vec_direction == Vector3i::N {
            } else {
                sides.insert(Direction::N);
            }

            if next_vec_direction == Vector3i::S {
            } else {
                sides.insert(Direction::S);
            }

            if next_vec_direction == Vector3i::E {
            } else {
                sides.insert(Direction::E);
            }

            if next_vec_direction == Vector3i::W {
            } else {
                sides.insert(Direction::W);
            }
        } else if vec_direction == Vector3i::DOWN {
            direction = Direction::DOWN;
        }

        {
            //If there is a node that runs into this one, connect them
            let neighbours = get_cardinal_neighbours_with_z(*position);

            let positions = ecs.read_storage::<Vector3i>();
            let ducts = ecs.read_storage::<Duct>();
            let directions = ecs.read_storage::<EntityDirection>();
            let mut blockers = ecs.write_storage::<Blocker>();
            let mut vision_blockers = ecs.write_storage::<VisionBlocker>();
            let mut renderables = ecs.write_storage::<Renderable>();

            for neighbour in neighbours.iter() {
                for (_, other_position, other_direction, blockers, vision_blockers, renderable) in (
                    &ducts,
                    &positions,
                    &directions,
                    &mut blockers,
                    &mut vision_blockers,
                    &mut renderables,
                )
                    .join()
                    .filter(|(_, x, _, _, _, _)| **x == *neighbour)
                {
                    if (direction == Direction::N || direction == Direction::S)
                        && (other_direction.direction == Direction::E
                            || other_direction.direction == Direction::W)
                    {
                        let mut relative_direction =
                            (*other_position - *position).normalize_delta();

                        relative_direction.z = 0;

                        if relative_direction == Vector3i::N {
                            sides.remove(&Direction::N);
                            blockers.remove_side(Direction::S);
                            vision_blockers.remove_side(Direction::S);
                        } else {
                            sides.remove(&Direction::S);
                            blockers.remove_side(Direction::N);
                            vision_blockers.remove_side(Direction::N);
                        }
                    } else if (direction == Direction::E || direction == Direction::W)
                        && (other_direction.direction == Direction::N
                            || other_direction.direction == Direction::S)
                    {
                        let mut relative_direction =
                            (*other_position - *position).normalize_delta();

                        relative_direction.z = 0;

                        if relative_direction == Vector3i::E {
                            sides.remove(&Direction::E);
                            blockers.remove_side(Direction::W);
                            vision_blockers.remove_side(Direction::W);
                        } else {
                            sides.remove(&Direction::W);
                            blockers.remove_side(Direction::E);
                            vision_blockers.remove_side(Direction::E);
                        }
                    }

                    let mut relative_direction = (*other_position - *position).normalize_delta();

                    relative_direction.x = 0;
                    relative_direction.y = 0;

                    if other_direction.direction == Direction::UP {
                        sides.remove(&Direction::DOWN);
                        blockers.remove_side(Direction::UP);
                        vision_blockers.remove_side(Direction::UP);
                    } else if other_direction.direction == Direction::DOWN {
                        sides.remove(&Direction::UP);
                        blockers.remove_side(Direction::DOWN);
                        vision_blockers.remove_side(Direction::DOWN);
                    }

                    let new_char = char_to_glyph(update_duct_char_from_sides(&blockers.sides));
                    renderable.side_glyph = new_char;
                    renderable.top_glyph = new_char;
                }
            }
        }

        let char = update_duct_char(&sides);

        let sides: Vec<Direction> = sides.into_iter().collect();

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
            .with(Prop::new())
            .with(Installed::new())
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
        .with(ControlPanel {})
        .with(Name::new("Control panel".to_string()))
        .with(PowerSwitch::new(true))
        .with(PowerNode::new())
        .with(Prop::new())
        .with(Installed::new())
        .marked::<SimpleMarker<SerializeThis>>()
        .build();
}

pub fn lay_wiring(
    ecs: &mut World,
    map: Map,
    start_position: Vector3i,
    end_position: Vector3i,
    avoid_positions: &HashSet<Vector3i>,
    color: RGBA,
    color_name: String,
    roof_preferred: bool,
    data: bool,
) {
    let path;
    if start_position.z == end_position.z {
        path = find_walkable_path(map, start_position, end_position);
    } else {
        path = wall_climb_path(
            map,
            start_position,
            end_position,
            avoid_positions,
            roof_preferred,
        );
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
            .with(Wire::new(color, color_name.clone(), data))
            .with(EntityDirection::new(direction))
            .with(PowerNode::new())
            .with(Prop::new())
            .with(Installed::new())
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
        .with(Prop::new())
        .with(ElectronicHeater::new(target_temperature, on))
        .with(Installed::new())
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub fn test_item(ecs: &mut World, position: Vector3i) -> Entity {
    ecs.create_entity()
        .with(position)
        .with(Renderable::new(
            char_to_glyph('¡'),
            char_to_glyph('¡'),
            RGB::named(rltk::RED).to_rgba(1.0),
            RGB::named(rltk::BLACK).to_rgba(0.0),
        ))
        .with(Photometry::new())
        .with(Name::new("Test tube".to_string()))
        .with(Item::new(1.0, 0.1))
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub fn storage_cabinet(ecs: &mut World, position: Vector3i) -> Entity {
    ecs.create_entity()
        .with(Cabinet::new())
        .with(position)
        .with(Renderable::new(
            char_to_glyph('H'),
            char_to_glyph('H'),
            RGB::named(rltk::WHITE).to_rgba(1.0),
            RGB::named(rltk::GRAY5).to_rgba(1.0),
        ))
        .with(Photometry::new())
        .with(Name::new("Storage cabinet".to_string()))
        .with(Installed::new())
        .with(Container::new(100.0))
        .with(Blocker::new_all_sides())
        .marked::<SimpleMarker<SerializeThis>>()
        .build()
}

pub fn put_item_in_container(ecs: &mut World, item: Entity, container: Entity) {
    let mut positions = ecs.write_storage::<Vector3i>();
    let mut in_container = ecs.write_storage::<InContainer>();
    let containers = ecs.write_storage::<Container>();

    positions.remove(item);

    if let Some(container_component) = containers.get(container) {
        let _ = in_container.insert(item, InContainer::new(container_component.id));
    }
}

fn update_duct_char(sides: &HashSet<Direction>) -> char {
    match (
        sides.contains(&Direction::N),
        sides.contains(&Direction::S),
        sides.contains(&Direction::E),
        sides.contains(&Direction::W),
    ) {
        (false, false, false, false) => '╬', // All sides open
        (true, true, true, true) => '■',     // All sides closed
        (true, true, true, false) => '╣',    // West open
        (true, true, false, true) => '╠',    // East open
        (true, false, true, true) => '╩',    // South open
        (false, true, true, true) => '╦',    // North open
        (true, true, false, false) => '═',   // East and West open
        (false, false, true, true) => '║',   // North and South open
        (false, true, false, true) => '╗',   // North and East open
        (false, true, true, false) => '╔',   // North and West open
        (true, false, false, true) => '╝',   // South and East open
        (true, false, true, false) => '╚',   // South and West open
        (false, true, false, false) => '╨',  // North, East, and West open
        (true, false, false, false) => '╥',  // South, East, and West open
        (false, false, false, true) => '╡',  // North, South, and East open
        (false, false, true, false) => '╞',  // North, South, and West open
    }
}

pub fn update_duct_char_from_sides(sides: &Vec<Direction>) -> char {
    match (
        sides.contains(&Direction::N),
        sides.contains(&Direction::S),
        sides.contains(&Direction::E),
        sides.contains(&Direction::W),
    ) {
        (false, false, false, false) => '╬', // All sides open
        (true, true, true, true) => '■',     // All sides closed
        (true, true, true, false) => '╣',    // West open
        (true, true, false, true) => '╠',    // East open
        (true, false, true, true) => '╩',    // South open
        (false, true, true, true) => '╦',    // North open
        (true, true, false, false) => '═',   // East and West open
        (false, false, true, true) => '║',   // North and South open
        (false, true, false, true) => '╗',   // North and East open
        (false, true, true, false) => '╔',   // North and West open
        (true, false, false, true) => '╝',   // South and East open
        (true, false, true, false) => '╚',   // South and West open
        (false, true, false, false) => '╨',  // North, East, and West open
        (true, false, false, false) => '╥',  // South, East, and West open
        (false, false, false, true) => '╡',  // North, South, and East open
        (false, false, true, false) => '╞',  // North, South, and West open
    }
}
