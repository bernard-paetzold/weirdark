use specs::prelude::*;
use specs_derive::Component;

use crate::vectors::Vector3i;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Vector3i>,
    pub view_distance: i32,
}

impl Viewshed {
    pub fn new(view_distance: i32) -> Viewshed {
        Viewshed {
            visible_tiles: Vec::new(),
            view_distance,
        }
    }
}