
use std::f32;

use image::{Rgba};

use crate::color::Color;
use crate::intersectable::{Hit, Intersectable};
use crate::ray::Ray;
use crate::scene::*;


pub fn cast_ray(scene: &Scene, ray: &Ray) -> Option<Hit> {
    // loop over objects in the scene, find closest intersection
    scene.objects
        .iter()
        .filter_map(|obj| obj.intersect(ray))
        .min_by(|hit, hit2| hit.dist.partial_cmp(&hit2.dist).expect("float comparison"))
}

pub fn cast_camera_ray(scene: &Scene, camera_ray: &Ray) -> Rgba<u8> {
    let mut returned_color = Color::black();
    // if the camera ray hit an object
    if let Some(camera_hit) = cast_ray(scene, camera_ray) {
        // cast shadow rays to light sources
        for light in &scene.lights {
            let vec_to_light = light.origin() - camera_hit.point;
            let shadow_ray = Ray::from(&camera_hit.point, &vec_to_light);
            if let Some(shadow_hit) = cast_ray(scene, &shadow_ray) {
                let ambient_color = camera_hit.color * 0.002;
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
