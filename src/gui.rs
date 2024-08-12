
use std::u32::MAX;

use rltk::{to_char, Point, Rltk, VirtualKeyCode, RGB};
use crate::graphics::char_to_glyph;
use crate::systems::power_system::get_devices_on_network;
use crate::{systems::interaction_system::get_entity_interactions, Renderable};
use crate::entities::power_components::{BreakerBox, PowerNode, PowerSource, PowerSwitch, PoweredState, Wire};
use specs::prelude::*;

use crate::{
    gamelog::GameLog, get_player_entity, graphics::get_viewport_position, vectors::Vector3i, InteractIntent, Map, Name, Player, RunState, State, Viewshed, MAP_SCREEN_HEIGHT, MAP_SCREEN_WIDTH, TERMINAL_HEIGHT, TERMINAL_WIDTH
};

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection { NewGame, LoadGame, Quit }

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult { NoSelection{ selected : MainMenuSelection }, Selected{ selected: MainMenuSelection } }

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
    ctx.set_active_console(2);
    //ctx.set_translation_mode(2, rltk::CharacterTranslationMode::Codepage437);
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

    //let player_pos = ecs.fetch::<Vector3i>();
    //let viewport_position = get_viewport_position(&ecs);

    //let map_mouse_position = Vector3i::new(mouse_pos.0  - (MAP_SCREEN_WIDTH / 2) + viewport_position.x, mouse_pos.1  - (MAP_SCREEN_HEIGHT / 2) + viewport_position.y, player_pos.z);


   // draw_tooltips(ecs, ctx, map_mouse_position);
}

