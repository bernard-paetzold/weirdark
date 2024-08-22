use std::{collections::HashSet, time};

use rltk::RGB;
use specs::prelude::*;

use crate::{
    colors::mix_colors, entities::power_components::Wire, get_player_entity, vectors::Vector3i,
    Illuminant, Map, Photometry, Player, Viewshed,
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
        ReadStorage<'a, Wire>,
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
            wires,
            entities,
        ) = data;
        let map_tiles = &mut map.tiles;
        let mut discovered_tiles = HashSet::new();
        let mut affected_tiles = HashSet::new();

        // Reset light levels if any illuminants are dirty
        if (&illuminants, &positions)
            .join()
            .any(|(illuminant, _)| illuminant.dirty)
        {
            // Process illuminants
            for (_illuminant, viewshed, position) in (&illuminants, &viewsheds, &positions)
                .join()
                .filter(|(illuminant, _, _)| illuminant.dirty)
            {
                //Reset light levels in affected illuminants
                for (tile_position, tile) in map_tiles.iter_mut().filter(|(tile_position, _)| {
                    tile_position.distance_to_int(*position) < viewshed.view_distance as i32
                }) {
                    tile.photometry.light_level = 0.0;
                    tile.photometry.light_color = RGB::named(rltk::WHITE).to_rgba(1.0);
                    tile.photometry.dirty = true;
                    affected_tiles.insert(*tile_position);
                }
            }

            //TODO: Add some light leaking to neighbouring tiles that have lower light levels than the lit tile
            for (illuminant, viewshed, position) in
                (&mut illuminants, &viewsheds, &positions).join()
            {
                illuminant.dirty = false;

                if illuminant.on {
                    for tile_position in &viewshed.visible_tiles {
                        let distance_to_tile = position.distance_to_int(*tile_position);

                        if distance_to_tile <= illuminant.range as i32 {
                            if let Some(tile) = map_tiles.get_mut(tile_position) {
                                let illumination = illuminant.intensity
                                    - (illuminant.intensity
                                        * (distance_to_tile as f32
                                            / (illuminant.range - 1) as f32));

                                tile.photometry.light_color = mix_colors(
                                    tile.photometry.light_color,
                                    illuminant.color,
                                    get_illumination_ratio(
                                        tile.photometry.light_level,
                                        illumination,
                                    ),
                                );
                                tile.photometry.light_level += illumination.max(0.0);

                                discovered_tiles.insert(*tile_position);
                            }
                        }
                    }
                }
            }
        }

        // Update player's discovered tiles
        if let Some(player_entity) = get_player_entity(&entities, &players) {
            if let Some(player_viewshed) = viewsheds.get_mut(player_entity) {
                for tile_position in discovered_tiles
                    .iter()
                    .filter(|tile_position| player_viewshed.visible_tiles.contains(tile_position))
                {
                    if let Some(tile) = map_tiles.get(tile_position) {
                        if tile.photometry.light_level >= 1.0 - player_viewshed.dark_vision {
                            player_viewshed.discovered_tiles.insert(*tile_position);
                        }
                    }
                }
            }
        };

        let now = time::Instant::now();
        //All affected photometry must be marked as dirty
        for (photometry, _position) in (&mut photometria, &positions)
            .join()
            .filter(|(_, position)| affected_tiles.contains(*position))
        {
            photometry.dirty = true;
        }

        // Update dirty photometry
        for (photometry, position, _) in (&mut photometria, &positions, &entities).join()
        //.filter(|(_, _, entity)| wires.get(*entity).is_none())
        {
            if photometry.dirty {
                photometry.dirty = false;

                if let Some(tile) = map_tiles.get_mut(position) {
                    photometry.light_color = tile.photometry.light_color;
                    photometry.light_level = tile.photometry.light_level;
                } else if let Some(tile) = map_tiles.get_mut(&(*position + Vector3i::new(0, 0, -1)))
                {
                    photometry.light_color = tile.photometry.light_color;
                    photometry.light_level = tile.photometry.light_level;
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
