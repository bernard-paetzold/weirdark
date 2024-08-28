use std::collections::HashSet;

use crate::vectors::Vector3i;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AreaType {
    Corridor,
    GenericRoom,
    GeneratorRoom,
}

pub trait Area {
    fn get_area_id(&self) -> usize;
    fn get_area_name(&self) -> &String;
    fn get_area_position(&self) -> &Vector3i;
    fn get_nodes(&mut self) -> &mut Vec<Vector3i>;
    fn get_breaker_pos(&self) -> &Option<Vector3i>;
    fn set_breaker_pos(&mut self, breaker_position: Vector3i);
    fn get_power_connections(&self) -> &Vec<Vector3i>;
    fn update_power_connections(&mut self) -> &mut Vec<Vector3i>;
    fn get_size(&self) -> &Vector3i;
    fn get_area_type(&self) -> AreaType;
    fn set_area_type(&mut self, area_type: AreaType);
}

#[derive(Clone)]
pub struct Room {
    pub centre: Vector3i,
    pub size: Vector3i,
    pub area_id: usize,
    pub area_name: String,
    pub area_type: AreaType,
    pub nodes: Vec<Vector3i>,
    pub breaker_position: Option<Vector3i>,
    pub power_connections: Vec<Vector3i>,
}

impl Room {
    pub fn new(
        centre: Vector3i,
        size: Vector3i,
        area_name: String,
        area_type: AreaType,
        power_connection: bool,
    ) -> Self {
        Self {
            centre,
            size,
            area_id: crate::rng::random_int() as usize,
            area_name,
            area_type,
            nodes: Vec::new(),
            breaker_position: if power_connection {
                Some(Vector3i::new_equi(0))
            } else {
                None
            },
            power_connections: Vec::new(),
        }
    }
}

impl Area for Room {
    fn get_area_id(&self) -> usize {
        self.area_id
    }

    fn get_area_name(&self) -> &String {
        &self.area_name
    }

    fn get_nodes(&mut self) -> &mut Vec<Vector3i> {
        &mut self.nodes
    }

    fn get_breaker_pos(&self) -> &Option<Vector3i> {
        &self.breaker_position
    }

    fn set_breaker_pos(&mut self, breaker_position: Vector3i) {
        self.breaker_position = Some(breaker_position);
    }

    fn get_area_position(&self) -> &Vector3i {
        &self.centre
    }

    fn update_power_connections(&mut self) -> &mut Vec<Vector3i> {
        &mut self.power_connections
    }

    fn get_power_connections(&self) -> &Vec<Vector3i> {
        &self.power_connections
    }

    fn get_area_type(&self) -> AreaType {
        self.area_type
    }

    fn set_area_type(&mut self, area_type: AreaType) {
        self.area_type = area_type;
    }

    fn get_size(&self) -> &Vector3i {
        &self.size
    }
}

#[derive(Clone)]
pub struct Corridor {
    pub start: Vector3i,
    pub end: Vector3i,
    pub area_id: usize,
    pub area_name: String,
    pub area_type: AreaType,
    pub width: usize,
    pub connected_areas: HashSet<usize>,
    pub nodes: Vec<Vector3i>,
    pub breaker_position: Option<Vector3i>,
    pub power_connections: Vec<Vector3i>,
    pub size: Vector3i,
}

impl Corridor {
    pub fn new(
        start: Vector3i,
        end: Vector3i,
        width: usize,
        area_name: String,
        area_type: AreaType,
        power_connection: bool,
    ) -> Self {
        Self {
            start,
            end,
            area_id: crate::rng::random_int() as usize,
            area_name,
            area_type,
            width,
            connected_areas: HashSet::new(),
            nodes: Vec::new(),
            breaker_position: if power_connection {
                Some(Vector3i::new_equi(0))
            } else {
                None
            },
            power_connections: Vec::new(),
            size: Vector3i::new_equi(width as i32),
        }
    }
}

impl Area for Corridor {
    fn get_area_id(&self) -> usize {
        self.area_id
    }

    fn get_area_name(&self) -> &String {
        &self.area_name
    }

    fn get_nodes(&mut self) -> &mut Vec<Vector3i> {
        &mut self.nodes
    }

    fn get_breaker_pos(&self) -> &Option<Vector3i> {
        &self.breaker_position
    }

    fn set_breaker_pos(&mut self, breaker_position: Vector3i) {
        self.breaker_position = Some(breaker_position);
    }

    fn get_area_position(&self) -> &Vector3i {
        &self.start
    }

    fn update_power_connections(&mut self) -> &mut Vec<Vector3i> {
        &mut self.power_connections
    }

    fn get_power_connections(&self) -> &Vec<Vector3i> {
        &self.power_connections
    }

    fn get_area_type(&self) -> AreaType {
        self.area_type
    }

    fn set_area_type(&mut self, area_type: AreaType) {
        self.area_type = area_type;
    }

    fn get_size(&self) -> &Vector3i {
        &self.size
    }
}
