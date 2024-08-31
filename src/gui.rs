use std::u32::MAX;

use crate::entities::intents::{InteractIntent, OpenIntent, PickUpIntent};
use crate::entities::power_components::{
    BreakerBox, PowerNode, PowerSource, PowerSwitch, PoweredState, Wire,
};
use crate::graphics::char_to_glyph;
use crate::menu::interaction_menu;
use crate::systems::event_system::{
    get_default_interactions, InteractionInformation, InteractionType,
};
use crate::systems::power_system::get_devices_on_subnetwork;
use crate::{mouse_to_map, InContainer, Installed, Item, INTERACT_MENU_WIDTH};
use crate::{systems::event_system::get_entity_interactions, Renderable};
use rltk::{to_char, Point, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

use crate::{
    gamelog::GameLog, get_player_entity, graphics::get_viewport_position, vectors::Vector3i, Map,
    Name, Player, RunState, State, Viewshed, MAP_SCREEN_HEIGHT, MAP_SCREEN_WIDTH, TERMINAL_HEIGHT,
    TERMINAL_WIDTH,
};

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection {
    NewGame,
    LoadGame,
    Quit,
}

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult {
    NoSelection { selected: MainMenuSelection },
    Selected { selected: MainMenuSelection },
}

pub fn draw_ui(ecs: &World, ctx: &mut Rltk, draw_pointer: bool) {
    ctx.set_active_console(2);
    ctx.cls();

    let gui_height = TERMINAL_HEIGHT - MAP_SCREEN_HEIGHT - 1;
    ctx.draw_box(
        0,
        MAP_SCREEN_HEIGHT,
        TERMINAL_WIDTH - 1,
        gui_height,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let log = ecs.fetch::<GameLog>();

    let mut y = 1;

    for entry in log.entries.iter().rev() {
        if y < gui_height {
            ctx.print(2, MAP_SCREEN_HEIGHT + y, entry);
        }
        y += 1;
    }

    let viewport_position = get_viewport_position(&ecs);

    if draw_pointer {
        let mouse_position = ctx.mouse_pos();

        ctx.set(
            mouse_position.0,
            mouse_position.1,
            RGB::named(rltk::GOLD),
            RGB::named(rltk::BLACK).to_rgba(0.0),
            char_to_glyph('┼'),
        );

        let mouse_pos = ctx.mouse_pos();
        if mouse_pos.0 >= MAP_SCREEN_WIDTH || mouse_pos.1 >= MAP_SCREEN_HEIGHT {
            return;
        }

        let player_pos = ecs.fetch::<Vector3i>();

        let map_mouse_position = Vector3i::new(
            mouse_pos.0 - (MAP_SCREEN_WIDTH / 2) + viewport_position.x,
            mouse_pos.1 - (MAP_SCREEN_HEIGHT / 2) + viewport_position.y,
            player_pos.z,
        );
        draw_tooltips(ecs, ctx, map_mouse_position);
    }

    let target = ecs.read_resource::<Vector3i>();

    //Draw interact menu

    ctx.draw_box(
        MAP_SCREEN_WIDTH - 1,
        0,
        INTERACT_MENU_WIDTH,
        MAP_SCREEN_HEIGHT - 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let map = ecs.fetch::<Map>();

    let players = ecs.read_storage::<Player>();
    let viewsheds = ecs.read_storage::<Viewshed>();
    let positions = ecs.read_storage::<Vector3i>();
    let breaker_boxes = ecs.read_storage::<BreakerBox>();
    let entities = ecs.entities();

    let player = get_player_entity(&entities, &players);

    let mut interactables: Vec<InteractionInformation> = Vec::new();
    let mut tile_entities: Vec<Entity> = Vec::new();

    let target_tile = map.tiles.get(&target);

    if let Some(player_entity) = player {
        if let Some(player_viewshed) = viewsheds.get(player_entity) {
            for (entity, position) in (&entities, &positions)
                .join()
                .filter(|&x| player_viewshed.visible_tiles.contains(x.1))
            {
                //if position.x == target.x && position.y == target.y && (position.z == target.z || position.z == target.z - 1) {
                if position.x == target.x && position.y == target.y && (position.z == target.z) {
                    interactables.append(&mut get_entity_interactions(&ecs, entity));
                    tile_entities.push(entity);

                    //Handle breaker boxes or other entities that control interactions off tile
                    if let Some(_) = breaker_boxes.get(entity) {
                        interactables.append(&mut get_devices_on_subnetwork(&ecs, entity));
                    }
                }
            }
        }
    }

    let tile_info_y = 1;
    const TILE_INFORMATION_MENU_HEIGHT: i32 = 10;

    if let Some(target_tile) = target_tile {
        ctx.draw_hollow_box(
            MAP_SCREEN_WIDTH,
            tile_info_y,
            INTERACT_MENU_WIDTH - 2,
            TILE_INFORMATION_MENU_HEIGHT,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 1,
            format!("{}", target_tile.name.clone()),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 2,
            format!(
                "Glyphs: {}, {}",
                to_char(target_tile.renderable.top_glyph as u8),
                to_char(target_tile.renderable.side_glyph as u8)
            ),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 3,
            format!("Light level: {:.2}", target_tile.photometry.light_level),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 4,
            format!(
                "Color: ({:.2},{:.2},{:.2})",
                target_tile.renderable.foreground.r,
                target_tile.renderable.foreground.g,
                target_tile.renderable.foreground.b
            ),
        );

        let mut count = 0;
        for (gas, mols) in &target_tile.atmosphere.gasses {
            ctx.print(
                MAP_SCREEN_WIDTH + 1,
                tile_info_y + 5 + count,
                format!(
                    "{}: {:.3}, {:.2}%",
                    gas,
                    mols,
                    (target_tile.atmosphere.get_gas_ratio((*gas).clone()) * 100.0)
                ),
            );
            count += 1;
        }
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 5 + count + 1,
            format!(
                "Temperature: {:.2} C",
                target_tile.atmosphere.get_celcius_temperature()
            ),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 5 + count + 2,
            format!(
                "Pressure: {:.2} kpa",
                target_tile.atmosphere.get_pressure_kpa()
            ),
        );
    }
}

pub fn draw_tooltips(ecs: &World, ctx: &mut Rltk, target: Vector3i) {
    let entities = ecs.entities();
    let players = ecs.read_storage::<Player>();
    let viewsheds = ecs.read_storage::<Viewshed>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Vector3i>();

    let viewport_position = get_viewport_position(&ecs);

    let screen_position = Vector3i::new(
        target.x + MAP_SCREEN_WIDTH / 2 - viewport_position.x,
        target.y + MAP_SCREEN_HEIGHT / 2 - viewport_position.y,
        target.z,
    );

    ctx.set_active_console(2);

    let mut tooltip: Vec<String> = Vec::new();

    if let Some(player_entity) = get_player_entity(&entities, &players) {
        if let Some(player_viewshed) = viewsheds.get(player_entity) {
            for (name, position) in (&names, &positions)
                .join()
                .filter(|&x| player_viewshed.visible_tiles.contains(x.1))
            {
                if position.x == target.x && position.y == target.y && (position.z == target.z) {
                    if position.z < viewport_position.z {
                        tooltip.push((name.name.to_string() + " (below)").to_string());
                    } else {
                        tooltip.push(name.name.to_string());
                    }
                }
            }
        }
    }

    if !tooltip.is_empty() {
        let mut width = 0;

        for item in tooltip.iter() {
            if width < item.len() as i32 {
                width = item.len() as i32;
            }
        }
        width += 3;

        if screen_position.x > TERMINAL_WIDTH / 2 {
            let arrow_pos = Point::new(screen_position.x - 2, screen_position.y);
            let left_x = screen_position.x - width;
            let mut y = screen_position.y;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::BLACK),
                    s,
                );
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x - i,
                        y,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::BLACK),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::GREY),
                &"->".to_string(),
            );
        } else {
            let arrow_pos = Point::new(screen_position.x + 2, screen_position.y);

            let left_x = screen_position.x + 3;
            let mut y = screen_position.y;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x + 2,
                    y,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::BLACK),
                    s,
                );
                let padding = (width - s.len() as i32) - 1;
                for i in 0..padding {
                    ctx.print_color(
                        arrow_pos.x + 1 + i,
                        y,
                        RGB::named(rltk::WHITE),
                        RGB::named(rltk::BLACK),
                        &" ".to_string(),
                    );
                }
                y += 1;
            }
            ctx.print_color(
                arrow_pos.x,
                arrow_pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                &"<-".to_string(),
            );
        }
    }
}

