use std::collections::HashMap;

use rltk::Rltk;
use specs::prelude::*;

use crate::{vectors::Vector3i, Camera, Map, Player, Renderable, Viewshed, TERMINAL_HEIGHT, TERMINAL_WIDTH};

pub mod components;

pub fn draw_game_screen(ctx : &mut Rltk, ecs: &mut World) {
    //Rendering
    ctx.cls();


    let viewport_position = get_viewport_position(ecs);

    draw_tiles(ctx, ecs, viewport_position);
    draw_entities(ctx, ecs, viewport_position);
            
    

    
}

fn draw_tiles(ctx : &mut Rltk, ecs: &mut World, viewport_position: Vector3i) {   
    let mut positions = ecs.write_storage::<Vector3i>();
    let viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();

    let map = ecs.fetch::<Map>();

    /*for (_player, viewshed, position) in (&mut players, &viewsheds, &mut positions).join() {
        //TODO: Add Viewshed systems
        for x in (position.x - viewshed.view_distance)..(position.x + viewshed.view_distance) {
            for y in (position.y - viewshed.view_distance)..(position.y + viewshed.view_distance) {
                for z in position.z..position.z + 1 {
                    let tile = map.tiles.get(&Vector3i::new(x, y, z));

                    match tile {
                        Some(tile) => {
                            if viewshed.visible_tiles.contains(&tile.position) {  
                                println!("{}", position.z);
                                if position.z == viewport_position.z {
                                    ctx.set(position.x - viewport_position.x + (TERMINAL_WIDTH / 2), position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), tile.foreground, tile.background, tile.side_glyph);
                                }
                                else if viewport_position.z - position.z == 1 {
                                    ctx.set(position.x - viewport_position.x + (TERMINAL_WIDTH / 2), position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), tile.foreground, tile.background, tile.top_glyph);
                                } 
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }*/

    for (_player, viewshed, position) in (&mut players, &viewsheds, &mut positions).join() {
        //TODO: Add Viewshed systems
        for x in (position.x - viewshed.view_distance)..(position.x + viewshed.view_distance) {
            for y in (position.y - viewshed.view_distance)..(position.y + viewshed.view_distance) {
                for z in position.z - 1..position.z + 1 {
                    let target = &Vector3i::new(x, y, z);

                    let tile = map.tiles.get(target);

                    match tile {
                        Some(tile) => {
                            if viewshed.visible_tiles.contains(&tile.position) {  
                                if z == viewport_position.z {
                                    ctx.set(x - viewport_position.x + (TERMINAL_WIDTH / 2), y - viewport_position.y + (TERMINAL_HEIGHT / 2), tile.foreground, tile.background, tile.side_glyph);
                                }
                                else if viewport_position.z - z == 1 {
                                    ctx.set(x - viewport_position.x + (TERMINAL_WIDTH / 2), y - viewport_position.y + (TERMINAL_HEIGHT / 2), tile.foreground, tile.background, tile.top_glyph);
                                } 
                            }
                        },
                        _ => {}
                    }
                }
            }
        }
    }

}

fn draw_entities(ctx : &mut Rltk, ecs: &mut World, viewport_position: Vector3i) {
    let positions = ecs.read_storage::<Vector3i>();
    let renderables = ecs.read_storage::<Renderable>();

    let mut rendered_entities = HashMap::new();

    for (position, renderable) in (&positions, &renderables).join() {

        if !rendered_entities.contains_key(position) && !rendered_entities.contains_key(&(*position + Vector3i::new(0, 0, 1))) {
            if position.z == viewport_position.z {
                ctx.set(position.x - viewport_position.x + (TERMINAL_WIDTH / 2), position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), renderable.foreground, renderable.background, renderable.side_glyph);
                rendered_entities.insert(position, renderable);
            }
            else if viewport_position.z - position.z == 1 {
                ctx.set(position.x - viewport_position.x + (TERMINAL_WIDTH / 2), position.y - viewport_position.y + (TERMINAL_HEIGHT / 2), renderable.foreground, renderable.background, renderable.top_glyph);
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