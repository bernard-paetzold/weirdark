use std::{collections::{HashMap, HashSet}, time::Instant};

use rltk::RGB;
use specs::prelude::*;

use crate::{map, vectors::Vector3i, Map, Player, Tile, Viewshed, MAP_SIZE};

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
            let mut is_player = false;

            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();

                let player = player.get(entity);

                match player {
                    Some(_) => {
                        is_player = true;
                    }
                    _ => {}
                }
                let now = Instant::now();

                viewshed.visible_tiles = los(
                    map_tiles,
                    &mut HashSet::new(),
                    *position,
                    viewshed
                )
                .clone();
                
                let elapsed = now.elapsed();
                //println!("LOS: {:.2?}", elapsed);
            }
        }
    }
}

fn los<'a>(
    map_tiles: &'a mut HashMap<Vector3i, Tile>,
    visible_tiles: &'a mut HashSet<Vector3i>,
    source: Vector3i, 
    viewshed: &mut Viewshed) -> &'a HashSet<Vector3i> {

    let octants = [
        (1, 0, 0, 1),    // 0 - East
        (0, 1, 1, 0),    // 1 - South
        (0, -1, 1, 0),   // 2 - North
        (-1, 0, 0, 1),   // 3 - West
        (1, 0, 0, -1),   // 4 - Northeast
        (0, -1, -1, 0),  // 5 - Northwest
        (-1, 0, 0, -1),  // 6 - Southwest
        (0, 1, -1, 0),   // 7 - Southeast
    ];

    for &(xx, xy, yx, yy) in octants.iter() {
        light_cast(
            map_tiles,
            0,           // starting row
            1.0,         // start_slope
            0.0,         // end_slope
            xx, xy, yx, yy,
            viewshed.view_distance,
            source,
            visible_tiles
        );
    }
    //Insert source tile
    visible_tiles.insert(source + Vector3i::new(0, 0, -1));
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
                    _ => {
                        //If there is no tile or the tile is not opaque show the tile below that
                        let tile_below = map_tiles.get_mut(&(current_position + Vector3i::new(0,0,-1)));

                        match tile_below {
                            Some(_) => {
                                visible_tiles.insert(current_position + Vector3i::new(0,0,-1));
                            }
                            _ => {
                                //TODO: Change this to allow further z level view distance
                            }
                        }

                        //Also check tile above
                        let tile_above = map_tiles.get_mut(&(current_position + Vector3i::new(0,0,1)));

                        match tile_above {
                            Some(_) => {
                                visible_tiles.insert(current_position + Vector3i::new(0,0,1));
                            }
                            _ => {
                                //TODO: Change this to allow further z level view distance
                            }
                        }
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
                }
                else {
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
                        );
                        start_slope = right_slope;
                    }
                }
            }
        }
    }
    visible_tiles
}


fn bresenhams_line(target: Vector3i, source: Vector3i) -> Vec<Vector3i> {
    let mut tiles_positions = Vec::new();
    let mut x = source.x;
    let mut y = source.y;
    let x2 = target.x;
    let y2 = target.y;
    let w = x2 - x;
    let h = y2 - y;
    let mut dx1 = 0;
    let mut dy1 = 0;
    let mut dx2 = 0;
    let mut dy2 = 0;

    if w < 0 {
        dx1 = -1;
        dx2 = -1;
    } else if w > 0 {
        dx1 = 1;
        dx2 = 1;
    }

    if h < 0 {
        dy1 = -1;
    } else if h > 0 {
        dy1 = 1;
    }

    let mut longest = w.abs();
    let mut shortest = h.abs();

    if longest <= shortest {
        std::mem::swap(&mut longest, &mut shortest);
        if h < 0 {
            dy2 = -1;
        } else if h > 0 {
            dy2 = 1;
        }
        dx2 = 0;
    }

    let mut numerator = longest >> 1;
    for _ in 0..longest {
        tiles_positions.push(Vector3i::new(x, y, source.z));
        numerator += shortest;
        if numerator >= longest {
            numerator -= longest;
            x += dx1;
            y += dy1;
        } else {
            x += dx2;
            y += dy2;
        }
    }

    //Add the target tile to the line
    tiles_positions.push(target);
    tiles_positions
}

/*if let Some(target) = unchecked_tiles.iter().next().cloned() {
        unchecked_tiles.remove(&target);
        let mut ray = bresenhams_line(source, target.clone());
        let mut ray_blocked = false;

        while let Some(tile_position) = ray.pop() {
            if !ray_blocked {
                //Add tile to visible tiles and check if it blocks the ray
                visible_tiles.insert(tile_position);

                let tile = map_tiles.get_mut(&tile_position);

                match tile {
                    Some(tile) => {
                        if tile.opaque {
                            ray_blocked = true;
                        }
                    }
                    _ => {
                        //If there is no tile or the tile is not opaque show the tile below that
                        let tile_below = map_tiles.get_mut(&(tile_position + Vector3i::new(0,0,-1)));

                        match tile_below {
                            Some(_) => {
                                visible_tiles.insert(tile_position + Vector3i::new(0,0,-1));
                            }
                            _ => {
                                //TODO: Change this to allow further z level view distance
                            }
                        }

                        //Also check tile above
                        let tile_above = map_tiles.get_mut(&(tile_position + Vector3i::new(0,0,1)));

                        match tile_above {
                            Some(_) => {
                                visible_tiles.insert(tile_position + Vector3i::new(0,0,1));
                            }
                            _ => {
                                //TODO: Change this to allow further z level view distance
                            }
                        }
}
                }
            }
            //Remove the checked tile
            unchecked_tiles.remove(&tile_position);
        }
        los(map_tiles, unchecked_tiles, visible_tiles, source, is_player, dark_vision);
    }
    visible_tiles */
