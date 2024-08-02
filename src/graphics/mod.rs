use std::collections::{HashMap, HashSet};

use rltk::{to_cp437, ColorPair, DrawBatch, Point, Rltk, RGB, RGBA};
use specs::{
    prelude::*,
    shred::{Fetch, FetchMut},
    storage::MaskedStorage,
};

use crate::{
    colors::{dim_color, mix_colors},
    vectors::Vector3i,
    Camera, Map, Photometry, Player, Renderable, Viewshed, TERMINAL_HEIGHT, TERMINAL_WIDTH,
};

pub mod components;

pub fn draw_tiles(ctx: &mut Rltk, ecs: &mut World, viewport_position: Vector3i) {
    let mut draw_batch = DrawBatch::new();

    let discovered_tile_dimming = 0.2;

    let positions = ecs.write_storage::<Vector3i>();
    let viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();

    let map = ecs.fetch::<Map>();

    for (_player, viewshed, position) in (&mut players, &viewsheds, &positions).join() {
        for tile_position in viewshed.discovered_tiles.iter().filter(|tile_position| {
            (tile_position.x - position.x).abs() < TERMINAL_WIDTH / 2
                && (tile_position.y - position.y).abs() < TERMINAL_HEIGHT / 2
                && (tile_position.z - position.z).abs() < viewshed.z_range as i32
        }) {
            let tile = map.tiles.get(&tile_position);

            match tile {
                Some(tile) if tile.foreground.a > 0.0 || tile.background.a > 0.0 => {
                    if viewshed.visible_tiles.contains(&tile.position) && tile.light_level > 0.0 {
                        let foreground_color = calculate_lit_color(
                            tile.foreground,
                            tile.light_color,
                            tile.light_level,
                        );
                        if tile_position.z == viewport_position.z {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2),
                                ),
                                ColorPair::new(foreground_color, tile.background),
                                tile.side_glyph,
                                1,
                            );
                        } else if viewport_position.z - tile_position.z == 1 {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2),
                                ),
                                ColorPair::new(foreground_color, tile.background),
                                tile.top_glyph,
                                0,
                            );
                        } else {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2),
                                ),
                                ColorPair::new(
                                    dim_color(
                                        foreground_color,
                                        tile_position.z as f32
                                            / (viewport_position.z + tile_position.z) as f32,
                                    ),
                                    tile.background,
                                ),
                                tile.side_glyph,
                                0,
                            );
                        }
                    } else {
                        let foreground =
                            dim_discovered_tile_color(tile.foreground, discovered_tile_dimming)
                                .to_greyscale();
                        let background =
                            dim_discovered_tile_color(tile.background, discovered_tile_dimming)
                                .to_greyscale();

                        if tile_position.z == viewport_position.z {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2),
                                ),
                                ColorPair::new(foreground, background),
                                tile.side_glyph,
                                1,
                            );
                        } else if viewport_position.z - tile_position.z == 1 {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2),
                                ),
                                ColorPair::new(foreground, background),
                                tile.top_glyph,
                                0,
                            );
                        } else {
                            draw_batch.set_with_z(
                                Point::new(
                                    tile_position.x - viewport_position.x + (TERMINAL_WIDTH / 2),
                                    tile_position.y - viewport_position.y + (TERMINAL_HEIGHT / 2),
                                ),
                                ColorPair::new(
                                    dim_color(
                                        foreground,
                                        tile_position.z as f32
                                            / (viewport_position.z + tile_position.z) as f32,
                                    ),
                                    background,
                                ),
                                tile.top_glyph,
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

pub fn draw_entities(ctx: &mut Rltk, ecs: &mut World, viewport_position: Vector3i) {
    let mut entity_draw_batch = DrawBatch::new();

    let positions = ecs.write_storage::<Vector3i>();
    let viewsheds = ecs.write_storage::<Viewshed>();
    let mut players = ecs.write_storage::<Player>();
    let renderables = ecs.read_storage::<Renderable>();
    let photometria = ecs.read_storage::<Photometry>();

    for (_player, viewshed) in (&mut players, &viewsheds).join() {
        let mut rendered_entities = HashMap::new();

        for (position, renderable, photometry) in (&positions, &renderables, &photometria)
            .join()
            .filter(|&x| {
                viewshed.visible_tiles.contains(x.0)
                    || viewshed
                        .visible_tiles
                        .contains(&(*x.0 + Vector3i::new(0, 0, -1)))
            })
        {
            if !rendered_entities.contains_key(position)
                && !rendered_entities.contains_key(&(*position + Vector3i::new(0, 0, 1)))
            {
                let mut foreground_color = calculate_lit_color(
                    renderable.foreground,
                    photometry.light_color,
                    photometry.light_level,
                );
                let background_color = renderable.background;
                foreground_color.a = photometry.light_level;

                if position.z == viewport_position.z {
                    entity_draw_batch.set_with_z(
                        Point::new(
                            position.x - viewport_position.x + (TERMINAL_WIDTH / 2),
                            position.y - viewport_position.y + (TERMINAL_HEIGHT / 2),
                        ),
                        ColorPair::new(foreground_color, background_color),
                        renderable.side_glyph,
                        1,
                    );
                    //rendered_entities.insert(position, renderable);
                } else if viewport_position.z - position.z == 1 {
                    entity_draw_batch.set_with_z(
                        Point::new(
                            position.x - viewport_position.x + (TERMINAL_WIDTH / 2),
                            position.y - viewport_position.y + (TERMINAL_HEIGHT / 2),
                        ),
                        ColorPair::new(foreground_color, background_color),
                        renderable.top_glyph,
                        0,
                    );
                }
                rendered_entities.insert(position, renderable);
            }
        }
    }
    entity_draw_batch.submit(1).expect("Batch error");
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
    mix_colors(
        dim_color(surface_color, intensity),
        light_color,
        intensity - (1.0 - light_color.to_rgb().to_hsv().v),
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