pub fn draw_tooltips(ecs: &World, ctx: &mut Rltk, target: Vector3i) {
    let entities = ecs.entities();
    let players = ecs.read_storage::<Player>();
    let viewsheds = ecs.read_storage::<Viewshed>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Vector3i>();

    let viewport_position = get_viewport_position(&ecs);

    let screen_position = Vector3i::new(target.x  + MAP_SCREEN_WIDTH / 2 - viewport_position.x, target.y  + MAP_SCREEN_HEIGHT / 2 - viewport_position.y, target.z);

    ctx.set_active_console(2);
    //ctx.set_translation_mode(2, rltk::CharacterTranslationMode::Codepage437);

    let mut tooltip: Vec<String> = Vec::new();

    if let Some(player_entity) = get_player_entity(&entities, &players) {
        if let Some(player_viewshed) = viewsheds.get(player_entity) {
            for (name, position) in (&names, &positions).join().filter(|&x| player_viewshed.visible_tiles.contains(x.1)) {
                if position.x == target.x && position.y == target.y  && (position.z == target.z) {
                    if position.z < viewport_position.z {
                        tooltip.push((name.name.to_string() + " (below)").to_string());
                    }
                    else {
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

pub fn interact_gui(game_state: &mut State, ctx: &mut Rltk, range: usize, source: Vector3i, target: Vector3i) -> RunState {
    crate::set_camera_position(target, &mut game_state.ecs);

    let entities = game_state.ecs.entities();
    
    let viewport_positon = get_viewport_position(&game_state.ecs);

    draw_ui(&game_state.ecs, ctx);
    //ctx.set_translation_mode(2, rltk::CharacterTranslationMode::Codepage437);

    ctx.set(
        target.x + MAP_SCREEN_WIDTH / 2 - viewport_positon.x,
        target.y + MAP_SCREEN_HEIGHT / 2 - viewport_positon.y,
        RGB::named(rltk::GOLD),
        RGB::named(rltk::BLACK).to_rgba(0.0),
        char_to_glyph('┼'),
    );
    draw_tooltips(&game_state.ecs, ctx, target);

    //Draw interact menu
    const INTERACT_MENU_WIDTH: i32 = 35;

    ctx.draw_box(
        MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH - 1,
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

    let player = get_player_entity(&entities, &players);

    let mut interactables: Vec<(usize, String, u32, u32)> = Vec::new();
    let mut tile_entities: Vec<Entity> = Vec::new();

    let target_tile = map.tiles.get(&target);

    if let Some(player_entity) = player {
        if let Some(player_viewshed) = viewsheds.get(player_entity) {
            for (entity, position) in (&entities, &positions).join().filter(|&x| player_viewshed.visible_tiles.contains(x.1)) {
                //if position.x == target.x && position.y == target.y && (position.z == target.z || position.z == target.z - 1) {                
                if position.x == target.x && position.y == target.y && (position.z == target.z) {                
                    interactables.append(&mut get_entity_interactions(&game_state.ecs, entity));
                    tile_entities.push(entity);


                    //Handle breaker boxes or other entities that control interactions off tile
                    if let Some(_) = breaker_boxes.get(entity) {
                        interactables.append(&mut get_devices_on_network(&game_state.ecs, entity));
                    }
                }
            }
        }
    }

    let mut entity_menu_y = 0;
    let mut interactable_menu_y = 0;

    let tile_info_y = 1;
    const TILE_INFORMATION_MENU_HEIGHT: i32 = 10;

    if let Some(target_tile) = target_tile {
        entity_menu_y = TILE_INFORMATION_MENU_HEIGHT + 2;
        interactable_menu_y = TILE_INFORMATION_MENU_HEIGHT + 2;

        ctx.draw_hollow_box(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH, tile_info_y, INTERACT_MENU_WIDTH - 2, TILE_INFORMATION_MENU_HEIGHT, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
        ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, tile_info_y + 1, format!("{}", target_tile.name.clone()));
        ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, tile_info_y + 2, format!("Glyphs: {}, {}", to_char(target_tile.renderable.top_glyph as u8), to_char(target_tile.renderable.side_glyph as u8)));
        ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, tile_info_y + 3, format!("Light level: {:.2}", target_tile.photometry.light_level));
        ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, tile_info_y + 4, format!("Color: ({:.2},{:.2},{:.2})", target_tile.renderable.foreground.r, target_tile.renderable.foreground.g, target_tile.renderable.foreground.b));
    }

    const ENTITY_FIELDS: usize = 8;

    if tile_entities.len() > 0 {
        let mut render_menu = false;
        let entity_menu_height = tile_entities.len() * ENTITY_FIELDS;    
        let mut y = 3;
        let mut prev_id = MAX;

        for entity in tile_entities.iter() {
            let renderable = renderables.get(*entity);
            let name = names.get(*entity);

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
                    ctx.print_color(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, 
                        entity_menu_y + y, color, RGB::named(rltk::BLACK), format!("{}", entity_name));
                    y += 2;

                    render_menu = true;
                }

                prev_id = entity.id();
                    
            }
            if let Some(renderable) = renderables.get(*entity) {
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Glyphs: {}, {}", to_char(renderable.top_glyph as u8), to_char(renderable.side_glyph as u8)));
                y += 1;
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Color: ({:.2},{:.2},{:.2})", renderable.foreground.r, renderable.foreground.g, renderable.foreground.b));
                y += 1;
            }
            if let Some(powered_state) = power_states.get(*entity) {
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Power on: {}", powered_state.state_description()));
                y += 1;
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Power available: {}", powered_state.available_wattage));
                y += 1;
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Power draw: {}", powered_state.wattage));
                y += 1;

            }
            if let Some(wire) = wires.get(*entity) {
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Power available: {}", wire.available_wattage));
                y += 1;
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Power load: {}", wire.power_load));
                y += 1;
            }
            if let Some(power_switch) = power_switches.get(*entity) {
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Power switch: {}", power_switch.state_description()));
                y += 1;
            }
            if let Some(power_source) = power_sources.get(*entity) {
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Power capacity: {}", power_source.max_wattage));
                y += 1;
            }
            if let Some(node) = nodes.get(*entity) {
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, entity_menu_y + y, format!("Network: {}", node.network_id));
                y += 1;
            }
            
        }

        if render_menu {
            ctx.draw_hollow_box(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH, entity_menu_y, INTERACT_MENU_WIDTH - 2, entity_menu_height, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
            ctx.print_color(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, interactable_menu_y + 1, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Entities:");
            interactable_menu_y += entity_menu_height as i32 + 2;
        }
    }

    if interactables.len() > 0 {
        let interaction_menu_height = (tile_entities.len() * 4) + (interactables.len());

        ctx.draw_hollow_box(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH, interactable_menu_y, INTERACT_MENU_WIDTH - 2, interaction_menu_height, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK));
        ctx.print_color(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, interactable_menu_y + 1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Interactables:");
        
        let mut y = 3;
        let mut count = 0;
        let mut prev_id = MAX;

        for (_interaction_id, interaction_name, listing_id, entity_id) in interactables.iter() {
            let interactable_entity = entities.entity(*entity_id);
            let renderable = renderables.get(interactable_entity);
            let name = names.get(interactable_entity);

            if y < 50 {
                if *listing_id != prev_id {
                    let mut color = RGB::named(rltk::WHITE).to_rgba(1.0);
                    let mut entity_name = "{unknown}".to_string();

                    if let Some(renderable) = renderable {
                        color = renderable.foreground;
                    }
                    if let Some(name) = name {
                        entity_name = name.name.to_string();
                    }
                    y += 1;
                        ctx.print_color(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 1, 
                            interactable_menu_y + y, color, RGB::named(rltk::BLACK), format!("{}:", entity_name));

                    y += 2;
                    prev_id = *listing_id;
                        
                }
                ctx.print(MAP_SCREEN_WIDTH - INTERACT_MENU_WIDTH + 2, interactable_menu_y + y, format!("<{}> {}", to_char(97 + count), format!("{}", interaction_name)));
                y += 1;
                count += 1;
            }
        }
    }

    match ctx.key {
        None => return RunState::InteractGUI { range, source, target },
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { 
                    return RunState::AwaitingInput 
                },
                VirtualKeyCode::I => { 
                    return RunState::AwaitingInput 
                },
                VirtualKeyCode::Period => {
                    return check_range(range, source, target, Vector3i::new(0, 0, -1));
                },
                VirtualKeyCode::Comma => {
                    return check_range(range, source, target, Vector3i::new(0, 0, 1));
                },
                VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => {
                    return check_range(range, source, target, Vector3i::new(0, -1, 0));
                },
                VirtualKeyCode::Numpad9 => {
                    return check_range(range, source, target, Vector3i::new(1, -1, 0));
                },
                VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => {
                    return check_range(range, source, target, Vector3i::new(1, 0, 0));
                },
                VirtualKeyCode::Numpad3 => {
                    return check_range(range, source, target, Vector3i::new(1, 1, 0));
                },
                VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => {
                    return check_range(range, source, target, Vector3i::new(0, 1, 0));
                },
                VirtualKeyCode::Numpad1 => {
                    return check_range(range, source, target, Vector3i::new(-1, 1, 0));
                },
                VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => {
                    return check_range(range, source, target, Vector3i::new(-1, 0, 0));
                },
                VirtualKeyCode::Numpad7 => {
                    return check_range(range, source, target, Vector3i::new(-1, -1, 0));
                },
                VirtualKeyCode::A => {
                    if interactables.len() > 0 {
                        let interactable = interactables[0].clone();
                        let entity = entities.entity(interactable.3);
                        if let Some(player) = player {
                            let mut interaction = game_state.ecs.write_storage::<InteractIntent>();
                            let _ = interaction.insert(entity, InteractIntent::new(player, entity, interactable.0, interactable.1.clone()));
                        
                            return RunState::PreRun; 
                        }       
                    }
                    return RunState::InteractGUI { range, source, target }
                },
                VirtualKeyCode::B => { 
                    if interactables.len() > 1 {
                        let interactable = interactables[1].clone();
                        let entity = entities.entity(interactable.3);
                        if let Some(player) = player {
                            let mut interaction = game_state.ecs.write_storage::<InteractIntent>();
                            let _ = interaction.insert(entity, InteractIntent::new(player, entity, interactable.0, interactable.1.clone()));
                        
                            return RunState::PreRun; 
                        }       
                    }
                    return RunState::InteractGUI { range, source, target }
                },
                VirtualKeyCode::C => { 
                    if interactables.len() > 2 {
                        let interactable = interactables[2].clone();
                        let entity = entities.entity(interactable.3);
                        if let Some(player) = player {
                            let mut interaction = game_state.ecs.write_storage::<InteractIntent>();
                            let _ = interaction.insert(entity, InteractIntent::new(player, entity, interactable.0, interactable.1.clone()));
                        
                            return RunState::PreRun; 
                        }       
                    }
                    return RunState::InteractGUI { range, source, target }
                },
                VirtualKeyCode::D => { 
                    if interactables.len() > 3 {
                        let interactable = interactables[3].clone();
                        let entity = entities.entity(interactable.3);
                        if let Some(player) = player {
                            let mut interaction = game_state.ecs.write_storage::<InteractIntent>();
                            let _ = interaction.insert(entity, InteractIntent::new(player, entity, interactable.0, interactable.1.clone()));
                        
                            return RunState::PreRun; 
                        }       
                    }
                    return RunState::InteractGUI { range, source, target }
                },
                VirtualKeyCode::E => { 
                    if interactables.len() > 4 {
                        let interactable = interactables[4].clone();
                        let entity = entities.entity(interactable.3);
                        if let Some(player) = player {
                            let mut interaction = game_state.ecs.write_storage::<InteractIntent>();
                            let _ = interaction.insert(entity, InteractIntent::new(player, entity, interactable.0, interactable.1.clone()));
                        
                            return RunState::PreRun; 
                        }       
                    }
                    return RunState::InteractGUI { range, source, target }
                },
                _ => return RunState::InteractGUI { range, source, target }
                
            }
        }
    }
}

fn check_range(range: usize, source: Vector3i, target: Vector3i, delta: Vector3i) -> RunState {
    let new_target = target + delta;
    if (source.x - new_target.x).abs() <= range as i32 && (source.y - new_target.y).abs() <= range as i32 && (source.z - new_target.z).abs() <= range as i32 {
        return RunState::InteractGUI { range, source, target: new_target }
    }
    else {
        return RunState::InteractGUI { range, source, target }
    }
}