use std::collections::HashSet;

use rltk::RGB;
use specs::{prelude::*, world};

use crate::{
    colors::mix_colors, get_player_entity, player, vectors::Vector3i, Illuminant, Map, Photometry,
    Player, Viewshed,
};

pub struct LightingSystem {}

impl<'a> System<'a> for LightingSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Illuminant>,
        WriteStorage<'a, Photometry>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Vector3i>,
        ReadStorage<'a, Player>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            mut illuminants,
            mut photometria,
            mut viewsheds,
            positions,
            players,
            entities,
        ) = data;
        let map_tiles = &mut map.tiles;
        let mut discovered_tiles = HashSet::new();

        // Reset light levels if any illuminants are dirty
        if (&illuminants, &positions)
            .join()
            .any(|(illuminant, _)| illuminant.dirty)
        {
            for tile in map_tiles.values_mut() {
                tile.light_level = 0.0;
                tile.light_color = RGB::named(rltk::WHITE).to_rgba(1.0);
            }

            // Process illuminants
            for (illuminant, viewshed, position) in
                (&mut illuminants, &viewsheds, &positions).join()
            {
                if illuminant.on {
                    illuminant.dirty = false;

                    for tile_position in &viewshed.visible_tiles {
                        let distance_to_tile = position.distance_to(*tile_position);

                        if distance_to_tile <= illuminant.range as i32 {
                            if let Some(tile) = map_tiles.get_mut(tile_position) {
                                let illumination = illuminant.intensity
                                    - (illuminant.intensity
                                        * (distance_to_tile as f32 / illuminant.range as f32));

                                tile.light_color = mix_colors(
                                    tile.light_color,
                                    illuminant.color,
                                    get_illumination_ratio(tile.light_level, illumination),
                                );
                                tile.light_level += illumination;

                                discovered_tiles.insert(*tile_position);
                            }
                        }
                    }
                }
            }
        }

        // Update player's discovered tiles
        if let Some(player_entity) = get_player_entity(entities, players) {
            if let Some(player_viewshed) = viewsheds.get_mut(player_entity) {
                for tile_position in &discovered_tiles {
                    if let Some(tile) = map_tiles.get(tile_position) {
                        if tile.light_level >= 1.0 - player_viewshed.dark_vision {
                            player_viewshed.discovered_tiles.insert(tile.position);
                        }
                    }
                }
            }
        }

        // Update dirty photometry
        for (photometry, position) in (&mut photometria, &positions).join() {
            if photometry.dirty {
                photometry.dirty = false;

                if let Some(tile) = map_tiles.get_mut(position) {
                    photometry.light_color =
                        mix_colors(photometry.light_color, tile.light_color, tile.light_level);
                    photometry.light_level = tile.light_level;
                } else if let Some(tile) = map_tiles.get_mut(&(*position + Vector3i::new(0, 0, -1))) {
                    photometry.light_level = tile.light_level;
                }
            }
        }
    }
}

fn get_illumination_ratio(intensity_one: f32, intensity_two: f32) -> f32 {
    if intensity_one == 0.0 && intensity_two == 0.0 {
        return 0.5;
    }

    intensity_two / (intensity_one + intensity_two)
}
