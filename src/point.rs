use crate::vector3::Vector3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector3;
    fn sub(self, other: Point) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add<Vector3> for Point {
    type Output = Point;
    fn add(self, other: Vector3) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
