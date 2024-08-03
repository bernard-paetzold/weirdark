use rltk::{Rltk, VirtualKeyCode, RGB};

use crate::{gui::{MainMenuResult, MainMenuSelection}, save_load_system, RunState, State, TERMINAL_HEIGHT};



pub fn main_menu(game_state: &mut State, ctx: &mut Rltk) -> MainMenuResult {
    let runstate = game_state.ecs.fetch::<RunState>();
    let save_exists = save_load_system::does_save_exist();

    ctx.set_active_console(2);

    ctx.print_color_centered(TERMINAL_HEIGHT / 2 - 10, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Weirdark");

    if let RunState::MainMenu { menu_selection: selection } = *runstate {
        if selection == MainMenuSelection::NewGame {
            ctx.print_color_centered(TERMINAL_HEIGHT / 2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "New game");
        }
        else {
            ctx.print_color_centered(TERMINAL_HEIGHT / 2, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "New game");
        }

        if save_exists {
            if selection == MainMenuSelection::LoadGame {
                ctx.print_color_centered(TERMINAL_HEIGHT / 2 + 1, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Load game");
            }
            else {
                ctx.print_color_centered(TERMINAL_HEIGHT / 2 + 1, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Load game");
            }
        }

        if selection == MainMenuSelection::Quit {
            ctx.print_color_centered(TERMINAL_HEIGHT / 2 + 2, RGB::named(rltk::YELLOW), RGB::named(rltk::BLACK), "Quit");
        }
        else {
            ctx.print_color_centered(TERMINAL_HEIGHT / 2 + 2, RGB::named(rltk::WHITE), RGB::named(rltk::BLACK), "Quit");
        }

        match ctx.key {
            None => return MainMenuResult::NoSelection { selected: selection },
            Some(key) => {
                match key {
                    VirtualKeyCode::Escape => { return MainMenuResult::NoSelection { selected: MainMenuSelection::Quit }},
                    VirtualKeyCode::Up => {
                        let new_selection;

                        match selection {
                            MainMenuSelection::NewGame => new_selection = MainMenuSelection::Quit,
                            MainMenuSelection::LoadGame => new_selection = MainMenuSelection::NewGame,
                            MainMenuSelection::Quit => new_selection = MainMenuSelection::LoadGame,
                        }
                        return MainMenuResult::NoSelection { selected: new_selection }
                    }  
                    VirtualKeyCode::Down => {
                        let new_selection;

                        match selection {
                            MainMenuSelection::NewGame => new_selection = MainMenuSelection::LoadGame,
                            MainMenuSelection::LoadGame => new_selection = MainMenuSelection::Quit,
                            MainMenuSelection::Quit => new_selection = MainMenuSelection::NewGame,
                        }
                        return MainMenuResult::NoSelection { selected: new_selection }
                    } 
                    VirtualKeyCode::Return => return MainMenuResult::Selected{ selected : selection },
                    _ => return MainMenuResult::NoSelection{ selected: selection }, 
                }
            }
        }
    }
    MainMenuResult::NoSelection { selected: MainMenuSelection::NewGame }
}