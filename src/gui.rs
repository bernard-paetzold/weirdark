
use rltk::{to_char, to_cp437, Point, Rltk, VirtualKeyCode, RGB};
use specs::prelude::*;

use crate::{
    gamelog::GameLog, get_entity_interactions, get_player_entity, graphics::get_viewport_position, vectors::Vector3i, InteractIntent, Map, Name, Player, RunState, State, Viewshed, MAP_SCREEN_HEIGHT, MAP_SCREEN_WIDTH, TERMINAL_HEIGHT, TERMINAL_WIDTH
};

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuSelection { NewGame, LoadGame, Quit }

#[derive(PartialEq, Copy, Clone)]
pub enum MainMenuResult { NoSelection{ selected : MainMenuSelection }, Selected{ selected: MainMenuSelection } }

pub fn draw_ui(ecs: &World, ctx: &mut Rltk) {
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

    let mouse_position = ctx.mouse_pos();

    ctx.set(
        mouse_position.0,
        mouse_position.1,
        RGB::named(rltk::GOLD),
        RGB::named(rltk::BLACK).to_rgba(0.0),
        to_cp437('┼'),
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

    let mut tooltip: Vec<String> = Vec::new();

    if let Some(player_entity) = get_player_entity(entities, players) {
        if let Some(player_viewshed) = viewsheds.get(player_entity) {
            for (name, position) in (&names, &positions).join().filter(|&x| player_viewshed.visible_tiles.contains(x.1)) {
                if position.x == target.x && position.y == target.y {
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

/*pub fn look_gui(game_state: &mut State, ctx: &mut Rltk, target: Vector3i) -> RunState {
    let viewport_positon = get_viewport_position(&game_state.ecs);

    ctx.set_active_console(2);
    ctx.cls();
    ctx.set(
        target.x + MAP_SCREEN_WIDTH / 2 - viewport_positon.x,
        target.y + MAP_SCREEN_HEIGHT / 2 - viewport_positon.y,
        RGB::named(rltk::GOLD),
        RGB::named(rltk::BLACK).to_rgba(0.0),
        to_cp437('┼'),
    );
    draw_tooltips(&game_state.ecs, ctx, target);

    match ctx.key {
        None => return RunState::LookGUI { target },
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { return RunState::AwaitingInput },
                VirtualKeyCode::Period => return RunState::LookGUI { target: target + Vector3i::new(0, 0, -1) },
                VirtualKeyCode::Comma => return RunState::LookGUI { target: target + Vector3i::new(0, 0, 1) },
                VirtualKeyCode::Up | VirtualKeyCode::Numpad8 => return RunState::LookGUI { target: target + Vector3i::new(0, -1, 0) },
                VirtualKeyCode::Numpad9 => return RunState::LookGUI { target: target + Vector3i::new(1, -1, 0) },
                VirtualKeyCode::Right | VirtualKeyCode::Numpad6 => return RunState::LookGUI { target: target + Vector3i::new(1, 0, 0) },
                VirtualKeyCode::Numpad3 => return RunState::LookGUI { target: target + Vector3i::new(1, 1, 0) },
                VirtualKeyCode::Down | VirtualKeyCode::Numpad2 => return RunState::LookGUI { target: target + Vector3i::new(0, 1, 0) },
                VirtualKeyCode::Numpad1 => return RunState::LookGUI { target: target + Vector3i::new(-1, 1, 0) },
                VirtualKeyCode::Left | VirtualKeyCode::Numpad4 => return RunState::LookGUI { target: target + Vector3i::new(-1, 0, 0) },
                VirtualKeyCode::Numpad7 => return RunState::LookGUI { target: target + Vector3i::new(-1, -1, 0) },
                _ => return RunState::LookGUI { target }      
            }
        }
    }
}*/

pub fn interact_gui(game_state: &mut State, ctx: &mut Rltk, range: usize, source: Vector3i, target: Vector3i) -> RunState {
    let viewport_positon = get_viewport_position(&game_state.ecs);

    draw_ui(&game_state.ecs, ctx);

    ctx.set(
        target.x + MAP_SCREEN_WIDTH / 2 - viewport_positon.x,
        target.y + MAP_SCREEN_HEIGHT / 2 - viewport_positon.y,
        RGB::named(rltk::GOLD),
        RGB::named(rltk::BLACK).to_rgba(0.0),
        to_cp437('┼'),
    );
    draw_tooltips(&game_state.ecs, ctx, target);

    //Draw interact menu
    let interact_menu_width = 25;

    ctx.draw_box(
        MAP_SCREEN_WIDTH - interact_menu_width - 1,
        0,
        interact_menu_width,
        MAP_SCREEN_HEIGHT - 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );

    let map = game_state.ecs.fetch::<Map>();
    let entity = map.entities.get_by_left(&target);

    let entities = game_state.ecs.entities();
    let players = game_state.ecs.read_storage::<Player>();

    let player = get_player_entity(entities, players);

    let mut interactables: Vec<(String, String)> = Vec::new();

    if let Some(entity) = entity {
        interactables = get_entity_interactions(&game_state.ecs, *entity);

        let mut y = 0;

        for (_interaction_id, interaction_name) in interactables.iter() {
            ctx.print(MAP_SCREEN_WIDTH - interact_menu_width, y + 1, format!("<{}> {}", to_char(97 + y), interaction_name));
            y += 1;
        }
    }

    match ctx.key {
        None => return RunState::InteractGUI { range, source, target },
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => { 
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
                    if let Some(player) = player {
                        if let Some(entity) = entity {
                            if interactables.len() > 0 {
                                let mut interaction = game_state.ecs.write_storage::<InteractIntent>();
                                let _ = interaction.insert(*entity, InteractIntent::new(player, *entity, interactables[0].0.clone(), interactables[0].1.clone()));
                            }
                        }
                    }
                    return RunState::PreRun;
                },
                VirtualKeyCode::B => {
                    return RunState::PreRun;
                },
                VirtualKeyCode::Return => {
                    return RunState::PreRun;
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