use std::{collections::{HashMap, HashSet}, time::Instant};

use specs::prelude::*;

use crate::{vectors::Vector3i, Map, Tile, Viewshed, MAP_SIZE};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        WriteStorage<'a, Vector3i>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut viewshed, position) = data;
        let map_tiles = &map.tiles;

        for (viewshed, position) in (&mut viewshed, &position).join() {
            if viewshed.dirty {
                let now = Instant::now();
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                println!("LOS calculations");

                let mut unchecked_tiles = HashSet::<Vector3i>::new();

                //Add all tiles within view range to a hashset
                for x in (position.x - viewshed.view_distance)..(position.x + viewshed.view_distance) {
                    for y in (position.y - viewshed.view_distance)..(position.y + viewshed.view_distance) {
                        let target = Vector3i::new(x, y, position.z);

                        if position.distance_to(target) < viewshed.view_distance
                        && target.x >= -MAP_SIZE && target.x < MAP_SIZE 
                        && target.y >= -MAP_SIZE && target.y < MAP_SIZE 
                        && target.z >= -MAP_SIZE && target.z < MAP_SIZE {
                            unchecked_tiles.insert(target);
                        }   
                    }
                }

                //Run each tile within view through a los function
                let mut visible_tiles = HashSet::<Vector3i>::new();
                viewshed.visible_tiles = los(map_tiles, unchecked_tiles, &mut visible_tiles, *position).clone();

                let elapsed = now.elapsed();
                println!("LOS: {:.2?}", elapsed);
            }
        }
    }

}

fn los<'a> (map_tiles: &'a HashMap<Vector3i, Tile>, mut unchecked_tiles: HashSet<Vector3i>, visible_tiles: &'a mut HashSet<Vector3i>, source: Vector3i) -> &'a HashSet<Vector3i> {
    if let Some(target) = unchecked_tiles.iter().next().cloned() {
        unchecked_tiles.remove(&target);
        let mut ray = bresenhams_line(source, target.clone());
        let mut ray_blocked = false;

        while let Some(tile_position) = ray.pop() {
            if !ray_blocked {
                //Add tile to visible tiles and check if it blocks the ray
                if let Some(tile) = map_tiles.get(&tile_position) {
                    visible_tiles.insert(tile_position);
        
                    if tile.opaque {
                        ray_blocked = true;
                    }
                }
                else if let Some(_) = map_tiles.get(&(tile_position + Vector3i::new(0,0,-1))) {
                    visible_tiles.insert(tile_position + Vector3i::new(0,0,-1));
                }
            }
            //Remove the checked tile
            unchecked_tiles.remove(&tile_position);  
        }
        los(map_tiles, unchecked_tiles, visible_tiles, source);
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

    tiles_positions
}