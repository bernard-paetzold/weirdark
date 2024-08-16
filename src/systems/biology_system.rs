use specs::prelude::*;

use crate::{entities::biology::Breather, vectors::Vector3i, Map};



pub struct BiologySystem {}

impl<'a> System<'a> for BiologySystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Vector3i>,
        WriteStorage<'a, Breather>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            positions,
            mut breathers,
        ) = data;

        for (breather, position) in (&mut breathers, &positions).join().filter(|(x, _)| x.trigger_breath) {
            breather.trigger_breath = false;

            if let Some(tile) = map.tiles.get_mut(position) {
                  breather.breath(&mut tile.atmosphere);
            }
        }
    }
}