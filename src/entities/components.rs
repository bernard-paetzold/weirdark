use std::collections::HashSet;

use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

use crate::vectors::Vector3i;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: HashSet<Vector3i>,
    pub visible_entities: HashSet<Entity>,
    pub view_distance: i32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(view_distance: i32) -> Viewshed {
        Viewshed {
            visible_tiles: HashSet::new(),
            visible_entities: HashSet::new(),
            view_distance,
            dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Illuminant {
    pub intensity: f32,
    pub color: RGB,
    pub beam_angle: f32,
    pub dirty: bool,
}

impl Illuminant {
    pub fn new(intensity: f32, color: RGB, beam_angle: f32) -> Illuminant {
        Illuminant {
            intensity,
            color,
            beam_angle,
            dirty: true,
        }
    } 
}


#[derive(Component)]
pub struct Photometry {
    pub light_level: f32,
    pub light_color: RGB,
}

//TODO: Once lighting is calculated set initial light level to 0.0
impl Photometry {
    pub fn new() -> Photometry {
        Photometry {
            light_level:  1.0,
            light_color: RGB::named(rltk::WHITE),
        }
    } 
}

