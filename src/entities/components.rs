use std::collections::HashSet;

use specs::prelude::*;
use specs_derive::Component;

use crate::vectors::Vector3i;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: HashSet<Vector3i>,
    pub view_distance: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(view_distance: i32) -> Viewshed {
        Viewshed {
            visible_tiles: HashSet::new(),
            view_distance,
            dirty: true,
        }
    }
}