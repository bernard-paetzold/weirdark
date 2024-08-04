use super::Vector3i;


pub fn get_neighbours(position: Vector3i) -> Vec<Vector3i> {
    vec![position + Vector3i::new(0, 0, -1),
        position + Vector3i::new(0, 0, 1),
        position + Vector3i::new(0, -1, 0),
        position + Vector3i::new(1, -1, 0),
        position + Vector3i::new(1, 0, 0),
        position + Vector3i::new(1, 1, 0),
        position + Vector3i::new(0, 1, 0),
        position + Vector3i::new(-1, 1, 0),
        position + Vector3i::new(-1, 0, 0),
        position + Vector3i::new(-1, -1, 0)]
}