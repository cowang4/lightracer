
use std::f64;
use std::f32;
use std::mem;

use image::{Rgba};
use rayon::prelude::*;

use color::Color;
use point::Point;
use vector::Vector3;
use scene::*;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    /// By convention, the camera is aligned along the negative z-axis, with positive x towards the right and positive y being up.
    /// That’s why the sphere is at (0.0, 0.0, -5.0) - it’s directly centered, five units away from the camera.
    /// We’ll start by pretending there’s a two-unit by two-unit square one unit in front of the camera.
    /// This square represents the image sensor or film of our camera.
    /// Then we’ll divide that sensor square into pixels, and use the directions to each pixel as our rays.
    /// We need to translate the (0…800, 0…600) coordinates of our pixels to the (-1.0…1.0, -1.0…1.0) coordinates of the sensor.
    pub fn create_camera(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan(); //Field of view is the angle between the left-most ray and the right-most ray
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);

        let x_pixel_center = x as f64 + 0.5; //add 0.5 (one half-pixel) because we want our ray to pass through the center (rather than the corner) of the pixel on our imaginary sensor
        let x_normalized_to_width = x_pixel_center / scene.width as f64; //Then we divide by the image width to convert from our original coordinates (0…800) to (0.0…1.0)
        let x_adjusted_screen_pos = (x_normalized_to_width * 2.0) - 1.0; //the (-1.0…1.0) coordinates we want, so we multiply by two and subtract one
        let sensor_x = (x_adjusted_screen_pos * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment; //  the image coordinates have positive y meaning down, where we want positive y to be up, so we negate

        Ray {
            origin: Point::zero(), // Assumes camera's origin at 0,0,0
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0, //Camera is looking down negative z
            }
            .normalize(),
        }
    }

    pub fn from(origin: &Point, direction: &Vector3) -> Ray {
        Ray {origin: origin.clone(), direction: direction.clone().normalize()}
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Hit {
    pub dist: f64,
    pub color: Color,
    pub hit_light: bool,
    pub point: Point,
    pub normal: Vector3,
}

impl Hit {

    pub fn new(dist: f64, color: Color, hit_light: bool, point: Point, normal: Vector3) -> Hit {
        Hit {dist: dist, color: color, hit_light: hit_light, point: point, normal: normal,}
    }
}

pub trait Intersectable : Send + Sync {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;

    fn color(&self) -> Color;
    fn origin(&self) -> Point;
    fn intensity(&self) -> f32;

    fn is_light(&self) -> bool {
        self.intensity() > 0.0
    }
}

fn solve_quadratic(a: f64, b: f64, c: f64) -> (Option<f64>, Option<f64>) {
    let x0;
    let x1;
    let discr = b * b - 4.0 * a * c;
    if discr < 0.0 {
        return (None, None);
    } else if discr == 0.0 {
        x0 = Some(-0.5 * b / a);
        x1 = Some(-0.5 * b / a);
    } else {
        let q = match b > 0.0 {
            true    => -0.5 * (b + discr.sqrt()),
            false   => -0.5 * (b - discr.sqrt()),
        };
        x0 = Some(q / a);
        x1 = Some(c / q);
    }
    (x0, x1)
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        // Math from https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection

        // analytic solution
        let l: Vector3 = ray.origin - self.center;
        let a: f64  = ray.direction.dot(&ray.direction);
        let b: f64  = 2.0 * ray.direction.dot(&l);
        let c: f64 = l.dot(&l) - self.radius * self.radius;
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
                let intersection_point: Point = ray.origin + t * ray.direction * 0.99999999; // 0.9999 moves point slightly off of surface
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
        let projection: f64 = self.normal.dot(&ray.direction);
        if projection.abs() > 0.000001 {
            let t: f64 = (self.center - ray.origin).dot(&self.normal) / projection;
            if t >= 0.0000001 {
                let intersection_point: Point = ray.origin + t * ray.direction * 0.99999999;
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


pub fn cast_ray(scene: &Scene, ray: &Ray) -> Option<Hit> {
    // loop over objects in the scene, find closest intersection
    scene.objects
        .par_iter()
        .filter_map(|obj| obj.intersect(ray))
        .min_by(|hit, hit2| hit.dist.partial_cmp(&hit2.dist).expect("float comparison"))
}

pub fn cast_camera_ray(scene: &Scene, camera_ray: &Ray) -> Rgba<u8> {
    let mut returned_color = Color::black();
    // if the camera ray hit an object
    if let Some(camera_hit) = cast_ray(scene, camera_ray) {
        // cast shadow rays to light sources
        for ref light in scene.objects.iter().filter(|o| o.is_light()) {
            let vec_to_light = light.origin() - camera_hit.point;
            let shadow_ray = Ray::from(&camera_hit.point, &vec_to_light);
            if let Some(shadow_hit) = cast_ray(scene, &shadow_ray) {
                let ambient_color = camera_hit.color * 0.003;
                if shadow_hit.hit_light && shadow_hit.color == light.color() {
                    let cos_term = vec_to_light.normalize().dot(&camera_hit.normal).max(0.0) as f32;
                    returned_color += camera_hit.color * light.color() * light.intensity() * cos_term * (1.0 / f32::consts::PI) / (vec_to_light.length() as f32) + ambient_color;
                }
                else {
                    returned_color += ambient_color;
                }
            }
        }
    }
    returned_color.into()
}
