use rltk::RGB;
use specs::prelude::*;

use crate::{colors::mix_colors, entities, vectors::Vector3i, Illuminant, Map, Photometry, Player, Viewshed};


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
                tile.1.light_color = RGB::named(rltk::WHITE).to_rgba(1.0);
            }
        

            for (illuminant, viewshed, position) in (&mut illuminants, &viewsheds, &positions).join() {
                if illuminant.on {
                    illuminant.dirty = false;
                    
                    for tile_position in viewshed.visible_tiles.iter() {
                        match map_tiles.get_mut(tile_position) {
                            Some(tile) => {
                                //let illumination = (illuminant.intensity - position.distance_to(*tile_position) as f32 / illuminant.range as f32).max(0.0); 
                                let illumination = (illuminant.intensity - position.distance_to(*tile_position) as f32 / illuminant.range as f32 * illuminant.intensity).max(0.0); 

                                tile.light_color = mix_colors(tile.light_color, illuminant.color, get_illumination_ratio(tile.light_level, illumination));
                                tile.light_level = tile.light_level.max(illumination);

                                //TODO: Change this to be in a better location
                                for (_player, player_viewshed) in (&players, &viewsheds).join() {
                                    if player_viewshed.visible_tiles.contains(tile_position) && tile.light_level >= 1.0 - player_viewshed.dark_vision {
                                        tile.discovered = true;
                                    }
                                }
                            },
                            _ => {},
                        }        
                    }
                }
            }
        }

        //Update dirty photometry
        //TODO: Improve this system so it does not rely on a player being in or on a tile
        for (entity, photometry, position) in (&entities, &mut photometria, &positions).join() {
            if photometry.dirty {
                photometry.dirty = false;

                match map_tiles.get_mut(position) {
                    Some(tile) => {
                        photometry.light_color = mix_colors(photometry.light_color, tile.light_color, tile.light_level);
                        photometry.light_level = tile.light_level;
                    },
                    _ => {
                        match map_tiles.get_mut(&(*position + Vector3i::new(0, 0, -1))) {
                            Some(tile) => {
                                photometry.light_level = tile.light_level;
                            },
                            _ => {
                                if let Some(illuminant) = illuminants.get(entity) {
                                    photometry.light_color = illuminant.color;
                                    photometry.light_level = illuminant.intensity;
                                }
                            },
                        }
                    },
                }    
            }
        }
    }

}

fn get_illumination_ratio(intensity_one: f32, intensity_two: f32) -> f32 {
    if intensity_one == 0.0 && intensity_two == 0.0 {
        return 0.5
    }
    
    intensity_two / (intensity_one + intensity_two)
}