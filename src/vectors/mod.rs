use std::{fmt, ops::{Add, AddAssign, Div, Mul, Sub, SubAssign}};

use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use specs::prelude::*;
use specs_derive::Component;

mod utils;
//pub use utils::find_path;


#[derive(Component, Copy, Clone, Debug, Default, Ord, PartialOrd, Eq, Hash, Serialize, Deserialize)]
pub struct Vector3i
{
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Vector3i {
    pub const UP: Vector3i = Vector3i { x:0, y: 0, z: 1 };
    pub const DOWN: Vector3i = Vector3i { x:0, y: 0, z: -1 };
    pub const TOP: Vector3i = Vector3i { x:0, y: 1, z: 0 };
    pub const BOTTOM: Vector3i = Vector3i { x:0, y: -1, z: 0 };
    pub const LEFT: Vector3i = Vector3i { x:-1, y: 0, z: 0 };
    pub const RIGHT: Vector3i = Vector3i { x:1, y: 0, z: 0 };

    pub fn new(x: i32, y: i32, z: i32) -> Vector3i 
    {
        Vector3i{x, y, z}
    }
    
    pub fn new_equi(size: i32) -> Vector3i 
    {
        Vector3i{ x: size, y: size, z: size}
    }


    pub fn manhattan(&self, other: Vector3i) -> i32 
    {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn distance_to(&self, other: Self) -> i32{
        (((self.x - other.x).pow(2) + (self.y - other.y).pow(2) + (self.z - other.z).pow(2)) as f32).sqrt().round() as i32
    }
}

/*impl Serialize for Vector3i {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer {
        let mut state = serializer.serialize_struct("Vector3i", 3)?;
    
        state.serialize_field("x", &self.x)?;
        state.serialize_field("y", &self.y)?;
        state.serialize_field("z", &self.z)?;
        state.end()
    }
}*/

impl Add for Vector3i {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Vector3i::new(self.x + other.x, self.y + other.y, self.z + other.z);
    }
}

impl AddAssign for Vector3i {
    fn add_assign(&mut self, other: Self) {
        *self = Self{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z};
    }
}

impl Sub for Vector3i {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Vector3i::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign for Vector3i {
    fn sub_assign(&mut self, other: Self) {
        *self = Self{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z};
    }
}

impl Div<i32> for Vector3i {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        return Vector3i::new(self.x / other, self.y / other, self.z / other)
    }
}

impl Mul<i32> for Vector3i {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        return Vector3i::new(self.x * other, self.y * other, self.z * other)
    }
}

impl Mul<Vector3i> for i32 {
    type Output = Vector3i;

    fn mul(self, other: Vector3i) -> Vector3i {
        return Vector3i::new(other.x * self, other.y * self, other.z * self)
    }
}

impl fmt::Display for Vector3i {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}

impl PartialEq for Vector3i {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other .z
    }
}
