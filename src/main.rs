//! # Main
//! `main` drives the program


const FLOAT_THRESHOLD: f64 = 0.00001;

// Macro used for easier operation overloading (versions for references)
#[macro_use] extern crate impl_ops;

pub mod float;
pub mod tuple;
pub mod matrix;
pub mod color;
pub mod canvas;
pub mod file;

use tuple::Tuple;
use canvas::Canvas;
use crate::color::Color;

fn main() {
//    let map = vec![vec![0u8; 3]; 2];
//    println!("Map: {:?}", map);
//
//    let mut c = Canvas::new(5, 3);
//    c.write_pixel(0, 0, &Color::new(1.5, 0.0, 0.0));
//    c.write_pixel(1, 2, &Color::new(0.0, 0.5, 0.0));
//    c.write_pixel(2, 4, &Color::new(-0.5, 0.0, 1.0));
//    let s = c.to_ppm();
//    println!("String s: {}", s);
//    file::write_to_file(s, String::from("new.ppm"));

    let initial_projectile = Projectile {position: tuple::point(0.0, 1.0, 0.0), velocity: tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25};
    let environment = Environment {gravity: tuple::vector(0.0, -0.1, 0.0), wind: tuple::vector(-0.01, 0.0, 0.0)};

    let canvas = &mut Canvas::new(900, 550);
    tick_loop(initial_projectile, environment, canvas);
    file::write_to_file(canvas.to_ppm(), String::from("projectile.ppm"));
}

// For Testing Tuples
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
