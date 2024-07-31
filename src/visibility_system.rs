use std::{collections::{HashMap, HashSet}, time::Instant};

use specs::prelude::*;

use crate::{vectors::Vector3i, Map, Player, Tile, Viewshed};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Vector3i>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, positions, player) = data;
        let map_tiles = &mut map.tiles;

        for (entity, viewshed, position) in (&entities, &mut viewshed, &positions).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();

                let now = Instant::now();

                viewshed.visible_tiles =
                    los(map_tiles, &mut HashSet::new(), *position, viewshed).clone();

                let elapsed = now.elapsed();
                println!("LOS: {:.2?}", elapsed);
            }
        }
    }
}

fn los<'a>(
    map_tiles: &'a mut HashMap<Vector3i, Tile>,
    visible_tiles: &'a mut HashSet<Vector3i>,
    source: Vector3i,
    viewshed: &mut Viewshed,
) -> &'a HashSet<Vector3i> {
    let octants = [
        (1, 0, 0, 1),   // 0 - East
        (0, 1, 1, 0),   // 1 - South
        (0, -1, 1, 0),  // 2 - North
        (-1, 0, 0, 1),  // 3 - West
        (1, 0, 0, -1),  // 4 - Northeast
        (0, -1, -1, 0), // 5 - Northwest
        (-1, 0, 0, -1), // 6 - Southwest
        (0, 1, -1, 0),  // 7 - Southeast
    ];

    for &(xx, xy, yx, yy) in octants.iter() {
        light_cast(
            map_tiles,
            0,
            1.0,
            0.0,
            xx,
            xy,
            yx,
            yy,
            viewshed.view_distance,
            source,
            visible_tiles,
            viewshed.z_range,
        );
    }
    //Insert source tiles
    let mut down_z_blocked = false;
    let mut up_z_blocked = false;
    let mut current_z_offset = 0;

    while current_z_offset < viewshed.z_range && !(down_z_blocked || up_z_blocked) {
        if !down_z_blocked {
            //If there is no tile or the tile is not opaque show the tile below that
            let tile_below = map_tiles
                .get_mut(&(source + Vector3i::new(0, 0, -(current_z_offset as i32))));

            match tile_below {
                Some(tile) => {
                    visible_tiles
                        .insert(source + Vector3i::new(0, 0, -(current_z_offset as i32)));

                    if tile.opaque {
                        down_z_blocked = true;
                    }
                }
                _ => {}
            }
        }

        if !up_z_blocked {
            //Also check tile above
            let tile_above = map_tiles
                .get_mut(&(source + Vector3i::new(0, 0, current_z_offset as i32)));

            match tile_above {
                Some(tile) => {
                    visible_tiles
                        .insert(source + Vector3i::new(0, 0, current_z_offset as i32));

                    if tile.opaque {
                        up_z_blocked = true;
                    }
                }
                _ => {}
            }
        }
        current_z_offset += 1;
    }

    visible_tiles
}

fn light_cast<'a>(
    map_tiles: &'a mut HashMap<Vector3i, Tile>,
    row: usize,
    mut start_slope: f32,
    end_slope: f32,
    xx: i32,
    xy: i32,
    yx: i32,
    yy: i32,
    radius: usize,
    start_position: Vector3i,
    visible_tiles: &'a mut HashSet<Vector3i>,
    viewshed_z_range: usize,
) -> &'a mut HashSet<Vector3i> {
    if start_slope < end_slope {
        return visible_tiles;
    }

    let mut blocked = false;
    for distance in row..=radius {
        if blocked {
            break;
        }

        let delta_y = -(distance as i32);
        for delta_x in -(distance as i32)..=0 {
            let current_x = start_position.x + delta_x * xx + delta_y * xy;
            let current_y = start_position.y + delta_x * yx + delta_y * yy;
            let current_position = Vector3i::new(current_x, current_y, start_position.z);

            let left_slope = (delta_x as f32 - 1.0) / (delta_y as f32 + 0.5);
            let right_slope = (delta_x as f32 + 0.5) / (delta_y as f32 - 0.5);

            if start_slope < right_slope {
                continue;
            } else if end_slope > left_slope {
                break;
            }

            // Check if it's within viewshed
            if current_position.distance_to(start_position) as f32 <= radius as f32 {
                let tile = map_tiles.get(&current_position);

                match tile {
                    Some(_) => {
                        visible_tiles.insert(current_position);
                    }
                    _ => {}
                }

                let mut down_z_blocked = false;
                let mut up_z_blocked = false;
                let mut current_z_offset = 0;

                while current_z_offset < viewshed_z_range && !(down_z_blocked || up_z_blocked) {
                    if !down_z_blocked {
                        //If there is no tile or the tile is not opaque show the tile below that
                        let tile_below = map_tiles.get_mut(
                            &(current_position + Vector3i::new(0, 0, -(current_z_offset as i32))),
                        );

                        match tile_below {
                            Some(tile) => {
                                visible_tiles.insert(
                                    current_position
                                        + Vector3i::new(0, 0, -(current_z_offset as i32)),
                                );

                                if tile.opaque {
                                    down_z_blocked = true;
                                }
                            }
                            _ => {}
                        }
                    }

                    if !up_z_blocked {
                        //Also check tile above
                        let tile_above = map_tiles.get_mut(
                            &(current_position + Vector3i::new(0, 0, current_z_offset as i32)),
                        );

                        match tile_above {
                            Some(tile) => {
                                visible_tiles.insert(
                                    current_position + Vector3i::new(0, 0, current_z_offset as i32),
                                );

                                if tile.opaque {
                                    up_z_blocked = true;
                                }
                            }
                            _ => {}
                        }
                        current_z_offset += 1;
                    }
                }
            }

            if blocked {
                if let Some(tile) = map_tiles.get_mut(&current_position) {
                    if tile.opaque {
                        start_slope = right_slope;
                        continue;
                    } else {
                        blocked = false;
                    }
                } else {
                    blocked = false;
                }
            } else {
                if let Some(tile) = map_tiles.get_mut(&current_position) {
                    if tile.opaque {
                        blocked = true;

                        light_cast(
                            map_tiles,
                            distance as usize + 1,
                            start_slope,
                            left_slope,
                            xx,
                            xy,
                            yx,
                            yy,
                            radius,
                            start_position,
                            visible_tiles,
                            viewshed_z_range,
                        );
                        start_slope = right_slope;
                    }
                }
            }
        }
    }
    visible_tiles
}
