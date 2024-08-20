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
        target_postion = Vector3i::new(rng::range(x_lower_limit, x_upper_limit ), y_lower_limit, z_lower_limit);
    }
    else if breaker_wall == 1 {
        target_postion = Vector3i::new(x_lower_limit, rng::range(y_lower_limit, y_upper_limit), z_lower_limit);
    }
    else if breaker_wall == 2 {
        target_postion = Vector3i::new(rng::range(x_lower_limit, x_upper_limit), y_upper_limit, z_lower_limit);
    }
    else if breaker_wall == 3 {
        target_postion = Vector3i::new(x_upper_limit, rng::range(y_lower_limit, y_upper_limit), z_lower_limit);
    }
    target_postion
}

pub struct Room {
    pub centre: Vector3i,
    pub size: Vector3i,
    pub door_sides: Vec<Vector3i>,
}

impl Room {
    pub fn new(centre: Vector3i, size: Vector3i, door_sides: Vec<Vector3i>) -> Self {
        Self {
            centre,
            size,
            door_sides,
        }
    }
}