use graphics::render_map;
use rltk::{Rltk, GameState};
use specs::prelude::*;

extern crate serde;

use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

use vectors::Vector3i;

use crate::player::*;
use crate::map::*;
use crate::map::components::*;
use crate::camera::*;
use crate::graphics::components::*;
use crate::entities::components::*;

const TERMINAL_WIDTH: i32 = 200;
const TERMINAL_HEIGHT: i32 = 100;
const MAP_SCREEN_WIDTH: i32 = 200;
const MAP_SCREEN_HEIGHT: i32 = 75;
const MAP_SIZE: i32 = 100;

const SHOW_FPS : bool = true;


mod states;
mod gui;
mod gamelog;
mod player;
mod map;
mod map_builders;
mod spawner;
mod vectors;
mod camera;
mod entities;
mod graphics;
mod colors;
mod menu;
pub mod save_load_system;

mod systems;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { 
    AwaitingInput, 
    PreRun,
    PlayerTurn,
    NPCTurn, 
    MainMenu { menu_selection: gui::MainMenuSelection },
    SaveGame,
}

pub struct State {
    ecs: World,
    dispatcher: Box<dyn systems::UnifiedDispatcher + 'static>
}

impl State {
    fn run_systems(&mut self) {
        self.dispatcher.run_now(&mut self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        
        for index in 0..3 {
            ctx.set_active_console(index);
            ctx.cls();
        }

        let mut new_runstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            new_runstate = *runstate;
        }

        match new_runstate {
            RunState::MainMenu { .. } => {
                
            }
            _ => {
                render_map(&mut self.ecs, ctx);
                gui::draw_ui(&self.ecs, ctx);
            }
        }

        match new_runstate {
            RunState::PreRun => {
                self.run_systems();
                self.ecs.maintain();
                new_runstate = RunState::AwaitingInput;
            },
            RunState::AwaitingInput => {
                new_runstate = player_input(self, ctx);
            },
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                new_runstate = RunState::AwaitingInput;
            },
            RunState::NPCTurn => {
                self.run_systems();
                self.ecs.maintain();
                new_runstate = RunState::AwaitingInput;
            },
            RunState::MainMenu { .. } => {
                let result = menu::main_menu(self, ctx);

                match result {
                    gui::MainMenuResult::NoSelection { selected } => new_runstate = RunState::MainMenu { menu_selection: selected },
                    gui::MainMenuResult::Selected { selected } => {
                        match selected {
                            gui::MainMenuSelection::NewGame => new_runstate = RunState::PreRun,
                            gui::MainMenuSelection::LoadGame => {
                                save_load_system::load_game(&mut self.ecs, ctx);
                                new_runstate = RunState::AwaitingInput;
                                //save_load_system::delete_save();
                            },
                            gui::MainMenuSelection::Quit => { 
                                ::std::process::exit(0); 
                            }
                        }
                    }
                }
            }
            RunState::SaveGame => {
                save_load_system::save_game(&mut self.ecs);
                new_runstate = RunState::MainMenu{ menu_selection : gui::MainMenuSelection::LoadGame };
            }
        }

        {
            let mut run_writer = self.ecs.write_resource::<RunState>();
            *run_writer = new_runstate;
        }
        if SHOW_FPS {
            ctx.print(1, 1, &format!("FPS: {}", ctx.fps));
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(TERMINAL_WIDTH, TERMINAL_HEIGHT)
    .unwrap()
    .with_title("Weirdark")
    .with_font("vga8x16.png", 8, 16)
    .with_sparse_console(TERMINAL_WIDTH, TERMINAL_HEIGHT, "terminal8x8.png")
    .with_sparse_console(TERMINAL_WIDTH, TERMINAL_HEIGHT, "terminal8x8.png")
    .with_vsync(false)
    .with_title("weirdark")
    .build()?;

    let mut game_state = State{ 
        ecs: World::new(),
        dispatcher: systems::build(),
     };

    game_state.ecs.register::<Vector3i>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Tile>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Name>();
    game_state.ecs.register::<Viewshed>();
    game_state.ecs.register::<Camera>();
    game_state.ecs.register::<Illuminant>();
    game_state.ecs.register::<Photometry>();
    game_state.ecs.register::<SimpleMarker<SerializeThis>>();
    game_state.ecs.register::<SerializationHelper>();

    game_state.ecs.insert(SimpleMarkerAllocator::<SerializeThis>::new());

    let map = map_builders::build_system_test_map(Vector3i::new(MAP_SIZE, MAP_SIZE, 5));
    game_state.ecs.insert(map);
    game_state.ecs.insert(gamelog::GameLog{ entries : vec!["Game log".to_string()] });
    game_state.ecs.insert(RunState::MainMenu { menu_selection: gui::MainMenuSelection::NewGame });

    //Create player
    let player_start_position = Vector3i::new(2, 1, 1);
    let player_entity = spawner::player(&mut game_state.ecs, player_start_position);

    //Add a light
     spawner::standing_lamp(&mut game_state.ecs, Vector3i::new(-6, 6, player_start_position.z));

    game_state.ecs.insert(player_start_position);
    game_state.ecs.insert(player_entity);

    rltk::main_loop(context, game_state)
}

