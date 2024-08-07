use std::collections::{HashMap, HashSet};

use specs::{prelude::*, shred::{Fetch, FetchMut}, storage::MaskedStorage};

use crate::{vectors::Vector3i, Illuminant, Map, Player, Tile, Viewshed, VisionBlocker};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Vector3i>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, VisionBlocker>,
        WriteStorage<'a, Illuminant>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, positions, _player, vision_blockers, mut illuminants) = data;
        let map_tiles = &mut map.tiles;

        for (entity, viewshed) in (&entities, &mut viewshed).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();

                //let now = Instant::now();
                let position = positions.get(entity).clone();

                if let Some(position) = position {
                    viewshed.visible_tiles =
                    los(map_tiles, &mut HashSet::new(), *position, viewshed, &vision_blockers, &positions).clone();
                }

                //let elapsed = now.elapsed();
                //println!("LOS: {:.2?}", elapsed);
            }

            if let Some(illuminant) = illuminants.get_mut(entity) {
                illuminant.dirty = true;
            }
        }
    }
}

fn los<'a>(
    map_tiles: &'a mut HashMap<Vector3i, Tile>,
    visible_tiles: &'a mut HashSet<Vector3i>,
    source: Vector3i,
    viewshed: &mut Viewshed,
    vision_blockers: &Storage<'a, VisionBlocker, Fetch<'a, MaskedStorage<VisionBlocker>>>,
    positions: &Storage<'a, Vector3i, FetchMut<'a, MaskedStorage<Vector3i>>>
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


    for z in -(viewshed.z_range as i32)..(viewshed.z_range as i32) + 1 {
        let current_source = source + Vector3i::new(0, 0, z);

        if !map_tiles.get(&current_source).is_some_and(|x| x.opaque) {
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
                    current_source,
                    visible_tiles,
                    viewshed.z_range,
                    &vision_blockers,
                    &positions,
                );
            }
        }
    }
    //Insert source tiles
    let mut down_z_blocked = false;
    let mut up_z_blocked = false;
    let mut current_z_offset = 0;

    while current_z_offset < viewshed.z_range && !(down_z_blocked || up_z_blocked) {
        if !down_z_blocked {
            let target_position = source + Vector3i::new(0, 0, -(current_z_offset as i32));
            //If there is no tile or the tile is not opaque show the tile below that
            let tile_below = map_tiles
                .get_mut(&target_position);

            match tile_below {
                Some(tile) => {
                    visible_tiles.insert(target_position);

                    if tile.opaque || (vision_blockers, positions).join().filter(|x| *x.1 == target_position).next().is_some() {
                        down_z_blocked = true;
                    }
                }
                _ => {}
            }
        }

        if !up_z_blocked {
            let target_position = source + Vector3i::new(0, 0, current_z_offset as i32);
            //Also check tile above
            let tile_above = map_tiles
                .get_mut(&target_position);

            match tile_above {
                Some(tile) => {
                    visible_tiles
                        .insert(target_position);

                    if tile.opaque || (vision_blockers, positions).join().filter(|x| *x.1 == target_position).next().is_some() {
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
    vision_blockers: &Storage<'a, VisionBlocker, Fetch<'a, MaskedStorage<VisionBlocker>>>,
    positions: &Storage<'a, Vector3i, FetchMut<'a, MaskedStorage<Vector3i>>>
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
        for delta_x in -(distance as i32)..= 0 {
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

                let mut tile_transparent = false;

                match tile {
                    Some(tile) => {
                        visible_tiles.insert(current_position);

                        if !tile.opaque && (vision_blockers, positions).join().filter(|x| *x.1 == current_position).next().is_none() {
                            tile_transparent = true;     
                        }
                    }
                    _ => {}
                }

                if tile_transparent {
                    let target_position = current_position + Vector3i::new(0, 0, -1);
                    let tile_below = map_tiles.get_mut(
                        &(target_position),
                    );
    
                    match tile_below {
                        Some(_) => {
                            visible_tiles.insert(target_position);  
                        }
                        _ => {}
                    }

                }
            }

            if blocked {
                if let Some(tile) = map_tiles.get_mut(&current_position) {
                    if tile.opaque || (vision_blockers, positions).join().filter(|x| *x.1 == current_position).next().is_some() {
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
                    if tile.opaque || (vision_blockers, positions).join().filter(|x| *x.1 == current_position).next().is_some() {
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
                            vision_blockers,
                            positions
                        );
                        start_slope = right_slope;
                    }
                }
            }
        }
    }
    visible_tiles
}
