/// # Camera
/// `camera` is a module to represent a sphere shape

use crate::float::Float;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::point;
use crate::world::World;
use crate::canvas::Canvas;
use indicatif::ProgressStyle;

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

        let half_width;
        let half_height;

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

    pub fn render(&self, world: World) -> Canvas {
        let mut image = Canvas::new(self.h_size, self.v_size);

        let pb = indicatif::ProgressBar::new(self.v_size as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:50} {pos:>7}/{len:7} {msg}"));

        for y in 0..self.v_size {
            for x in 0..self.h_size {
                let ray = self.ray_for_pixel(x, y);
                let color = world.color_at(&ray);
                image.write_pixel(y, x, &color);
            }
            pb.inc(1);
        }
        pb.finish_with_message("Finished Rendering!");
        image
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    use crate::transformation::{rotation_y, translation, view_transform};
    use crate::color::Color;
    use crate::tuple::vector;

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

    #[test]
    fn camera_render() {
        let w = World::default_world();
        let mut c = Camera::new(11, 11, PI/2.0);
        let from = point(0.0, 0.0, -5.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);
        c.transform = view_transform(from, to, up);
        let image = c.render(w);
        assert_eq!(image.pixel_at(5, 5), &Color::new(0.38066, 0.47583, 0.2855));
    }
}