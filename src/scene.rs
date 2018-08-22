

use color::Color;
use point::Point;
use rendering::Intersectable;
use vector::Vector3;

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
    pub intensity: f32, // intensity of outputted light. 0.0 for none
}

pub struct Plane {
    pub center: Point,
    pub normal: Vector3,
    pub color: Color,
    pub intensity: f32,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Box<Intersectable>>,
}

impl Scene {
    pub fn lights(&self) -> Vec<&Box<Intersectable>> {
        self.objects.iter().filter(|o| o.is_light()).collect()
    }
}
