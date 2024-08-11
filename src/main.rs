
use graphics::render_map;
use rltk::{GameState, Rltk};
use specs::prelude::*;

extern crate serde;

use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};

use states::RunState;
use vectors::Vector3i;

use crate::camera::*;
use crate::entities::components::*;
use crate::graphics::components::*;
use crate::map::components::*;
use crate::map::*;
use crate::player::*;

const TERMINAL_WIDTH: i32 = 200;
const TERMINAL_HEIGHT: i32 = 100;
const MAP_SCREEN_WIDTH: i32 = 200;
const MAP_SCREEN_HEIGHT: i32 = 75;
const MAP_SIZE: i32 = 100;

const SHOW_FPS: bool = true;

mod camera;
mod colors;
mod entities;
mod gamelog;
mod graphics;
mod gui;
mod map;
mod map_builders;
mod menu;
mod player;
pub mod save_load_system;
mod spawner;
mod states;
mod vectors;
pub mod rng;

mod systems;
pub struct State {
    ecs: World,
    dispatcher: Box<dyn systems::UnifiedDispatcher + 'static>,
}

impl State {
    fn run_systems(&mut self) {
        use std::time::Instant;
        let now = Instant::now();

        self.dispatcher.run_now(&mut self.ecs);


        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
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
            RunState::MainMenu { .. } => {}
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
            }
            RunState::AwaitingInput => {
                new_runstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                new_runstate = RunState::AwaitingInput;
            }
            RunState::NPCTurn => {
                self.run_systems();
                self.ecs.maintain();
                new_runstate = RunState::AwaitingInput;
            }
            RunState::MainMenu { .. } => {
                let result = menu::main_menu(self, ctx);

                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        new_runstate = RunState::MainMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::MainMenuResult::Selected { selected } => {
                        match selected {
                            gui::MainMenuSelection::NewGame => new_runstate = RunState::PreRun,
                            gui::MainMenuSelection::LoadGame => {
                                save_load_system::load_game(&mut self.ecs, ctx);
                                new_runstate = RunState::AwaitingInput;
                                //save_load_system::delete_save();
                            }
                            gui::MainMenuSelection::Quit => {
                                ::std::process::exit(0);
                            }
                        }
                    }
                }
            }
            RunState::SaveGame => {
                save_load_system::save_game(&mut self.ecs);
                new_runstate = RunState::MainMenu {
                    menu_selection: gui::MainMenuSelection::LoadGame,
                };
            }
            RunState::InteractGUI {
                range,
                source,
                target,
            } => {
                new_runstate = gui::interact_gui(self, ctx, range, source, target);

                //If the gui exits snap the camera position to the player
                match new_runstate {
                    RunState::AwaitingInput { .. } | RunState::PreRun => {
                        crate::player::reset_camera_position(&mut self.ecs);
                    }
                    _ => {}
                }
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
        //.with_font("vga8x16.png", 8, 16)
        .with_sparse_console(TERMINAL_WIDTH, TERMINAL_HEIGHT, "terminal8x8.png")
        .with_sparse_console(TERMINAL_WIDTH, TERMINAL_HEIGHT, "terminal8x8.png")
        .with_title("weirdark")
        .with_vsync(false)
        .build()?;

    let mut game_state = State {
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
    game_state.ecs.register::<InteractIntent>();
    game_state.ecs.register::<Blocker>();
    game_state.ecs.register::<VisionBlocker>();
    game_state.ecs.register::<Door>();
    game_state.ecs.register::<PowerSource>();
    game_state.ecs.register::<Wire>();
    game_state.ecs.register::<Duct>();
    game_state.ecs.register::<EntityDirection>();

    //Power
    game_state.ecs.register::<PoweredState>();
    game_state.ecs.register::<PowerSwitch>();
    game_state.ecs.register::<PowerNode>();


    let player_start_position = Vector3i::new(0, 0, 10);

    game_state.ecs.insert(SimpleMarkerAllocator::<SerializeThis>::new());

    let mut builder = map_builders::build_system_test_map(
        Vector3i::new(MAP_SIZE, MAP_SIZE, 5),
        player_start_position + Vector3i::new(0, 0, -1),
    );
    builder.build_map();
    //builder.spawn_entities(&mut game_state.ecs);
    let map = builder.get_map();

    game_state.ecs.insert(map);
    builder.spawn_entities(&mut game_state.ecs);

    
    game_state.ecs.insert(gamelog::GameLog {
        entries: vec!["Game log".to_string()],
    });
    game_state.ecs.insert(RunState::MainMenu {
        menu_selection: gui::MainMenuSelection::NewGame,
    });

    //Create player
    let player_entity = spawner::player(&mut game_state.ecs, player_start_position);

    game_state.ecs.insert(player_start_position);
    game_state.ecs.insert(player_entity);

    rltk::main_loop(context, game_state)
}
