
use enum_dispatch::enum_dispatch;

use crate::color::Color;
use crate::intersectable::{Intersectable, Hit};
use crate::point::Point;
use crate::ray::Ray;
use crate::vector::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub color: Color,
    pub intensity: f32, // intensity of outputted light. 0.0 for none
}

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    pub center: Point,
    pub normal: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[enum_dispatch(Intersectable)]
#[derive(Debug, Copy, Clone)]
pub enum Object {
    Sphere,
    Plane,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub objects: Vec<Object>,
    pub lights: Vec<Object>,
}

impl Scene {
    
    pub fn new(width: u32, height: u32, fov: f32, objects: Vec<Object>) -> Scene {
        Scene {
            width: width,
            height: height,
            fov: fov,
            objects: objects.clone(),
            lights: objects.iter().filter(|o| o.is_light()).map(|o| o.clone()).collect(),
        }
    }
}
