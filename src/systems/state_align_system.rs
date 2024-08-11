use specs::prelude::*;

use crate::{
    vectors::Vector3i, Blocker, Door, Map, Renderable, Viewshed, VisionBlocker
};

pub struct StateAlignSystem {}

impl<'a> System<'a> for StateAlignSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Door>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Blocker>,
        WriteStorage<'a, VisionBlocker>,
        ReadStorage<'a, Vector3i>,
        WriteStorage<'a, Viewshed>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            _map,
            mut doors,
            mut renderables,
            mut blockers,
            mut vision_blockers,
            positions,
            mut viewsheds,
            entities
        ) = data;

        let mut affected_tiles = Vec::new();
        //Align door states
        for (door, entity, position) in (&mut doors, &entities, &positions).join() {
            if door.open {
                if let Some(renderable) = renderables.get_mut(entity) {
                    renderable.side_glyph = door.open_glyph;
                    renderable.top_glyph = door.open_glyph;
                }

                if let Some(_blocker) = blockers.get(entity) {
                    blockers.remove(entity);

                    //Door state changed since blocker was removed
                    affected_tiles.push(position);
                }

                if let Some(_vision_blocker) = vision_blockers.get(entity) {
                    vision_blockers.remove(entity);
                }
            }
            else {
                if let Some(renderable) = renderables.get_mut(entity) {
                    renderable.side_glyph = door.closed_glyph;
                    renderable.top_glyph = door.closed_glyph;

                    //If the door is opaque, block vision
                    if renderable.foreground.a >= 1.0 {
                        vision_blockers.insert(entity, VisionBlocker::new_all_sides()).expect("Error inserting component");
                    }
                }
                let result = blockers.insert(entity, Blocker::new_all_sides()).expect("Error inserting component");

                match result {
                    None => {
                        //Door state changed
                        affected_tiles.push(position);
                    }
                    _ => {}
                }   
            }
        }

        //Rebuild viewsheds
        for (viewshed, position) in (&mut viewsheds, &positions).join() {
            if affected_tiles.iter().filter(|affected_tile| affected_tile.distance_to_int(*position) < viewshed.view_distance as i32).next().is_some() {
                viewshed.dirty = true;
            }
        }
    }
}
