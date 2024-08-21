use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    f32::consts::SQRT_2,
};

use crate::vectors::{utils::get_cardinal_neighbours, Vector3i};

use super::Map;

#[allow(dead_code)]
pub fn find_walkable_path(map: Map, start_position: Vector3i, target: Vector3i) -> Vec<Vector3i> {
    if let Some(path) = a_star(&map, start_position, target) {
        path
    } else {
        println!("Failed");
        vec![start_position]
    }
}

pub fn find_path_with_width(
    map: Map,
    start_position: Vector3i,
    target: Vector3i,
    width: usize,
) -> Vec<Vector3i> {
    if let Some(path) = a_star_with_width(&map, start_position, target, width) {
        path
    } else {
        println!("Failed");
        vec![start_position]
    }
}

pub fn wall_climb_path(
    map: Map,
    mut start_position: Vector3i,
    mut target: Vector3i,
    roof_preferred: bool,
) -> Vec<Vector3i> {
    let mut path: Vec<Vector3i> = Vec::new();
    let direction: i32;

    if roof_preferred {
        let temp = start_position;
        start_position = target;
        target = temp;
    }

    if start_position.z > target.z {
        direction = 1;
    } else {
        direction = -1;
    }

    // Find nearest wall to crawl up
    let mut wall_found = false;
    let mut wall_position = Vector3i::new_equi(0);
    let mut wall_climb: Vec<Vector3i> = Vec::new();
    let mut unchecked_tiles: BinaryHeap<Reverse<(i32, Vector3i)>> = BinaryHeap::new();
    let mut checked_tiles: HashSet<Vector3i> = HashSet::new();

    unchecked_tiles.push(Reverse((0, start_position)));

    while let Some(Reverse((distance, test_position))) = unchecked_tiles.pop() {
        if checked_tiles.contains(&test_position) {
            continue;
        }

        checked_tiles.insert(test_position);

        let neighbours = get_cardinal_neighbours(test_position);

        for neighbour in neighbours.iter() {
            if let Some(neighbour_tile) = map.tiles.get(neighbour) {
                if !neighbour_tile.passable {
                    // Tile is next to a wall, check downwards to see if it reaches the target z level
                    let mut test_wall_position: Vector3i = *neighbour;
                    let mut wall_invalid = false;

                    wall_climb = Vec::new();

                    while (target.z.abs() - test_wall_position.z.abs()) >= 0 && !wall_invalid {
                        let down_neighbours = get_cardinal_neighbours(test_wall_position);
                        let mut invalid_walls = 0;

                        for down_neighbour in down_neighbours.iter() {
                            if let Some(down_neighbour_tile) = map.tiles.get(down_neighbour) {
                                if down_neighbour_tile.passable {
                                    invalid_walls += 1;
                                }
                            }
                        }

                        if invalid_walls < 4 {
                            wall_climb.push(Vector3i::new(
                                test_position.x,
                                test_position.y,
                                test_wall_position.z,
                            ));
                            test_wall_position =
                                test_wall_position + Vector3i::new(0, 0, -direction);
                        } else {
                            wall_invalid = true;
                        }
                    }

                    if !wall_invalid {
                        wall_found = true;
                        wall_position = test_position;
                        break;
                    }
                } else {
                    let new_distance = distance + 1 + target.distance_to_int(*neighbour);
                    unchecked_tiles.push(Reverse((new_distance, *neighbour)));
                }
            }
        }

        if wall_found {
            break;
        }
    }

    if wall_found {
        if let Some(mut path_to_wall) = a_star(&map, start_position, wall_position) {
            path.append(&mut path_to_wall);
        }

        let wall_end_position = wall_climb.last().cloned();

        path.append(&mut wall_climb);

        if let Some(wall_end_position) = wall_end_position {
            if let Some(mut path_to_target) = a_star(&map, wall_end_position, target) {
                path_to_target.remove(0);
                path.append(&mut path_to_target);
            }
        }
    }
    path
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct State {
    g_score: i32,
    position: Vector3i,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.g_score
            .cmp(&other.g_score)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn a_star(map: &Map, start_position: Vector3i, target: Vector3i) -> Option<Vec<Vector3i>> {
    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse(State {
        g_score: 0,
        position: start_position,
    }));

    let mut came_from: HashMap<Vector3i, Vector3i> = HashMap::new();
    let mut g_score: HashMap<Vector3i, i32> = HashMap::new();
    g_score.insert(start_position, 0);
    let mut f_score: HashMap<Vector3i, f32> = HashMap::new();
    f_score.insert(start_position, heuristic(start_position, target));

    while let Some(Reverse(State {
        g_score: _current_g_score,
        position: current_position,
    })) = open_set.pop()
    {
        if current_position == target {
            return Some(reconstruct_path(&came_from, current_position));
        }

        let neighbours = get_accessible_neighbours(map, current_position);

        for neighbour in neighbours {
            let move_cost =
                if current_position.x != neighbour.x && current_position.y != neighbour.y {
                    (SQRT_2 * 100.0) as i32 // Diagonal move
                } else {
                    100 // Cardinal move
                };

            let tentative_g_score = g_score[&current_position] + move_cost;
            let tentative_f_score = tentative_g_score as f32 + heuristic(neighbour, target);

            if !g_score.contains_key(&neighbour) || tentative_g_score < g_score[&neighbour] {
                came_from.insert(neighbour, current_position);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_f_score);

                open_set.push(Reverse(State {
                    g_score: tentative_g_score,
                    position: neighbour,
                }));
            }
        }
    }
    None
}

