use rltk::prelude::*;
use specs::{prelude::*, storage::GenericReadStorage};

use crate::{
    entities::intents::{DropIntent, InteractIntent, OpenIntent, PickUpIntent},
    gui::{interact_gui, MainMenuResult, MainMenuSelection},
    save_load_system,
    systems::event_system::{
        get_default_interactions, get_entity_interactions, InteractionInformation, InteractionType,
    },
    vectors::Vector3i,
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
    secondary_menu: bool,
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

                let mut char_offset = 97;
                if secondary_menu {
                    char_offset = 65;
                }

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
                            to_char(char_offset + count),
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
    Action,
}

pub fn show_inventory(
    game_state: &mut State,
    ctx: &mut Rltk,
    container_id: u32,
    selected_item: Option<Entity>,
) -> (ItemMenuResult, Option<Entity>) {
    let names = game_state.ecs.read_storage::<Name>();
    let in_container = game_state.ecs.read_storage::<InContainer>();
    let positions = game_state.ecs.read_storage::<Vector3i>();
    let entities = game_state.ecs.entities();

    let mut items = Vec::new();

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
    for (_, name, entity) in (&in_container, &names, &entities)
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

        items.push(entity);
    }
    if let Some(item) = selected_item {
        let mut interactables = get_default_interactions(&game_state.ecs, item);
        interactables.append(&mut get_entity_interactions(&game_state.ecs, item));

        interaction_menu(
            &game_state.ecs,
            ctx,
            interactables.clone(),
            Point::new(MAP_SCREEN_WIDTH, 10),
            true,
        );

        let player = *game_state.ecs.fetch::<Entity>();

        return match ctx.key {
            None => (ItemMenuResult::NoResponse, None),
            Some(key) => {
                if key >= VirtualKeyCode::A && key <= VirtualKeyCode::Z && ctx.shift {
                    let selection = letter_to_option(key);
                    if let Some(interactable) = interactables.get(selection as usize) {
                        let entity = entities.entity(interactable.entity_id);
                        match interactable.interaction_type {
                            InteractionType::Component => {
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
                            InteractionType::PickUp => {
                                let mut pick_up_intents =
                                    game_state.ecs.write_storage::<PickUpIntent>();
                                let _ = pick_up_intents.insert(
                                    player,
                                    PickUpIntent::new(player, entity, interactable.cost),
                                );
                            }
                            InteractionType::Open => {
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
                            InteractionType::Drop => {
                                let mut open_intents = game_state.ecs.write_storage::<DropIntent>();
                                let _ = open_intents.insert(
                                    player,
                                    DropIntent::new(player, entity, interactable.cost),
                                );
                            }
                        }
                        return (ItemMenuResult::Action, None);
                    }
                    (ItemMenuResult::NoResponse, None)
                } else if key == VirtualKeyCode::Escape {
                    (ItemMenuResult::Cancel, None)
                } else {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        if let Some(item) = items.get(selection as usize) {
                            (ItemMenuResult::Selected, Some(*item))
                        } else {
                            (ItemMenuResult::NoResponse, None)
                        }
                    } else {
                        (ItemMenuResult::NoResponse, None)
                    }
                }
            }
        };
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                let item = items.get(selection as usize);

                if let Some(item) = item {
                    if selection > -1 && selection < count as i32 {
                        return (ItemMenuResult::Selected, Some(*item));
                    }
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}
