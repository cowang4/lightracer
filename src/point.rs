use std::ops::{Add, Sub};

use serde_derive::{Deserialize};

use crate::vector::Vector3;

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Point {
    pub fn zero() -> Point {
        Point::from_one(0.0)
    }

    pub fn from_one(v: f32) -> Point {
        Point { x: v, y: v, z: v }
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
impl Add<Point> for Vector3 {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        other + self
    }
}

impl Sub<Vector3> for Point {
    type Output = Point;

    fn sub(self, other: Vector3) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Sub<Point> for Vector3 {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        other - self
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

impl From<Vector3> for Point {
    fn from(v: Vector3) -> Point {
        Point {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
