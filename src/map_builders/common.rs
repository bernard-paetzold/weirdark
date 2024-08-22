use std::collections::HashSet;

use crate::{rng, vectors::Vector3i};

pub fn rand_wall_adj_tile(room_center: Vector3i, size: Vector3i) -> Vector3i {
    let x_lower_limit = room_center.x - size.x / 2 + 1;
    let x_upper_limit = room_center.x + size.x / 2 - 1;

    let y_lower_limit = room_center.y - size.y / 2 + 1;
    let y_upper_limit = room_center.y + size.y / 2 - 1;

    let z_lower_limit = room_center.z - size.z / 2 + 1;
    //let z_upper_limit = room_center.z + size.z / 2 - 1;

    let breaker_wall = rng::range(0, 4);

    let mut target_postion = Vector3i::new_equi(0);

    if breaker_wall == 0 {
        target_postion = Vector3i::new(
            rng::range(x_lower_limit, x_upper_limit),
            y_lower_limit,
            z_lower_limit,
        );
    } else if breaker_wall == 1 {
        target_postion = Vector3i::new(
            x_lower_limit,
            rng::range(y_lower_limit, y_upper_limit),
            z_lower_limit,
        );
    } else if breaker_wall == 2 {
        target_postion = Vector3i::new(
            rng::range(x_lower_limit, x_upper_limit),
            y_upper_limit,
            z_lower_limit,
        );
    } else if breaker_wall == 3 {
        target_postion = Vector3i::new(
            x_upper_limit,
            rng::range(y_lower_limit, y_upper_limit),
            z_lower_limit,
        );
    }
    target_postion
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
}

#[derive(Clone)]
pub struct Room {
    pub centre: Vector3i,
    pub size: Vector3i,
    pub area_id: usize,
    pub area_name: String,
    pub nodes: Vec<Vector3i>,
    pub breaker_position: Option<Vector3i>,
    pub power_connections: Vec<Vector3i>,
}

impl Room {
    pub fn new(centre: Vector3i, size: Vector3i, name: String, power_connection: bool) -> Self {
        Self {
            centre,
            size,
            area_id: crate::rng::random_int() as usize,
            area_name: name,
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
}

#[derive(Clone)]
pub struct Corridor {
    pub start: Vector3i,
    pub end: Vector3i,
    pub area_id: usize,
    pub area_name: String,
    pub width: usize,
    pub connected_areas: HashSet<usize>,
    pub nodes: Vec<Vector3i>,
    pub breaker_position: Option<Vector3i>,
    pub power_connections: Vec<Vector3i>,
}

impl Corridor {
    pub fn new(
        start: Vector3i,
        end: Vector3i,
        width: usize,
        name: String,
        power_connection: bool,
    ) -> Self {
        Self {
            start,
            end,
            area_id: crate::rng::random_int() as usize,
            area_name: name,
            width,
            connected_areas: HashSet::new(),
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
}
