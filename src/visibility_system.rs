use specs::prelude::*;

use crate::{vectors::Vector3i, Viewshed, MAP_SIZE};

pub struct VisibilitySystem {}

impl<'a> System<'a> for VisibilitySystem {
    type SystemData = (WriteStorage<'a, Viewshed>,
                        WriteStorage<'a, Vector3i>);

    fn run(&mut self, (mut viewshed, position) : Self::SystemData) {
        for (viewshed, position) in (&mut viewshed, &position).join() {
            viewshed.visible_tiles.clear();

            for x in (position.x - viewshed.view_distance)..(position.x + viewshed.view_distance) {
                for y in (position.y - viewshed.view_distance)..(position.y + viewshed.view_distance) {
                    for z in position.z - 1..position.z + 1 {
                        let target = Vector3i::new(x, y, z);

                        if position.distance_to(target) < viewshed.view_distance {
                            viewshed.visible_tiles.push(Vector3i::new(x, y, z));
                            viewshed.visible_tiles.retain(|p| p.x >= 0 && p.x < MAP_SIZE && p.y >= 0 && p.y < MAP_SIZE && p.z >= 0 && p.z < MAP_SIZE);
                        }
                    }
                }
            }
            //println!("{}",viewshed.visible_tiles.len());
        }
    }

}