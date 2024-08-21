use super::Vector3i;

#[allow(dead_code)]
pub fn get_neighbours(position: Vector3i) -> Vec<Vector3i> {
    vec![
        position + Vector3i::new(0, -1, 0),
        position + Vector3i::new(1, -1, 0),
        position + Vector3i::new(1, 0, 0),
        position + Vector3i::new(1, 1, 0),
        position + Vector3i::new(0, 1, 0),
        position + Vector3i::new(-1, 1, 0),
        position + Vector3i::new(-1, 0, 0),
        position + Vector3i::new(-1, -1, 0),
    ]
}

#[allow(dead_code)]
pub fn get_neighbours_with_z(position: Vector3i) -> Vec<Vector3i> {
    vec![
        position + Vector3i::new(0, 0, -1),
        position + Vector3i::new(0, 0, 1),
        position + Vector3i::new(0, -1, 0),
        position + Vector3i::new(1, -1, 0),
        position + Vector3i::new(1, 0, 0),
        position + Vector3i::new(1, 1, 0),
        position + Vector3i::new(0, 1, 0),
        position + Vector3i::new(-1, 1, 0),
        position + Vector3i::new(-1, 0, 0),
        position + Vector3i::new(-1, -1, 0),
    ]
}

pub fn get_cardinal_neighbours(position: Vector3i) -> Vec<Vector3i> {
    vec![
        position + Vector3i::new(0, -1, 0),
        position + Vector3i::new(1, 0, 0),
        position + Vector3i::new(0, 1, 0),
        position + Vector3i::new(-1, 0, 0),
    ]
}

pub fn get_cardinal_neighbours_with_z(position: Vector3i) -> Vec<Vector3i> {
    vec![
        position + Vector3i::new(0, 0, -1),
        position + Vector3i::new(0, 0, 1),
        position + Vector3i::new(0, -1, 0),
        position + Vector3i::new(1, 0, 0),
        position + Vector3i::new(0, 1, 0),
        position + Vector3i::new(-1, 0, 0),
    ]
}

#[allow(dead_code)]
pub fn position_in_over_under(position: Vector3i, target: Vector3i) -> bool {
    position == target
        || position == (target + Vector3i::UP)
        || position == (target + Vector3i::DOWN)
}