fn a_star_with_width(
    map: &Map,
    start_position: Vector3i,
    target: Vector3i,
    width: usize,
) -> Option<Vec<Vector3i>> {
    let mut open_set = BinaryHeap::new();
    open_set.push(Reverse(State {
        g_score: 0,
        position: start_position,
    }));

    let mut came_from: HashMap<Vector3i, Vector3i> = HashMap::new();
    let mut g_score: HashMap<Vector3i, i32> = HashMap::new();
    g_score.insert(start_position, 0);
    let mut f_score: HashMap<Vector3i, f32> = HashMap::new();
    f_score.insert(start_position, heuristic(start_position, target));

    let mut count = 0;

    while let Some(Reverse(State {
        g_score: _current_g_score,
        position: current_position,
    })) = open_set.pop()
    {
        if current_position == target {
            return Some(reconstruct_path(&came_from, current_position));
        }

        let neighbours = if count == 0 || open_set.is_empty() {
            get_accessible_neighbours(map, current_position)
        } else {
            get_accessible_neighbours_with_width(map, current_position, width)
        };

        for neighbour in neighbours {
            let move_cost =
                if current_position.x != neighbour.x && current_position.y != neighbour.y {
                    (SQRT_2 * 100.0) as i32 // Diagonal move
                } else {
                    100 // Cardinal move
                };
            let tentative_g_score = g_score[&current_position] + move_cost;

            let tentative_f_score = tentative_g_score as f32 + heuristic(neighbour, target);

            if !g_score.contains_key(&neighbour) || tentative_g_score < g_score[&neighbour] {
                came_from.insert(neighbour, current_position);
                g_score.insert(neighbour, tentative_g_score);
                f_score.insert(neighbour, tentative_f_score);

                open_set.push(Reverse(State {
                    g_score: tentative_g_score,
                    position: neighbour,
                }));
            }
        }
        count += 1;
    }
    None
}

pub fn get_accessible_neighbours(map: &Map, position: Vector3i) -> Vec<Vector3i> {
    let mut neighbours = get_cardinal_neighbours(position);
    let mut accessible_neighbours = Vec::new();

    while neighbours.len() > 0 {
        let neighbour = neighbours.pop();

        if let Some(tile_position) = neighbour {
            if let Some(tile) = map.tiles.get(&tile_position) {
                if tile.passable {
                    accessible_neighbours.push(tile_position);
                }
            } else {
                accessible_neighbours.push(tile_position);
            }
        }
    }
    accessible_neighbours
}

pub fn get_accessible_neighbours_with_width(
    map: &Map,
    position: Vector3i,
    range: usize,
) -> Vec<Vector3i> {
    let mut neighbours = get_cardinal_neighbours(position);
    let mut accessible_neighbours = Vec::new();

    while neighbours.len() > 0 {
        let neighbour = neighbours.pop();

        let mut valid_tile = true;

        let half_range = (range / 2) as i32;

        if let Some(tile_position) = neighbour {
            for x in -half_range..=half_range {
                for y in -half_range..=half_range {
                    if let Some(tile) = map.tiles.get(&(tile_position + Vector3i::new(x, y, 0))) {
                        if !tile.passable && !(x.abs() == half_range || y.abs() == half_range) {
                            valid_tile = false;
                        }
                    }
                }
            }

            if valid_tile {
                accessible_neighbours.push(tile_position);
            }
        }
    }
    accessible_neighbours
}

fn heuristic(position: Vector3i, target: Vector3i) -> f32 {
    let dx = (position.x - target.x).abs();
    let dy = (position.y - target.y).abs();

    (dx * dy) as f32 + (SQRT_2 - 2.0) * dx.min(dy) as f32
}

fn reconstruct_path(came_from: &HashMap<Vector3i, Vector3i>, target: Vector3i) -> Vec<Vector3i> {
    let mut total_path = Vec::new();
    let mut current_position = target;

    while let Some(previous_position) = came_from.get(&current_position) {
        total_path.insert(0, current_position);
        current_position = *previous_position;
    }

    total_path.insert(0, current_position);
    total_path
}
