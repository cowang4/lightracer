
use std::mem;

use enum_dispatch::enum_dispatch;

use crate::color::Color;
use crate::math::solve_quadratic;
use crate::point::Point;
use crate::ray::Ray;
use crate::scene::*;
use crate::vector::Vector3;


#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub dist: f32,
    pub color: Color,
    pub hit_light: bool,
    pub point: Point,
    pub normal: Vector3,
}

impl Hit {

    pub fn new(dist: f32, color: Color, hit_light: bool, point: Point, normal: Vector3) -> Hit {
        Hit {dist: dist, color: color, hit_light: hit_light, point: point, normal: normal,}
    }
}

#[enum_dispatch]
pub trait Intersectable : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    fn color(&self) -> Color;
    fn origin(&self) -> Point;
    fn intensity(&self) -> f32;

    fn is_light(&self) -> bool {
        self.intensity() > 0.0
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // Math from https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection

        // analytic solution
        let l: Vector3 = ray.origin - self.center;
        let a: f32  = ray.direction.dot(&ray.direction);
        let b: f32  = 2.0 * ray.direction.dot(&l);
        let c: f32 = l.dot(&l) - self.radius * self.radius;
        let (mut t0, mut t1) = solve_quadratic(a, b, c);
        if t0.is_none() && t1.is_none() {
            return None;
        }
        if t0 > t1 {
            mem::swap(&mut t0, &mut t1);
        }
        if t0 < Some(0.0) {
            t0 = t1; // If t0 is negative, let's use t1 instead
            if t0 < Some(0.0) {
                return None; // both are negative, so sphere is behind the ray origin
            }
        }
        match t0 {
            Some(t) => {
                let intersection_point: Point = ray.origin + t * ray.direction * 0.99999; // 0.9999 moves point slightly off of surface
                let dist = (intersection_point - ray.origin).length();
                let normal = (intersection_point - self.center) / self.radius;
                Some(Hit::new(dist, self.color, self.intensity > 0.0, intersection_point, normal))
            },
            None => None
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn origin(&self) -> Point {
        self.center
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }
}

impl Intersectable for Plane {

    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // math from https://stackoverflow.com/questions/23975555/how-to-do-ray-plane-intersection#23976134
        let projection: f32 = self.normal.dot(&ray.direction);
        if projection.abs() > 0.000001 {
            let t: f32 = (self.center - ray.origin).dot(&self.normal) / projection;
            if t >= 0.0000001 {
                let intersection_point: Point = ray.origin + t * ray.direction * 0.99999;
                let dist = (intersection_point - ray.origin).length();
                return Some(Hit::new(dist, self.color, self.intensity > 0.0, intersection_point, self.normal));
            }
        }
        None
    }

    fn color(&self) -> Color {
        self.color
    }

    fn origin(&self) -> Point {
        self.center
    }

    fn intensity(&self) -> f32 {
        self.intensity
    }
}


