use std::collections::{HashMap, HashSet};

use rltk::{Rltk, RGBA};
use specs::{prelude::*, shred::{Fetch, FetchMut}, storage::MaskedStorage};

use crate::{colors::{dim_color, mix_colors}, vectors::Vector3i, Camera, Map, Photometry, Player, Renderable, Viewshed, TERMINAL_HEIGHT, TERMINAL_WIDTH};

pub mod components;

pub fn draw_game_screen(ctx : &mut Rltk, ecs: &mut World) {
    //Rendering
    ctx.cls();


    let viewport_position = get_viewport_position(ecs);

    draw_tiles(ctx, ecs, viewport_position);
    
}

fn draw_tiles(ctx : &mut Rltk, ecs: &mut World, viewport_position: Vector3i) {   
    let discovered_tile_dimming = 0.1;

    let positions = ecs.write_storage::<Vector3i>();
    let viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let renderables = ecs.read_storage::<Renderable>();
    let photometria = ecs.read_storage::<Photometry>();

    let map = ecs.fetch::<Map>();

    for (_player, viewshed, position) in (&mut players, &viewsheds, &positions).join() {
        for x in (position.x - 1) - (TERMINAL_WIDTH / 2)..position.x + (TERMINAL_WIDTH / 2) {
            for y in (position.y - 1) - (TERMINAL_HEIGHT / 2)..position.y + (TERMINAL_WIDTH / 2) {
                for z in position.z - 2..viewport_position.z + 1 {
                    let tile_position = &Vector3i::new(x, y, z);
                    let tile = map.tiles.get(tile_position);
                    match tile {          
                        Some(tile) => {
                            if viewshed.visible_tiles.contains(&tile.position) && tile.light_level > 1.0 - viewshed.dark_vision { 
                                let foreground_color = calculate_lit_color(tile.foreground, tile.light_color, tile.light_level);                               
                                if tile_position.z == viewport_position.z {
                                    ctx.set(tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2), 
                                            tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), 
                                           foreground_color, 
                                           // desaturate_color(mix_colors(tile.background, tile.light_color, tile.light_level), tile.light_level), 
                                           tile.background,
                                            tile.side_glyph);
                                }
                                else if viewport_position.z - tile_position.z == 1 {
                                    ctx.set(tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2), 
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), 
                                    foreground_color,
                                    //mix_colors(tile.foreground, tile.light_color, tile.light_level) 
                                   // desaturate_color(mix_colors(tile.background, tile.light_color, tile.light_level), tile.light_level), 
                                    tile.background,
                                    tile.top_glyph);
                                } 
                            }
                            else if tile.discovered {
                                let foreground = dim_discovered_tile_color(tile.foreground, discovered_tile_dimming).to_greyscale();
                                let background = dim_discovered_tile_color(tile.background, discovered_tile_dimming).to_greyscale();

                                if tile_position.z == viewport_position.z {
                                    ctx.set(tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2), 
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), 
                                    foreground, 
                                    background, 
                                    tile.side_glyph);
                                }
                                else if viewport_position.z - tile_position.z == 1 {
                                    ctx.set(tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2), 
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), 
                                    foreground, 
                                    background,
                                     tile.top_glyph);
                                } 
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
        draw_entities(ctx, &positions, &renderables, &photometria, viewport_position, &viewshed.visible_tiles);
    }
}

fn draw_entities(ctx : &mut Rltk, positions: &Storage<Vector3i, FetchMut<MaskedStorage<Vector3i>>>, renderables: &Storage<Renderable, Fetch<MaskedStorage<Renderable>>>, photometria: &Storage<Photometry, Fetch<MaskedStorage<Photometry>>>, viewport_position: Vector3i, visible_tiles: &HashSet<Vector3i>) {

    let mut rendered_entities = HashMap::new();

    for (position, renderable, photometry) in (positions, renderables, photometria).join().filter(|&x| visible_tiles.contains(x.0)) {
        if !rendered_entities.contains_key(position) && !rendered_entities.contains_key(&(*position + Vector3i::new(0, 0, 1))) {
            let mut foreground_color = renderable.foreground;//calculate_lit_color(renderable.foreground, photometry.light_color, photometry.light_level);
            let background_color = renderable.background;
            foreground_color.a = photometry.light_level;


            if position.z == viewport_position.z {
                ctx.set(position.x - viewport_position.x + (TERMINAL_WIDTH / 2), position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), foreground_color, background_color,  renderable.side_glyph);
                rendered_entities.insert(position, renderable);
            }
            else if viewport_position.z - position.z == 1 {
                ctx.set(position.x - viewport_position.x + (TERMINAL_WIDTH / 2), position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), foreground_color, background_color,  renderable.top_glyph);
                rendered_entities.insert(position, renderable);
            }
        }
    }
}

pub fn get_viewport_position(ecs: &mut World) -> Vector3i {
        //Get viewport position
        let positions = ecs.read_storage::<Vector3i>();
        let cameras = ecs.read_storage::<Camera>();
        let mut viewport_position = &Vector3i::new_equi(0);
    
        for (position, camera) in (&positions, &cameras).join() {
            if camera.is_active {
                viewport_position = position;
            }
        }
        *viewport_position
}

fn calculate_lit_color(surface_color: RGBA, light_color: RGBA, intensity: f32) -> RGBA {
    mix_colors(dim_color(surface_color, intensity), dim_color(light_color, intensity), intensity - (1.0 - light_color.to_rgb().to_hsv().s))
}

pub fn dim_discovered_tile_color(color: RGBA, factor: f32) -> RGBA {
    let alpha = color.a;

    let mut color = color.to_rgb().to_hsv();

    while color.v > factor {
        color.v = color.v * factor;
    }

    return color.to_rgba(alpha);
}