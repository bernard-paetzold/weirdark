use std::{collections::{HashMap, HashSet}, time::Instant};

use specs::{prelude::*, shred::FetchMut, storage::MaskedStorage, world::EntitiesRes};

use crate::{vectors::Vector3i, Illuminant, Map, Photometry, Tile, Viewshed, MAP_SIZE};

pub const LIGHT_FALLOFF: f32 = 0.02;

pub struct LightingSystem {}

impl<'a> System<'a> for LightingSystem {
    type SystemData = (WriteExpect<'a, Map>,
                        Entities<'a>,
                        WriteStorage<'a, Illuminant>,
                        WriteStorage<'a, Photometry>,
                        ReadStorage<'a, Viewshed>,
                        WriteStorage<'a, Vector3i>);

    fn run(&mut self, data : Self::SystemData) {
        let (mut map, entities, mut illuminants, mut photometria, viewsheds, positions) = data;
        let map_tiles = &mut map.tiles;

        for (illuminant, viewshed, position) in (&mut illuminants, &viewsheds, &positions).join() {
            if illuminant.dirty {
                let now = Instant::now();
                illuminant.dirty = false;

                for tile_position in viewshed.visible_tiles.iter() {
                    match map_tiles.get_mut(tile_position) {
                        Some(tile) => {
                            tile.light_level = illuminant.intensity - position.distance_to(*tile_position) as f32 * LIGHT_FALLOFF;
                        },
                        _ => {},
                    }        
                }
                let elapsed = now.elapsed();
                println!("Lighting: {:.2?}", elapsed);
            }
        }

        //Update dirty photometry
        //TODO: Improve this system so it does not rely on a player being in or on a tile
        for (photometry, position) in (&mut photometria, &positions).join() {
            if photometry.dirty {
                photometry.dirty = false;
                println!("{}", position);

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