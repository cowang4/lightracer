
use image::{Rgba};

use point::Point;
use vector::Vector3;
use scene::{Scene, Sphere};

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
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        //Create a line segment between the ray origin and the center of the sphere
        let l: Vector3 = self.center - ray.origin;
        //Use l as a hypotenuse and find the length of the adjacent side
        let adj2 = l.dot(&ray.direction);
        //Find the length-squared of the opposite side
        //This is equivalent to (but faster than) (l.length() * l.length()) - (adj2 * adj2)
        let d2 = l.dot(&l) - (adj2 * adj2);
        //If that length-squared is less than radius squared, the ray intersects the sphere
        d2 < (self.radius * self.radius)
    }
}

pub fn cast_ray(scene: &Scene, ray: &Ray) -> Rgba<u8> {
    if scene.sphere.instersect(ray) {
        scene.sphere.color
    } else {
        Rgba::new(0,0,0,1);
    }
}


