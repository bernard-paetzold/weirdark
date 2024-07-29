use std::{collections::{HashMap, HashSet}, time::Instant};

use specs::{prelude::*, shred::FetchMut, storage::{GenericReadStorage, MaskedStorage}, world::EntitiesRes};

use crate::{player, vectors::Vector3i, Map, Player, Tile, Viewshed, MAP_SIZE};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Viewshed>,
                        WriteStorage<'a, Vector3i>,
                        ReadStorage<'a, Player>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut viewshed, positions, player) = data;
        let map_tiles = &mut map.tiles;

        for (entity, viewshed, position) in (&entities, &mut viewshed, &positions).join() {
            let mut is_player = false;
            
            if viewshed.dirty {
                let now = Instant::now();
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();

                let player = player.get(entity);

                match player {
                    Some(_) => {
                        is_player = true;
                    },
                    _ => {}
                }

                let mut unchecked_tiles = HashSet::<Vector3i>::new();

                //Add all tiles within view range to a hashset
                for x in (position.x - viewshed.view_distance as i32)..(position.x + viewshed.view_distance as i32) {
                    for y in (position.y - viewshed.view_distance as i32)..(position.y + viewshed.view_distance as i32) {
                        let target = Vector3i::new(x, y, position.z);

                        if position.distance_to(target) < viewshed.view_distance as i32
                        && target.x >= -MAP_SIZE && target.x < MAP_SIZE 
                        && target.y >= -MAP_SIZE && target.y < MAP_SIZE 
                        && target.z >= -MAP_SIZE && target.z < MAP_SIZE {
                            unchecked_tiles.insert(target);
                        }   
                    }
                }

                //Run each tile within view through a los function
                viewshed.visible_tiles = los(map_tiles, unchecked_tiles, &mut HashSet::new(), *position, is_player, viewshed.dark_vision).clone();

                let elapsed = now.elapsed();
                println!("LOS: {:.2?}", elapsed);
            }
        }
    }

}

fn los<'a> (map_tiles: &'a mut HashMap<Vector3i, Tile>, mut unchecked_tiles: HashSet<Vector3i>, visible_tiles: &'a mut HashSet<Vector3i>, source: Vector3i, is_player: bool, dark_vision: f32) -> &'a HashSet<Vector3i> {
    if let Some(target) = unchecked_tiles.iter().next().cloned() {
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
                        let tile = map_tiles.get_mut(&(tile_position + Vector3i::new(0,0,-1)));

                        match tile {
                            Some(tile) => {
                                visible_tiles.insert(tile_position + Vector3i::new(0,0,-1));
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