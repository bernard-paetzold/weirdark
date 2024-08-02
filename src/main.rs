use std::f32::consts::PI;

use graphics::get_viewport_position;
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
const MAP_SIZE: i32 = 100;


mod states;
mod player;
mod map;
mod vectors;
mod camera;
mod entities;
mod graphics;
mod visibility_system;
mod lighting_system;
mod colors;

use visibility_system::VisibilitySystem;
use lighting_system::LightingSystem;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState { Paused, Running }

pub struct State {
    ecs: World,
    pub run_state: RunState,
}

impl State {
    fn run_systems(&mut self) {
        let mut visibility_system = VisibilitySystem {};
        let mut lighting_system = LightingSystem {};
        visibility_system.run_now(&self.ecs);
        lighting_system.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        if self.run_state == RunState::Running {
            self.run_systems();
            
            //Rendering
            let viewport_position = get_viewport_position(&mut self.ecs);

            let now = std::time::Instant::now();

            ctx.set_active_console(0);
            ctx.cls();

            graphics::draw_tiles(ctx, &mut self.ecs, viewport_position);
            rltk::render_draw_buffer(ctx).expect("Draw error");

            ctx.set_active_console(1);
            ctx.cls();

            graphics::draw_entities(ctx, &mut self.ecs, viewport_position);
            rltk::render_draw_buffer(ctx).expect("Draw error");

            let elapsed = now.elapsed();
            println!("Drawing: {:.2?}", elapsed);

            self.run_state = RunState::Paused;
        }
        else {
            self.run_state = player_input(self, ctx);
        }
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple(TERMINAL_WIDTH, TERMINAL_HEIGHT)
    .unwrap()
    .with_title("Weirdark")
    .with_font("vga8x16.png", 8, 16)
    //.with_sparse_console(TERMINAL_WIDTH, TERMINAL_HEIGHT, "vga8x16.png")
    .with_sparse_console(TERMINAL_WIDTH, TERMINAL_HEIGHT, "terminal8x8.png")
    .with_vsync(false)
    .with_title("weirdark")
    .build()?;

    let mut game_state = State{ ecs: World::new(), run_state: RunState::Running };
    game_state.ecs.register::<Vector3i>();
    game_state.ecs.register::<Renderable>();
    game_state.ecs.register::<Tile>();
    game_state.ecs.register::<Player>();
    game_state.ecs.register::<Viewshed>();
    game_state.ecs.register::<Camera>();
    game_state.ecs.register::<Illuminant>();
    game_state.ecs.register::<Photometry>();

    //Create player
    let player_start_position =Vector3i::new(2, 1, 1);

    game_state.ecs.create_entity()
    .with(player_start_position)
    .with(Renderable::new(
        rltk::to_cp437('@'),
        rltk::to_cp437('@'),
        RGB::named(rltk::YELLOW).to_rgba(1.0),
        RGB::named(rltk::BLACK).to_rgba(1.0),
    ))
    .with(Player::new())
    .with(Viewshed::new(20, 3, 0.9))
    .with(Photometry::new())
    .with(Illuminant::new(1.0, 5, RGB::named(rltk::WHITE).to_rgba(1.0), PI * 2.0, false))
    .build();

    add_camera(player_start_position, &mut game_state.ecs, true);


    game_state.ecs.create_entity()
    .with(player_start_position + Vector3i::new(5, -15, 0))
    .with(Renderable::new(
        rltk::to_cp437('☼'),
        rltk::to_cp437('☼'),
        RGB::named(rltk::YELLOW).to_rgba(1.0),
        RGB::named(rltk::BLACK).to_rgba(0.0),
    ))
    .with(Viewshed::new(60, 3, 1.0))
    .with(Photometry::new())
    .with(Illuminant::new(1.5, 15, RGB::named(rltk::BLUE).to_rgba(1.0), PI * 2.0, true))
    .build();

    game_state.ecs.create_entity()
    .with(player_start_position + Vector3i::new(10, 5, 0))
    .with(Renderable::new(
        rltk::to_cp437('☼'),
        rltk::to_cp437('☼'),
        RGB::named(rltk::YELLOW).to_rgba(1.0),
        RGB::named(rltk::BLACK).to_rgba(0.0),
    ))
    .with(Viewshed::new(60, 3, 1.0))
    .with(Photometry::new())
    .with(Illuminant::new(1.5, 15, RGB::named(rltk::RED).to_rgba(1.0), PI * 2.0, true))
    .build();

    let map = initialise_map(Vector3i::new_equi(MAP_SIZE));
    game_state.ecs.insert(map);



    rltk::main_loop(context, game_state)
}

