//! # Main
//! `main` drives the program

const FLOAT_THRESHOLD: f64 = 0.00001;
const TEST_ARG: &str = "--draw-circle";

#[macro_use] extern crate impl_ops;
#[macro_use] extern crate lazy_static;

pub mod float;
pub mod tuple;
pub mod matrix;
pub mod transformation;
pub mod ray;
pub mod intersection;
pub mod color;
pub mod shape;
pub mod canvas;
pub mod file;

use tuple::Tuple;
use canvas::Canvas;
use crate::color::Color;
use std::f64::consts::PI;
use crate::ray::Ray;
use crate::shape::sphere::Sphere;
use crate::shape::Shape;
use crate::intersection::hit;


fn main() {
    match TEST_ARG {
        "--draw-arch" => draw_arch(),
        "--draw-clock" => draw_clock(),
        "--draw-circle" => draw_circle(),
        _ => println!("No valid argument.")
    }
}

// Below is miscellaneous functions for testing and drawing

fn draw_circle() {
    let color = Color::new(1.0, 0.6, 0.1);
    let shape = Sphere::new();
//    shape.set_transform(transformation::scaling(0.5, 1.0, 1.0));
    let canvas_pixels = 500;

    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let ray_origin = tuple::point(0.0, 0.0, -5.0);
    let canvas = &mut Canvas::new(canvas_pixels, canvas_pixels);

    // Each row of pixels
    for y in 0..canvas_pixels {
        // World y coordinate top = +half and bottom = -half
        let world_y = half - pixel_size * y as f64;

        // Each col of pixels
        for x in 0..canvas_pixels {
            // World x coordinate left = -half and right = +half
            let world_x = -half + pixel_size * x as f64;

            // the point on the wall that the ray will target
            let position = tuple::point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersects(ray);

            if hit(xs) != None {
                canvas.write_pixel(x, y, &color);
            }
        }
    }
    file::write_to_file(canvas.to_ppm(), String::from("circle.ppm"))
}

//--------------------------------------------------

fn draw_clock() {
    let canvas = &mut Canvas::new(100, 100);
    let color = &Color::new(1.0, 0.0, 0.0);
    let radius = 3.0/8.0 * canvas.width as f64;

    let center_x = canvas.width / 2;
    let center_z = canvas.height / 2;
    let rotate_30_degrees = transformation::rotation_y(PI/6.0);

    let mut mark = tuple::point(0.0, 0.0, 1.0 * radius); // Initial point at 12 o'clock

    for _ in 0..12 {
//        println!("mark: {:?}", mark);
        // Rotate
        mark = &rotate_30_degrees * &mark;

        // Draw pixel
        let x = mark.x.value() as i32 + center_x;
        let z = mark.z.value() as i32 + center_z;
        canvas.write_pixel(x, z, color);
    }

    file::write_to_file(canvas.to_ppm(), String::from("clock.ppm"));
}

//--------------------------------------------------

fn draw_arch() {
    let initial_projectile = Projectile {position: tuple::point(0.0, 1.0, 0.0), velocity: tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25};
    let environment = Environment {gravity: tuple::vector(0.0, -0.1, 0.0), wind: tuple::vector(-0.01, 0.0, 0.0)};

    let canvas = &mut Canvas::new(900, 550);
    tick_loop(initial_projectile, environment, canvas);
    file::write_to_file(canvas.to_ppm(), String::from("projectile.ppm"));
}

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick_loop(mut projectile: Projectile, environment: Environment, canvas: &mut Canvas) {
    while projectile.position.y.value() > 0.0 {
        let x = projectile.position.x.value().round() as i32;
        let y = projectile.position.y.value().round() as i32;
        canvas.write_pixel(canvas.height - y, x, &Color::new(1.0, 1.0, 0.0));
        projectile = tick(projectile, &environment);
    }
}

fn tick(projectile: Projectile, environment: &Environment) -> Projectile {
    let position = &projectile.position + &projectile.velocity;
    let velocity = &projectile.velocity + &environment.gravity + &environment.wind;
    Projectile {position, velocity}
}
