
use image::{Rgba};

use point::Point;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Rgba<u8>,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}
