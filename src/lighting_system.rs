use std::{collections::{HashMap, HashSet}, time::Instant};

use specs::{prelude::*, shred::FetchMut, storage::MaskedStorage, world::EntitiesRes};

use crate::{vectors::Vector3i, Illuminant, Map, Photometry, Tile, Viewshed, MAP_SIZE};

pub const LIFT_FALLOFF: f32 = 0.1;

pub struct LightingSystem {}

impl<'a> System<'a> for LightingSystem {
    type SystemData = (WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Illuminant>,
                        ReadStorage<'a, Photometry>,
                        WriteStorage<'a, Vector3i>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut illuminants, mut photometry, positions) = data;
        let map_tiles = &mut map.tiles;

        for (illuminant, position) in (&mut illuminants, &positions).join() {
            if illuminant.dirty {
                let now = Instant::now();
                illuminant.dirty = false;

                let mut unmodified_tiles = HashSet::<Vector3i>::new();
                
                let illuminant_range: i32 = (illuminant.intensity / LIFT_FALLOFF).round() as i32;

                //Add all tiles within light range to a hashset
                for x in (position.x - illuminant_range)..(position.x + illuminant_range) {
                    for y in (position.y - illuminant_range)..(position.y + illuminant_range) {
                        let target = Vector3i::new(x, y, position.z);

                        if position.distance_to(target) < illuminant_range
                        && target.x >= -MAP_SIZE && target.x < MAP_SIZE 
                        && target.y >= -MAP_SIZE && target.y < MAP_SIZE 
                        && target.z >= -MAP_SIZE && target.z < MAP_SIZE {
                            unmodified_tiles.insert(target);
                        }   
                    }
                }

                //Run each tile within view through a los function
                let lit_tiles = los(map_tiles, unmodified_tiles, &mut HashSet::new(), *position).clone();

                for tile_position in lit_tiles.iter() {
                    match map_tiles.get_mut(tile_position) {
                        Some(tile) => {
                            tile.light_level = position.distance_to(*tile_position) as f32 * LIFT_FALLOFF;
                            println!("Light level: {}", tile.light_level);
                        },
                        _ => {},
                    }

                    
                }

                let elapsed = now.elapsed();
                println!("Lighting: {:.2?}", elapsed);
            }
        }
    }

}

fn los<'a> (map_tiles: &'a HashMap<Vector3i, Tile>, mut unchecked_tiles: HashSet<Vector3i>, lit_tiles: &'a mut HashSet<Vector3i>, source: Vector3i) -> &'a HashSet<Vector3i> {
    if let Some(target) = unchecked_tiles.iter().next().cloned() {
        unchecked_tiles.remove(&target);
        let mut ray = bresenhams_line(source, target.clone());
        let mut ray_blocked = false;

        while let Some(tile_position) = ray.pop() {
            if !ray_blocked {
                //Add tile to visible tiles and check if it blocks the ray
                lit_tiles.insert(tile_position);

                let tile = map_tiles.get(&tile_position);

                match tile {
                    Some(tile) if tile.opaque => {
                        ray_blocked = true;
                    }
                    _ => {
                        //If there is no tile or the tile is not opaque show the tile below that
                        //TODO: Change this to allow further z level view distance
                        lit_tiles.insert(tile_position + Vector3i::new(0,0,-1));
                    }
                }
            }
            //Remove the checked tile
            unchecked_tiles.remove(&tile_position);  
        }
        los(map_tiles, unchecked_tiles, lit_tiles, source);
    }
    lit_tiles                       
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