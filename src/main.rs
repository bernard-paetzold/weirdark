use rltk::{Rltk, GameState};
use specs::prelude::*;

use rltk::RGB;
use vectors::Vector3i;

use crate::player::*;
use crate::map::*;
use crate::map::components::*;
use crate::camera::*;
use crate::graphics::components::*;
use crate::entities::components::*;

const TERMINAL_WIDTH: i32 = 200;
const TERMINAL_HEIGHT: i32 = 100;
const MAP_SIZE: i32 = 50;


mod states;
mod player;
mod map;
mod vectors;
mod camera;
mod entities;
mod graphics;
mod visibility_system;
use visibility_system::VisibilitySystem;


pub struct State {
    ecs: World
}

impl State {
    fn run_systems(&mut self) {
        let mut visibility_system = VisibilitySystem {};
        visibility_system.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        self.run_systems();
        player_input(self, ctx);

        //Rendering
        graphics::draw_game_screen(ctx, &mut self.ecs);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(TERMINAL_WIDTH, TERMINAL_HEIGHT)
    .unwrap()
    .with_title("Weirdark")
    .with_font("vga8x16.png", 8, 16)
    .with_sparse_console(80, 30, "vga8x16.png")
    .with_vsync(false)
    .with_title("weirdark")
    .build()?;

    let mut game_state = State{ ecs: World::new() };
    game_state.ecs.register::<Vector3i>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Tile>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Viewshed>();
    game_state.ecs.register::<Camera>();

    //Create player
    let player_start_position =Vector3i::new(0, 0, MAP_SIZE - 2);

    game_state.ecs.create_entity()
    .with(player_start_position)
    .with(Renderable::new(
        rltk::to_cp437('@'),
        rltk::to_cp437('@'),
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
    ))
    .with(Player {})
    .with(Viewshed::new(20))
    .build();

    add_camera(player_start_position, &mut game_state.ecs, true);


    game_state.ecs.create_entity()
    .with(Vector3i::new(50, 51, MAP_SIZE - 1))
    .with(Renderable::new(
        rltk::to_cp437('@'),
        rltk::to_cp437('@'),
        RGB::named(rltk::GREEN),
        RGB::named(rltk::BLACK),
    ))
    .build();

    let map = initialise_map(Vector3i::new_equi(MAP_SIZE));
    game_state.ecs.insert(map);



    rltk::main_loop(context, game_state)
}

