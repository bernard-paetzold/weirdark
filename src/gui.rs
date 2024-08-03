use rltk::{to_cp437, Point, Rltk, RGB};
use specs::prelude::*;

use crate::{
    gamelog::GameLog, get_player_entity, graphics::get_viewport_position, vectors::Vector3i, Name, Player, Viewshed, MAP_SCREEN_HEIGHT, MAP_SCREEN_WIDTH, TERMINAL_HEIGHT, TERMINAL_WIDTH
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

    draw_tooltips(ecs, ctx);
}

fn draw_tooltips(ecs: &World, ctx: &mut Rltk) {
    let entities = ecs.entities();
    let players = ecs.read_storage::<Player>();
    let viewsheds = ecs.read_storage::<Viewshed>();
    let names = ecs.read_storage::<Name>();
    let positions = ecs.read_storage::<Vector3i>();

    ctx.set_active_console(2);
    
    let mouse_pos = ctx.mouse_pos();
    if mouse_pos.0 >= MAP_SCREEN_WIDTH || mouse_pos.1 >= MAP_SCREEN_HEIGHT {
        return;
    }

    let viewport_position = get_viewport_position(&ecs);

    let mut tooltip: Vec<String> = Vec::new();

    if let Some(player_entity) = get_player_entity(entities, players) {
        if let Some(player_viewshed) = viewsheds.get(player_entity) {
            for (name, position) in (&names, &positions).join().filter(|&x| player_viewshed.visible_tiles.contains(x.1)) {
                if position.x == mouse_pos.0 - (MAP_SCREEN_WIDTH / 2) + viewport_position.x
                    && position.y == mouse_pos.1 - (MAP_SCREEN_HEIGHT / 2) + viewport_position.y
                    && viewport_position.z - position.z < player_viewshed.z_range as i32
                    && player_viewshed.visible_tiles.contains(position)
                {
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

        if mouse_pos.0 > TERMINAL_WIDTH / 2 {
            let arrow_pos = Point::new(mouse_pos.0 - 2, mouse_pos.1);

            let left_x = mouse_pos.0 - width;
            let mut y = mouse_pos.1;
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
            let arrow_pos = Point::new(mouse_pos.0 + 1, mouse_pos.1);
            let left_x = mouse_pos.0 + 3;
            let mut y = mouse_pos.1;
            for s in tooltip.iter() {
                ctx.print_color(
                    left_x + 1,
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
