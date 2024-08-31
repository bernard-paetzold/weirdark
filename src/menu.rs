use rltk::prelude::*;
use specs::prelude::*;

use crate::{
    gui::{interact_gui, MainMenuResult, MainMenuSelection},
    save_load_system,
    systems::event_system::InteractionInformation,
    InContainer, Name, Renderable, RunState, State, INTERACT_MENU_WIDTH, MAP_SCREEN_WIDTH,
    TERMINAL_HEIGHT,
};

pub fn main_menu(game_state: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let runstate = game_state.ecs.fetch::<RunState>();
    let save_exists = save_load_system::does_save_exist();

    ctx.set_active_console(2);

    ctx.print_color_centered(
        TERMINAL_HEIGHT / 2 - 10,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Weirdark",
    );

    if let RunState::MainMenu {
        menu_selection: selection,
    } = *runstate
    {
        if selection == MainMenuSelection::NewGame {
            ctx.print_color_centered(
                TERMINAL_HEIGHT / 2,
                RGB::named(rltk::YELLOW),
                RGB::named(rltk::BLACK),
                "New game",
            );
        } else {
            ctx.print_color_centered(
                TERMINAL_HEIGHT / 2,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "New game",
            );
        }

        if save_exists {
            if selection == MainMenuSelection::LoadGame {
                ctx.print_color_centered(
                    TERMINAL_HEIGHT / 2 + 1,
                    RGB::named(rltk::YELLOW),
                    RGB::named(rltk::BLACK),
                    "Load game",
                );
            } else {
                ctx.print_color_centered(
                    TERMINAL_HEIGHT / 2 + 1,
                    RGB::named(rltk::WHITE),
                    RGB::named(rltk::BLACK),
                    "Load game",
                );
            }
        }

        if selection == MainMenuSelection::Quit {
            ctx.print_color_centered(
                TERMINAL_HEIGHT / 2 + 2,
                RGB::named(rltk::YELLOW),
                RGB::named(rltk::BLACK),
                "Quit",
            );
        } else {
            ctx.print_color_centered(
                TERMINAL_HEIGHT / 2 + 2,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                "Quit",
            );
        }

        match ctx.key {
            None => {
                return MainMenuResult::NoSelection {
                    selected: selection,
                }
            }
            Some(key) => match key {
                VirtualKeyCode::Escape => {
                    return MainMenuResult::NoSelection {
                        selected: MainMenuSelection::Quit,
                    }
                }
                VirtualKeyCode::Up => {
                    let new_selection;

                    match selection {
                        MainMenuSelection::NewGame => new_selection = MainMenuSelection::Quit,
                        MainMenuSelection::LoadGame => new_selection = MainMenuSelection::NewGame,
                        MainMenuSelection::Quit => {
                            if save_exists {
                                new_selection = MainMenuSelection::LoadGame
                            } else {
                                new_selection = MainMenuSelection::NewGame
                            }
                        }
                    }
                    return MainMenuResult::NoSelection {
                        selected: new_selection,
                    };
                }
                VirtualKeyCode::Down => {
                    let new_selection;

                    match selection {
                        MainMenuSelection::NewGame => {
                            if save_exists {
                                new_selection = MainMenuSelection::LoadGame
                            } else {
                                new_selection = MainMenuSelection::Quit
                            }
                        }
                        MainMenuSelection::LoadGame => new_selection = MainMenuSelection::Quit,
                        MainMenuSelection::Quit => new_selection = MainMenuSelection::NewGame,
                    }
                    return MainMenuResult::NoSelection {
                        selected: new_selection,
                    };
                }
                VirtualKeyCode::Return => {
                    return MainMenuResult::Selected {
                        selected: selection,
                    }
                }
                _ => {
                    return MainMenuResult::NoSelection {
                        selected: selection,
                    }
                }
            },
        }
    }
    MainMenuResult::NoSelection {
        selected: MainMenuSelection::NewGame,
    }
}

pub fn interaction_menu(
    ecs: &World,
    ctx: &mut Rltk,
    interactables: Vec<InteractionInformation>,
    position: Point,
) {
    let entities = ecs.entities();
    let renderables = ecs.read_storage::<Renderable>();
    let names = ecs.read_storage::<Name>();

    if interactables.len() > 0 {
        let interaction_menu_height = interactables.len() + 3;

        let interactable_menu_y = position.y;

        if interactable_menu_y < TERMINAL_HEIGHT {
            ctx.draw_hollow_box(
                MAP_SCREEN_WIDTH,
                interactable_menu_y,
                INTERACT_MENU_WIDTH - 2,
                interaction_menu_height,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
            );
            ctx.print_color(
                MAP_SCREEN_WIDTH + 1,
                interactable_menu_y + 1,
                RGB::named(rltk::YELLOW),
                RGB::named(rltk::BLACK),
                "Interactables:",
            );

            let mut y = 3;
            let mut count = 0;
            let mut prev_id = std::u32::MAX;

            for interaction_information in interactables.iter() {
                let interactable_entity = entities.entity(interaction_information.entity_id);
                let renderable = renderables.get(interactable_entity);
                let name = names.get(interactable_entity);

                if y < 50 {
                    if interaction_information.entity_id != prev_id {
                        let mut color = RGB::named(rltk::WHITE).to_rgba(1.0);
                        let mut entity_name = "{unknown}".to_string();

                        if let Some(renderable) = renderable {
                            color = renderable.foreground;
                        }
                        if let Some(name) = name {
                            entity_name = name.name.to_string();
                        }
                        y += 1;
                        ctx.print_color(
                            MAP_SCREEN_WIDTH + 1,
                            interactable_menu_y + y,
                            color,
                            RGB::named(rltk::BLACK),
                            format!("{}:", entity_name),
                        );

                        y += 2;
                        prev_id = interaction_information.entity_id;
                    }
                    ctx.print(
                        MAP_SCREEN_WIDTH + 2,
                        interactable_menu_y + y,
                        format!(
                            "<{}> {}",
                            to_char(97 + count),
                            format!("{}", interaction_information.description)
                        ),
                    );
                    y += 1;
                    count += 1;
                }
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn show_inventory(
    game_state: &mut State,
    ctx: &mut Rltk,
    container_id: u32,
    selected_item: Option<Entity>,
) -> ItemMenuResult {
    let names = game_state.ecs.read_storage::<Name>();
    let in_container = game_state.ecs.read_storage::<InContainer>();

    let inventory = (&in_container, &names)
        .join()
        .filter(|(in_container, name)| in_container.owner == container_id);
    let count = inventory.count();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Inventory",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    let mut j = 0;
    for (_, name) in (&in_container, &names)
        .join()
        .filter(|item| item.0.owner == container_id)
    {
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            97 + j as rltk::FontCharType,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );

        ctx.print(21, y, &name.name.to_string());
        y += 1;
        j += 1;
    }

    /*if let Some(item) = selected_item {

    }*/

    match ctx.key {
        None => ItemMenuResult::NoResponse,
        Some(key) => match key {
            VirtualKeyCode::Escape => ItemMenuResult::Cancel,
            _ => ItemMenuResult::NoResponse,
        },
    }
}
