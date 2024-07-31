use std::collections::HashSet;

use rltk::{RGB, RGBA};
use specs::prelude::*;
use specs_derive::Component;

use crate::vectors::Vector3i;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: HashSet<Vector3i>,
    pub z_range: usize,
    pub view_distance: usize,
    pub dark_vision: f32,
    pub dirty: bool,
}

impl Viewshed {
    pub fn new(view_distance: usize, z_range: usize, dark_vision: f32) -> Viewshed {
        Viewshed {
            visible_tiles: HashSet::new(),
            z_range,
            view_distance,
            dark_vision,
            dirty: true,
        }
    }
}

#[derive(Component)]
pub struct Illuminant {
    pub intensity: f32,
    pub range: usize,
    pub color: RGBA,
    pub beam_angle: f32,
    pub on: bool,
    pub dirty: bool,
}

impl Illuminant {
    pub fn new(intensity: f32, range: usize, color: RGBA, beam_angle: f32, on: bool) -> Illuminant {
        Illuminant {
            intensity,
            range,
            color,
            beam_angle,
            on,
            dirty: true,
        }
    } 
}


#[derive(Component)]
pub struct Photometry {
    pub light_level: f32,
    pub light_color: RGBA,
    pub dirty: bool,
}

//TODO: Once lighting is calculated set initial light level to 0.0
impl Photometry {
    pub fn new() -> Photometry {
        Photometry {
            light_level:  1.0,
            light_color: RGB::named(rltk::WHITE).to_rgba(1.0),
            dirty: true,
        }
    }
}

