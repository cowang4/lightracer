
use crate::point::Point;
use crate::scene::Scene;
use crate::vector::Vector3;


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
        let aspect_ratio = (scene.width as f32) / (scene.height as f32);

        let x_pixel_center = x as f32 + 0.5; //add 0.5 (one half-pixel) because we want our ray to pass through the center (rather than the corner) of the pixel on our imaginary sensor
        let x_normalized_to_width = x_pixel_center / scene.width as f32; //Then we divide by the image width to convert from our original coordinates (0…800) to (0.0…1.0)
        let x_adjusted_screen_pos = (x_normalized_to_width * 2.0) - 1.0; //the (-1.0…1.0) coordinates we want, so we multiply by two and subtract one
        let sensor_x = (x_adjusted_screen_pos * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f32 + 0.5) / scene.height as f32) * 2.0) * fov_adjustment; //  the image coordinates have positive y meaning down, where we want positive y to be up, so we negate

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
