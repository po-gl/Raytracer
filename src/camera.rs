/// # Camera
/// `camera` is a module to represent a sphere shape

use crate::float::Float;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::{point, vector};

#[derive(Debug)]
pub struct Camera {
    pub h_size: i32,
    pub v_size: i32,
    pub pixel_size: Float,
    pub field_of_view: Float,
    pub transform: Matrix4,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(h_size: i32, v_size: i32, field_of_view: f64) -> Camera {
        // Calculate the size of a pixel
        let half_view = (field_of_view/2.0).tan();
        let aspect_ratio = h_size as f64 / v_size as f64;

        let mut half_width = 0.0;
        let mut half_height = 0.0;

        if Float(aspect_ratio) >= Float(1.0) {
            half_width = half_view;
            half_height = half_view / aspect_ratio;
        } else {
            half_width = half_view * aspect_ratio;
            half_height = half_view;
        }
        let pixel_size = Float((half_width * 2.0) / h_size as f64);

        Camera {
            h_size,
            v_size,
            pixel_size,
            field_of_view: Float(field_of_view),
            transform: Matrix4::identity(),
            half_width,
            half_height,
        }
    }

    /// Returns a ray starting at the camera and passes through the (x, y) pixel
    pub fn ray_for_pixel(&self, x: i32, y: i32) -> Ray {
        // Offset from the edge of the canvas to the pixel's center
        let x_offset = (x as f64 + 0.5) * self.pixel_size.value();
        let y_offset = (y as f64 + 0.5) * self.pixel_size.value();

        // Untransformed coordinates of the pixel in world space
        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        // Transform the canvas point and origin
        // then compute the ray's direction vector
        let pixel = self.transform.inverse() * point(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::transformation::{rotation_y, translation};

    #[test]
    fn camera_creation() {
        let c = Camera::new(160, 120, PI/2.0);
        assert_eq!(c.h_size, 160);
        assert_eq!(c.v_size, 120);
        assert_eq!(c.field_of_view, PI/2.0);
        assert_eq!(c.transform, Matrix4::identity());
    }

    #[test]
    fn camera_pixel_size() {
        let c = Camera::new(200, 125, PI/2.0);
        assert_eq!(c.pixel_size, 0.01);

        let c = Camera::new(125, 200, PI/2.0);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn camera_rays() {
        // Ray through center of canvas
        let c = Camera::new(201, 101, PI/2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.0, 0.0, -1.0));

        // Ray through corner of canvas
        let c = Camera::new(201, 101, PI/2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(0.66519, 0.33259, -0.66851));

        // Ray at a transformed camera
        let mut c = Camera::new(201, 101, PI/2.0);
        c.transform = rotation_y(PI/4.0) * translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq!(r.direction, vector(2.0f64.sqrt()/2.0, 0.0, -2.0f64.sqrt()/2.0));
    }
}