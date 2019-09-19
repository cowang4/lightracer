
use enum_dispatch::enum_dispatch;

use crate::color::Color;
use crate::point::Point;
use crate::rendering::Intersectable;
use crate::vector::Vector3;

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

#[enum_dispatch(Intersectable)]
pub enum Object {
    Sphere,
    Plane,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn lights(&self) -> Vec<&Object> {
        self.objects.iter().filter(|o| o.is_light()).collect()
    }
}
