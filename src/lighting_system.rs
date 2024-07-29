use std::{borrow::BorrowMut, cmp, collections::{HashMap, HashSet}, time::Instant};

use specs::{prelude::*, shred::FetchMut, storage::MaskedStorage, world::EntitiesRes};

use crate::{player, vectors::Vector3i, Illuminant, Map, Photometry, Player, Tile, Viewshed, MAP_SIZE};

pub const LIGHT_FALLOFF: f32 = 0.05;

pub struct LightingSystem {}

impl<'a> System<'a> for LightingSystem {
    type SystemData = (WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Illuminant>,
                        WriteStorage<'a, Photometry>,
                        ReadStorage<'a, Viewshed>,
                        WriteStorage<'a, Vector3i>,
                        ReadStorage<'a, Player>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut illuminants, mut photometria, viewsheds, positions, players) = data;
        let map_tiles = &mut map.tiles;

        if !(&mut illuminants, &viewsheds, &positions).join().filter(|x| x.0.dirty).next().is_none() {
            //TODO: Improve this so only tiles in affected areas are reset
            for tile in map_tiles.iter_mut() {
                tile.1.light_level = 0.0;  
            }
        

            for (entity, illuminant, viewshed, position) in (&entities, &mut illuminants, &viewsheds, &positions).join() {
                //if illuminant.dirty {
                    //let now = Instant::now();
                    illuminant.dirty = false;

                    for tile_position in viewshed.visible_tiles.iter() {
                        match map_tiles.get_mut(tile_position) {
                            Some(tile) => {
                                tile.light_level = tile.light_level + (illuminant.intensity - position.distance_to(*tile_position) as f32 / illuminant.range as f32).max(0.0);
                                
                                //TODO: Change this to be in a better location
                                for (_player, player_viewshed) in (&players, &viewsheds).join() {
                                    if player_viewshed.visible_tiles.contains(tile_position) && tile.light_level > 1.0 - viewshed.dark_vision {
                                        tile.discovered = true;
                                    }
                                }
                            },
                            _ => {},
                        }        
                    }
                    //let elapsed = now.elapsed();
                    //println!("Lighting: {:.2?}", elapsed);
                //}
            }
        }

        //Update dirty photometry
        //TODO: Improve this system so it does not rely on a player being in or on a tile
        for (photometry, position) in (&mut photometria, &positions).join() {
            if photometry.dirty {
                photometry.dirty = false;

                match map_tiles.get_mut(position) {
                    Some(tile) => {
                        photometry.light_level = tile.light_level;
                    },
                    _ => {
                        match map_tiles.get_mut(&(*position + Vector3i::new(0, 0, -1))) {
                            Some(tile) => {
                                photometry.light_level = tile.light_level;
                            },
                            _ => {
                                photometry.light_level = 0.0;
                            },
                        }
                    },
                }    
            }
        }
    }

}