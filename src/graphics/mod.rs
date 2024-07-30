use std::collections::{HashMap, HashSet};

use rltk::{Rltk, RGB, RGBA};
use specs::{prelude::*, shred::{Fetch, FetchMut}, storage::MaskedStorage};

use crate::{colors::mix_colors, vectors::Vector3i, Camera, Map, Photometry, Player, Renderable, Viewshed, TERMINAL_HEIGHT, TERMINAL_WIDTH};

pub mod components;

pub fn draw_game_screen(ctx : &mut Rltk, ecs: &mut World) {
    //Rendering
    ctx.cls();


    let viewport_position = get_viewport_position(ecs);

    draw_tiles(ctx, ecs, viewport_position);
    
}

fn draw_tiles(ctx : &mut Rltk, ecs: &mut World, viewport_position: Vector3i) {   
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
                                if tile_position.z == viewport_position.z {
                                    let foreground_color = calculate_lit_color(tile.foreground, tile.light_color, tile.light_level);

                                    ctx.set(tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2), 
                                            tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), 
                                           foreground_color, 
                                           // desaturate_color(mix_colors(tile.background, tile.light_color, tile.light_level), tile.light_level), 
                                           tile.background,
                                            tile.side_glyph);
                                }
                                else if viewport_position.z - tile_position.z == 1 {
                                    let foreground_color = calculate_lit_color(tile.foreground, tile.light_color, tile.light_level);

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
                                if tile_position.z == viewport_position.z {
                                    ctx.set(tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2), 
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), 
                                    tile.foreground.to_greyscale(), 
                                    tile.background.to_greyscale(), 
                                    tile.side_glyph);
                                }
                                else if viewport_position.z - tile_position.z == 1 {
                                    ctx.set(tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2), 
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), 
                                    tile.foreground.to_greyscale(), 
                                    tile.background.to_greyscale(),
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
            if position.z == viewport_position.z {
                ctx.set(position.x - viewport_position.x + (TERMINAL_WIDTH / 2), position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), desaturate_color(renderable.foreground, photometry.light_level), desaturate_color(renderable.background, photometry.light_level),renderable.side_glyph);
                rendered_entities.insert(position, renderable);
            }
            else if viewport_position.z - position.z == 1 {
                ctx.set(position.x - viewport_position.x + (TERMINAL_WIDTH / 2), position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), desaturate_color(renderable.foreground, photometry.light_level), desaturate_color(renderable.background, photometry.light_level), renderable.top_glyph);
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

fn desaturate_color(color: RGBA, amount: f32) -> RGBA {
    let amount = amount.clamp(0.0, 1.0);

    color.lerp(color.to_greyscale(), 1.0 - amount)
}

fn calculate_lit_color(surface_color: RGBA, light_color: RGBA, intensity: f32) -> RGBA {
    //Calculates the color a lit surface should be, taking into account the relative saturation of the surface and light colors
    desaturate_color(
        mix_colors(surface_color, 
            light_color, 
            intensity - (surface_color.to_rgb().to_hsv().s - light_color.to_rgb().to_hsv().s)), 
        intensity)
}