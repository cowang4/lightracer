
mod color;
mod intersectable;
mod math;
mod point;
mod ray;
mod rendering;
mod scene;
mod vector;

use std::path::PathBuf;

use image::{DynamicImage, GenericImage};

use crate::color::Color;
use crate::point::Point;
use crate::ray::Ray;
use crate::rendering::{cast_camera_ray};
use crate::scene::*;
use crate::vector::Vector3;

// this project was inspired/copied from https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let camera_ray = Ray::create_camera(x, y, scene);
            let color = cast_camera_ray(scene, &camera_ray);
            image.put_pixel(x, y, color);
        }
    }
    image
}

fn main() {
    let scene = Scene::new(
        1920,
        1080,
        90.0,
        vec![
            Object::from(
                Plane {
                    center: Point {
                        x: 0.0,
                        y: -1.0,
                        z: 0.0,
                    },
                    normal: Vector3 {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                    color: Color {
                        red: 0.5,
                        green: 0.5,
                        blue: 0.5,
                    },
                    intensity: 0.0,
                }
            ),
            Object::from(
                Sphere {
                    center: Point {
                        x: 0.0,
                        y: 0.0,
                        z: -5.0,
                    },
                    radius: 1.0,
                    color: Color {
                        red: 0.1,
                        green: 0.1,
                        blue: 0.9,
                    },
                    intensity: 0.0,
                }
            ),
            Object::from(
                Sphere {
                    center: Point {
                        x: 1.0,
                        y: 1.0,
                        z: -4.0,
                    },
                    radius: 0.2,
                    color: Color {
                        red: 0.1,
                        green: 0.9,
                        blue: 0.1,
                    },
                    intensity: 1.0,
                }
            ),
            Object::from(
                Sphere {
                    center: Point {
                        x: 4.0,
                        y: 5.0,
                        z: -2.0,
                    },
                    radius: 0.2,
                    color: Color {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                    },
                    intensity: 10.0,
                }
            ),
            Object::from(
                Sphere {
                    center: Point {
                        x: 10.0,
                        y: 2.0,
                        z: -1.0,
                    },
                    radius: 0.2,
                    color: Color {
                        red: 1.0,
                        green: 1.0,
                        blue: 1.0,
                    },
                    intensity: 20.5,
                }
            ),
        ],
    );

    let image = render(&scene);

    let image_path: PathBuf = PathBuf::from("test.png");
    image.save(&image_path).unwrap();
}
