
extern crate image;
extern crate serde;
#[macro_use]
extern crate serde_derive;

mod scene;
mod point;
mod vector;
mod rendering;

use image::{DynamicImage, ImageFormat, GenericImage, Rgba};
use std::fs::{OpenOptions};
use std::path::PathBuf;

use scene::*;
use point::Point;
use rendering::{Ray, cast_ray};

// Inspired/Copied from https://bheisler.github.io/post/writing-raytracer-in-rust-part-1/

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let camera_ray = Ray::create_camera(x, y, scene);
            image.put_pixel(x, y, cast_ray(scene, &camera_ray));
        }
    }
    image
}

#[test]
fn test_can_render_scene() {   
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Rgba {
                red: 40,
                green: 255,
                blue: 40,
                alpha: 255,
            },
        },
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
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Rgba {
                red: 40,
                green: 255,
                blue: 40,
                alpha: 255,
            },
        },
    };
    let image = render(&scene);

    let image_path: PathBuf = PathBuf::new("test.png");

    let mut image_file =
        OpenOptions::new().write(true).truncate(true).create(true).open(image_path).unwrap();
    image.save(&mut image_file, ImageFormat::PNG).unwrap();
}
