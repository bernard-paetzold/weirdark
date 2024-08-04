use specs::{Entities, Join, ReadStorage, System, WriteExpect};

use crate::{vectors::Vector3i, Map};


pub struct MapIndexSystem {}

impl<'a> System<'a> for MapIndexSystem {
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadStorage<'a, Vector3i>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            positions,
            entities,
        ) = data;   

        map.clear_map_entities();

        for (entity, position) in (&entities, &positions).join() {
            map.entities.insert(*position, entity);
        }
    }
}