pub fn interact_gui(
    game_state: &mut State,
    ctx: &mut Rltk,
    range: usize,
    source: Vector3i,
    mut target: Vector3i,
    prev_mouse_position: Vector3i,
    mut selected_entity: Option<Entity>,
) -> RunState {
    crate::set_camera_z(target.z, &mut game_state.ecs);
    let viewport_position = get_viewport_position(&game_state.ecs);

    let mouse_pos = ctx.mouse_pos();

    let mouse_position = mouse_to_map(mouse_pos, viewport_position);

    let entities = game_state.ecs.entities();

    draw_ui(&game_state.ecs, ctx, false);

    ctx.set(
        target.x + MAP_SCREEN_WIDTH / 2 - viewport_position.x,
        target.y + MAP_SCREEN_HEIGHT / 2 - viewport_position.y,
        RGB::named(rltk::GOLD),
        RGB::named(rltk::BLACK).to_rgba(0.0),
        char_to_glyph('┼'),
    );
    draw_tooltips(&game_state.ecs, ctx, target);

    //Draw interact menu

    ctx.draw_box(
        MAP_SCREEN_WIDTH - 1,
        0,
        INTERACT_MENU_WIDTH,
        MAP_SCREEN_HEIGHT - 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let map = game_state.ecs.fetch::<Map>();

    let players = game_state.ecs.read_storage::<Player>();
    let renderables = game_state.ecs.read_storage::<Renderable>();
    let names = game_state.ecs.read_storage::<Name>();
    let viewsheds = game_state.ecs.read_storage::<Viewshed>();
    let positions = game_state.ecs.read_storage::<Vector3i>();
    let power_states = game_state.ecs.read_storage::<PoweredState>();
    let power_switches = game_state.ecs.read_storage::<PowerSwitch>();
    let power_sources = game_state.ecs.read_storage::<PowerSource>();
    let wires = game_state.ecs.read_storage::<Wire>();
    let nodes = game_state.ecs.read_storage::<PowerNode>();
    let breaker_boxes = game_state.ecs.read_storage::<BreakerBox>();
    let installed = game_state.ecs.read_storage::<Installed>();
    let items = game_state.ecs.read_storage::<Item>();

    let player = get_player_entity(&entities, &players);

    let mut interactables: Vec<InteractionInformation> = Vec::new();
    let mut tile_entities: Vec<Entity> = Vec::new();

    let target_tile = map.tiles.get(&target);

    if let Some(player_entity) = player {
        if let Some(player_viewshed) = viewsheds.get(player_entity) {
            for (entity, position) in (&entities, &positions)
                .join()
                .filter(|(_, x)| player_viewshed.visible_tiles.contains(x) && **x == target)
            {
                tile_entities.push(entity);
            }
        }
    }

    match selected_entity {
        Some(entity) => {
            //TODO: Change this to allow uninstalling of items
            interactables.append(&mut get_default_interactions(&game_state.ecs, entity));
            interactables.append(&mut get_entity_interactions(&game_state.ecs, entity));

            //Handle breaker boxes or other entities that control interactions off tile
            if let Some(_) = breaker_boxes.get(entity) {
                interactables.append(&mut get_devices_on_subnetwork(&game_state.ecs, entity));
            }
        }
        _ => {}
    }

    let mut entity_menu_y = 0;
    let mut interactable_menu_y = 0;

    let tile_info_y = 1;
    const TILE_INFORMATION_MENU_HEIGHT: i32 = 10;

    if let Some(target_tile) = target_tile {
        entity_menu_y = TILE_INFORMATION_MENU_HEIGHT + 2;
        interactable_menu_y = TILE_INFORMATION_MENU_HEIGHT + 2;

        ctx.draw_hollow_box(
            MAP_SCREEN_WIDTH,
            tile_info_y,
            INTERACT_MENU_WIDTH - 2,
            TILE_INFORMATION_MENU_HEIGHT,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 1,
            format!("{}", target_tile.name.clone()),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 2,
            format!(
                "Glyphs: {}, {}",
                to_char(target_tile.renderable.top_glyph as u8),
                to_char(target_tile.renderable.side_glyph as u8)
            ),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 3,
            format!("Light level: {:.2}", target_tile.photometry.light_level),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 4,
            format!(
                "Color: ({:.2},{:.2},{:.2})",
                target_tile.renderable.foreground.r,
                target_tile.renderable.foreground.g,
                target_tile.renderable.foreground.b
            ),
        );

        let mut count = 0;
        for (gas, mols) in &target_tile.atmosphere.gasses {
            ctx.print(
                MAP_SCREEN_WIDTH + 1,
                tile_info_y + 5 + count,
                format!(
                    "{}: {:.3}, {:.2}%",
                    gas,
                    mols,
                    (target_tile.atmosphere.get_gas_ratio((*gas).clone()) * 100.0)
                ),
            );
            count += 1;
        }
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 5 + count + 1,
            format!(
                "Temperature: {:.2}",
                target_tile.atmosphere.get_celcius_temperature()
            ),
        );
        ctx.print(
            MAP_SCREEN_WIDTH + 1,
            tile_info_y + 5 + count + 2,
            format!(
                "Pressure: {:.2} kpa",
                target_tile.atmosphere.get_pressure_kpa()
            ),
        );
    }

    const ENTITY_FIELDS: usize = 20;
    const MAX_ENTITY_MENU_HEIGHT: usize = 30;

    if tile_entities.len() > 0 {
        let mut render_menu = false;
        let entity_menu_height = (tile_entities.len() * ENTITY_FIELDS).min(MAX_ENTITY_MENU_HEIGHT);
        let mut y = 3;
        let mut prev_id = MAX;

        let mut count = 0;

        for entity in tile_entities.iter() {
            let renderable = renderables.get(*entity);
            let name = names.get(*entity);

            if y < MAX_ENTITY_MENU_HEIGHT as i32 - 5 {
                if entity.id() != prev_id {
                    let mut color = RGB::named(rltk::WHITE).to_rgba(1.0);

                    if let Some(renderable) = renderable {
                        color = renderable.foreground;

                        if color.r < 0.1 && color.g < 0.1 && color.b < 0.1 {
                            color = RGB::named(rltk::WHITE).to_rgba(1.0);
                        }
                    }
                    if let Some(name) = name {
                        let entity_name = name.name.to_string();

                        y += 1;
                        ctx.print_color(
                            MAP_SCREEN_WIDTH + 1,
                            entity_menu_y + y,
                            color,
                            RGB::named(rltk::BLACK),
                            format!("<{}> {}", to_char(97 + count), format!("{}", entity_name)),
                        );
                        y += 2;

                        render_menu = true;
                    }
                }

                if let Some(renderable) = renderables.get(*entity) {
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!(
                            "Glyphs: {}, {}",
                            to_char(renderable.top_glyph as u8),
                            to_char(renderable.side_glyph as u8)
                        ),
                    );
                    y += 1;
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!(
                            "Color: ({:.2},{:.2},{:.2})",
                            renderable.foreground.r,
                            renderable.foreground.g,
                            renderable.foreground.b
                        ),
                    );
                    y += 1;
                }
                if let Some(powered_state) = power_states.get(*entity) {
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!("Power on: {}", powered_state.state_description()),
                    );
                    y += 1;
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!("Power available: {}", powered_state.available_wattage),
                    );
                    y += 1;
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!("Power draw: {}", powered_state.wattage),
                    );
                    y += 1;
                }
                if let Some(wire) = wires.get(*entity) {
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!("Power available: {}", wire.available_wattage),
                    );
                    y += 1;
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!("Power load: {}", wire.power_load),
                    );
                    y += 1;
                }
                if let Some(power_switch) = power_switches.get(*entity) {
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!("Power switch: {}", power_switch.state_description()),
                    );
                    y += 1;
                }
                if let Some(power_source) = power_sources.get(*entity) {
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!("Power capacity: {}", power_source.max_wattage),
                    );
                    y += 1;
                }
                if let Some(node) = nodes.get(*entity) {
                    ctx.print(
                        MAP_SCREEN_WIDTH + 1,
                        entity_menu_y + y,
                        format!("Network: {}", node.network_id),
                    );
                    y += 1;
                }
            } else {
                //If the list gets too long just print the entity name
                if entity.id() != prev_id {
                    let mut color = RGB::named(rltk::WHITE).to_rgba(1.0);

                    if let Some(renderable) = renderable {
                        color = renderable.foreground;

                        if color.r < 0.1 && color.g < 0.1 && color.b < 0.1 {
                            color = RGB::named(rltk::WHITE).to_rgba(1.0);
                        }
                    }
                    if let Some(name) = name {
                        let entity_name = name.name.to_string();

                        y += 1;
                        ctx.print_color(
                            MAP_SCREEN_WIDTH + 1,
                            entity_menu_y + y,
                            color,
                            RGB::named(rltk::BLACK),
                            format!("{}", entity_name),
                        );
                        y += 2;

                        render_menu = true;
                    }

                    prev_id = entity.id();
                }
            }
            count += 1;
        }

        if render_menu {
            ctx.draw_hollow_box(
                MAP_SCREEN_WIDTH,
                entity_menu_y,
                INTERACT_MENU_WIDTH - 2,
                entity_menu_height,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
            );
            ctx.print_color(
                MAP_SCREEN_WIDTH + 1,
                interactable_menu_y + 1,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "Entities:",
            );
            interactable_menu_y += entity_menu_height as i32 + 2;
        }
    }

    interaction_menu(
        &game_state.ecs,
        ctx,
        interactables.clone(),
        Point::new(0, interactable_menu_y),
    );

    if (mouse_position.x != prev_mouse_position.x || mouse_position.x != prev_mouse_position.x)
        && (mouse_pos.0 <= MAP_SCREEN_WIDTH && mouse_pos.1 <= MAP_SCREEN_HEIGHT)
    {
        target = Vector3i::new(
            (mouse_position.x - source.x).abs().min(range as i32)
                * (mouse_position.x - source.x).signum()
                + source.x,
            (mouse_position.y - source.y).abs().min(range as i32)
                * (mouse_position.y - source.y).signum()
                + source.y,
            target.z,
        );
    }

    match ctx.key {
        None => {
            return RunState::InteractGUI {
                range,
                source,
                target,
                prev_mouse_position,
                selected_entity,
            }
        }
        Some(key) => match key {
            VirtualKeyCode::Escape => {
                if selected_entity == None {
                    return RunState::AwaitingInput;
                } else {
                    selected_entity = None;
                    return RunState::InteractGUI {
                        range,
                        source,
                        target,
                        prev_mouse_position,
                        selected_entity,
                    };
                }
            }
            VirtualKeyCode::I => return RunState::AwaitingInput,
            VirtualKeyCode::Period => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::DOWN,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::Comma => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::UP,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::N,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::Numpad9 => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::NE,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::E,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::Numpad3 => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::SE,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::S,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::Numpad1 => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::SW,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::W,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::Numpad7 => {
                return check_range(
                    range,
                    source,
                    target,
                    Vector3i::NW,
                    mouse_position,
                    selected_entity,
                );
            }
            VirtualKeyCode::A => {
                if selected_entity == None {
                    if tile_entities.len() > 0 {
                        {
                            selected_entity = Some(tile_entities[0].clone())
                        };
                    }
                } else if interactables.len() > 0 {
                    let interactable = interactables[0].clone();
                    let entity = entities.entity(interactable.entity_id);
                    if let Some(player) = player {
                        match interactable.interaction_type {
                            InteractionType::ComponentInteraction => {
                                let mut interactions =
                                    game_state.ecs.write_storage::<InteractIntent>();
                                let _ = interactions.insert(
                                    player,
                                    InteractIntent::new(
                                        player,
                                        entity,
                                        interactable.id,
                                        interactable.description.clone(),
                                        interactable.cost,
                                    ),
                                );
                            }
                            InteractionType::PickUpInteraction => {
                                let mut pick_up_intents =
                                    game_state.ecs.write_storage::<PickUpIntent>();
                                let _ = pick_up_intents.insert(
                                    player,
                                    PickUpIntent::new(
                                        player,
                                        entity,
                                        interactable.id,
                                        interactable.description.clone(),
                                        interactable.cost,
                                        0.0,
                                    ),
                                );
                            }
                            InteractionType::OpenInteraction => {
                                let mut open_intents = game_state.ecs.write_storage::<OpenIntent>();

                                let _ = open_intents.insert(
                                    player,
                                    OpenIntent::new(
                                        player,
                                        entity,
                                        interactable.id,
                                        interactable.description.clone(),
                                        interactable.cost,
                                    ),
                                );
                            }
                        }

                        return RunState::Ticking;
                    }
                }

                return RunState::InteractGUI {
                    range,
                    source,
                    target,
                    prev_mouse_position: mouse_position,
                    selected_entity,
                };
            }
            VirtualKeyCode::B => {
                if selected_entity == None {
                    if tile_entities.len() > 1 {
                        {
                            selected_entity = Some(tile_entities[1].clone())
                        };
                    }
                } else if interactables.len() > 1 {
                    let interactable = interactables[1].clone();
                    let entity = entities.entity(interactable.entity_id);
                    if let Some(player) = player {
                        match interactable.interaction_type {
                            InteractionType::ComponentInteraction => {
                                let mut interactions =
                                    game_state.ecs.write_storage::<InteractIntent>();
                                let _ = interactions.insert(
                                    player,
                                    InteractIntent::new(
                                        player,
                                        entity,
                                        interactable.id,
                                        interactable.description.clone(),
                                        interactable.cost,
                                    ),
                                );
                            }
                            InteractionType::PickUpInteraction => {
                                let mut pick_up_intents =
                                    game_state.ecs.write_storage::<PickUpIntent>();
                                let _ = pick_up_intents.insert(
                                    player,
                                    PickUpIntent::new(
                                        player,
                                        entity,
                                        interactable.id,
                                        interactable.description.clone(),
                                        interactable.cost,
                                        0.0,
                                    ),
                                );
                            }
                            InteractionType::OpenInteraction => {
                                let mut open_intents = game_state.ecs.write_storage::<OpenIntent>();

                                let _ = open_intents.insert(
                                    player,
                                    OpenIntent::new(
                                        player,
                                        entity,
                                        interactable.id,
                                        interactable.description.clone(),
                                        interactable.cost,
                                    ),
                                );
                            }
                        }

                        return RunState::Ticking;
                    }
                }

                return RunState::InteractGUI {
                    range,
                    source,
                    target,
                    prev_mouse_position: mouse_position,
                    selected_entity,
                };
            }
            VirtualKeyCode::C => {
                if selected_entity == None {
                    if tile_entities.len() > 2 {
                        {
                            selected_entity = Some(tile_entities[2].clone())
                        };
                    }
                } else if interactables.len() > 2 {
                    let interactable = interactables[2].clone();
                    let entity = entities.entity(interactable.entity_id);
                    if let Some(player) = player {
                        let mut interactions = game_state.ecs.write_storage::<InteractIntent>();
                        let _ = interactions.insert(
                            player,
                            InteractIntent::new(
                                player,
                                entity,
                                interactable.id,
                                interactable.description.clone(),
                                interactable.cost,
                            ),
                        );

                        return RunState::Ticking;
                    }
                }
                return RunState::InteractGUI {
                    range,
                    source,
                    target,
                    prev_mouse_position,
                    selected_entity,
                };
            }
            VirtualKeyCode::D => {
                if selected_entity == None {
                    if tile_entities.len() > 3 {
                        {
                            selected_entity = Some(tile_entities[3].clone())
                        };
                    }
                } else if interactables.len() > 3 {
                    let interactable = interactables[3].clone();
                    let entity = entities.entity(interactable.entity_id);
                    if let Some(player) = player {
                        let mut interactions = game_state.ecs.write_storage::<InteractIntent>();
                        let _ = interactions.insert(
                            player,
                            InteractIntent::new(
                                player,
                                entity,
                                interactable.id,
                                interactable.description.clone(),
                                interactable.cost,
                            ),
                        );

                        return RunState::Ticking;
                    }
                }
                return RunState::InteractGUI {
                    range,
                    source,
                    target,
                    prev_mouse_position,
                    selected_entity,
                };
            }
            VirtualKeyCode::E => {
                if selected_entity == None {
                    if tile_entities.len() > 4 {
                        {
                            selected_entity = Some(tile_entities[4].clone())
                        };
                    }
                } else if interactables.len() > 4 {
                    let interactable = interactables[4].clone();
                    let entity = entities.entity(interactable.entity_id);
                    if let Some(player) = player {
                        let mut interactions = game_state.ecs.write_storage::<InteractIntent>();
                        let _ = interactions.insert(
                            player,
                            InteractIntent::new(
                                player,
                                entity,
                                interactable.id,
                                interactable.description.clone(),
                                interactable.cost,
                            ),
                        );

                        return RunState::Ticking;
                    }
                }
                return RunState::InteractGUI {
                    range,
                    source,
                    target,
                    prev_mouse_position: mouse_position,
                    selected_entity,
                };
            }
            VirtualKeyCode::Return => {
                if interactables.len() > 40 {
                    let interactable = interactables[0].clone();
                    let entity = entities.entity(interactable.entity_id);
                    if let Some(player) = player {
                        let mut interactions = game_state.ecs.write_storage::<InteractIntent>();
                        let _ = interactions.insert(
                            player,
                            InteractIntent::new(
                                player,
                                entity,
                                interactable.id,
                                interactable.description.clone(),
                                interactable.cost,
                            ),
                        );

                        return RunState::Ticking;
                    }
                }
                return RunState::InteractGUI {
                    range,
                    source,
                    target,
                    prev_mouse_position: mouse_position,
                    selected_entity,
                };
            }
            _ => {
                return RunState::HandleOtherInput {
                    next_runstate: std::sync::Arc::new(RunState::InteractGUI {
                        range,
                        source,
                        target,
                        prev_mouse_position: mouse_position,
                        selected_entity,
                    }),
                    key,
                }
            }
        },
    }
}

fn check_range(
    range: usize,
    source: Vector3i,
    target: Vector3i,
    delta: Vector3i,
    mouse_position: Vector3i,
    selected_entity: Option<Entity>,
) -> RunState {
    let new_target = target + delta;
    if (source.x - new_target.x).abs() <= range as i32
        && (source.y - new_target.y).abs() <= range as i32
        && (source.z - new_target.z).abs() <= range as i32
    {
        return RunState::InteractGUI {
            range,
            source,
            target: new_target,
            prev_mouse_position: mouse_position,
            selected_entity,
        };
    } else {
        return RunState::InteractGUI {
            range,
            source,
            target,
            prev_mouse_position: mouse_position,
            selected_entity,
        };
    }
}
