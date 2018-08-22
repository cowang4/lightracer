
extern crate image;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rayon;

mod color;
mod scene;
mod point;
mod vector;
mod rendering;

use std::path::PathBuf;

use image::{DynamicImage, GenericImage};
//use rayon::prelude::*;

use color::Color;
use scene::*;
use point::Point;
use rendering::{Ray, cast_camera_ray};
use vector::Vector3;

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
    // (0u32..scene.width)
    //     .into_iter()
    //     .collect::<Vec<u32>>()
    //     .par_iter()
    //     .map(|x| {
    //         (0..scene.height)
    //             .into_iter()
    //             .collect::<Vec<u32>>()
    //             .par_iter()
    //             .map(|y| {
    //                 let camera_ray = Ray::create_camera(*x, *y, scene);
    //                 let color = cast_camera_ray(scene, &camera_ray);
    //                 image.put_pixel(*x, *y, color);
    //             });
    //     });
    image
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        objects: vec![
            Box::new(
                Sphere {
                    center: Point {
                        x: 0.0,
                        y: 0.0,
                        z: -5.0,
                    },
                    radius: 1.0,
                    color: Color(Rgba{data: [40, 55, 40, 255],}),
                    is_light: false,
                }
            ),
        ],
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());
}

fn main() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        objects: vec![
            Box::new(
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
            Box::new(
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
            Box::new(
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
                    intensity: 0.0,
                }
            ),
            Box::new(
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
            Box::new(
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
    };

    let image = render(&scene);

    let image_path: PathBuf = PathBuf::from("test.png");
    image.save(&image_path).unwrap();
}
