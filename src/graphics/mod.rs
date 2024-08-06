use std::collections::HashMap;

use rltk::{ColorPair, DrawBatch, Point, Rltk, RGBA};
use specs::prelude::*;

use crate::{
    colors::{dim_color, mix_colors}, vectors::Vector3i, Camera, Map, Photometry, Player, Renderable, Viewshed, MAP_SCREEN_HEIGHT, MAP_SCREEN_WIDTH
};

pub mod components;

pub fn render_map(ecs: &mut World, ctx: &mut Rltk) {
    //Rendering
    let viewport_position = get_viewport_position(&ecs);

    ctx.set_active_console(0);
    ctx.cls();

    draw_tiles(ecs, viewport_position);
    rltk::render_draw_buffer(ctx).expect("Draw error");

    ctx.set_active_console(1);
    ctx.cls();

    draw_entities(ecs, viewport_position);
    rltk::render_draw_buffer(ctx).expect("Draw error");
}

pub fn draw_tiles(ecs: &mut World, viewport_position: Vector3i) {
    let mut draw_batch = DrawBatch::new();

    let discovered_tile_dimming = 0.2;

    let positions = ecs.write_storage::<Vector3i>();
    let viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();

    let map = ecs.fetch::<Map>();

    for (_player, viewshed, position) in (&mut players, &viewsheds, &positions).join() {
        for tile_position in viewshed.discovered_tiles.iter().filter(|tile_position| {
            (tile_position.x - position.x).abs() < MAP_SCREEN_WIDTH / 2
                && (tile_position.y - position.y).abs() < MAP_SCREEN_HEIGHT / 2
                && (tile_position.z - position.z).abs() < viewshed.z_range as i32
        }) {
            let tile = map.tiles.get(&tile_position);

            match tile {
                Some(tile) if tile.renderable.foreground.a > 0.0 || tile.renderable.background.a > 0.0 => {
                    if viewshed.visible_tiles.contains(&tile.position) && tile.photometry.light_level > 0.0 {
                        let foreground_color = calculate_lit_color(
                            tile.renderable.foreground,
                            tile.photometry.light_color,
                            tile.photometry.light_level,
                        );
                        if tile_position.z == viewport_position.z {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (MAP_SCREEN_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (MAP_SCREEN_HEIGHT / 2),
                                ),
                                ColorPair::new(foreground_color, tile.renderable.background),
                                tile.renderable.side_glyph,
                                1,
                            );
                        } else if viewport_position.z - tile_position.z == 1 {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (MAP_SCREEN_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (MAP_SCREEN_HEIGHT / 2),
                                ),
                                ColorPair::new(foreground_color, tile.renderable.background),
                                tile.renderable.top_glyph,
                                0,
                            );
                        } else {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (MAP_SCREEN_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (MAP_SCREEN_HEIGHT / 2),
                                ),
                                ColorPair::new(
                                    dim_color(
                                        foreground_color,
                                        tile_position.z as f32
                                            / (viewport_position.z + tile_position.z) as f32,
                                    ),
                                    tile.renderable.background,
                                ),
                                tile.renderable.side_glyph,
                                0,
                            );
                        }
                    } else {
                        let foreground =
                            dim_discovered_tile_color(tile.renderable.foreground, discovered_tile_dimming)
                                .to_greyscale();
                        let background =
                            dim_discovered_tile_color(tile.renderable.background, discovered_tile_dimming)
                                .to_greyscale();

                        if tile_position.z == viewport_position.z {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (MAP_SCREEN_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (MAP_SCREEN_HEIGHT / 2),
                                ),
                                ColorPair::new(foreground, background),
                                tile.renderable.side_glyph,
                                1,
                            );
                        } else if viewport_position.z - tile_position.z == 1 {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (MAP_SCREEN_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (MAP_SCREEN_HEIGHT / 2),
                                ),
                                ColorPair::new(foreground, background),
                                tile.renderable.top_glyph,
                                0,
                            );
                        } else {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (MAP_SCREEN_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (MAP_SCREEN_HEIGHT / 2),
                                ),
                                ColorPair::new(
                                    dim_color(
                                        foreground,
                                        tile_position.z as f32
                                            / (viewport_position.z + tile_position.z) as f32,
                                    ),
                                    background,
                                ),
                                tile.renderable.top_glyph,
                                0,
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}

pub fn draw_entities(ecs: &mut World, viewport_position: Vector3i) {
    let mut entity_draw_batch = DrawBatch::new();

    let positions = ecs.write_storage::<Vector3i>();
    let viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let renderables = ecs.read_storage::<Renderable>();
    let photometria = ecs.read_storage::<Photometry>();

    for (_player, viewshed) in (&mut players, &viewsheds).join() {
        //let mut rendered_entities = HashMap::new();

        for (position, renderable, photometry) in (&positions, &renderables, &photometria)
            .join()
            .filter(|&x| {
                viewshed.visible_tiles.contains(x.0)
                    || viewshed
                        .visible_tiles
                        .contains(&(*x.0 + Vector3i::new(0, 0, -1)))
            })
        {

            let foreground_color = calculate_lit_color(
                renderable.foreground,
                photometry.light_color,
                photometry.light_level,
            );
            let background_color = renderable.background;

            if position.z == viewport_position.z {
                entity_draw_batch.set_with_z(
                    Point::new(
                        position.x - viewport_position.x + (MAP_SCREEN_WIDTH / 2),
                        position.y - viewport_position.y + (MAP_SCREEN_HEIGHT / 2),
                    ),
                    ColorPair::new(foreground_color, background_color),
                    renderable.side_glyph,
                    1,
                );
            } else if viewport_position.z - position.z == 1 {
                entity_draw_batch.set_with_z(
                    Point::new(
                        position.x - viewport_position.x + (MAP_SCREEN_WIDTH / 2),
                        position.y - viewport_position.y + (MAP_SCREEN_HEIGHT / 2),
                    ),
                    ColorPair::new(foreground_color, background_color),
                    renderable.top_glyph,
                    0,
                );
            }
        }
    }
    entity_draw_batch.submit(1).expect("Batch error");
}

pub fn get_viewport_position(ecs: &World) -> Vector3i {
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
    mix_colors(
        dim_color(surface_color, intensity),
        light_color,
        intensity - (1.0 - surface_color.to_rgb().to_hsv().v),
    )
    //light_color * intensity
}

pub fn dim_discovered_tile_color(color: RGBA, factor: f32) -> RGBA {
    let alpha = color.a;

    let mut color = color.to_rgb().to_hsv();

    while color.v > factor {
        color.v = color.v * factor;
    }

    return color.to_rgba(alpha);
}
