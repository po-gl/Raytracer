//! # Main
//! `main` drives the program


const FLOAT_THRESHOLD: f64 = 0.00001;

// Macro used for easier operation overloading (versions for references)
#[macro_use] extern crate impl_ops;

pub mod float;
pub mod tuple;
pub mod color;

use tuple::Tuple;

fn main() {

    let initial_projectile = Projectile {position: tuple::point(0.0, 1.0, 0.0), velocity: tuple::vector(1.0, 1.0, 0.0)};
    let environment = Environment {gravity: tuple::vector(0.0, -0.1, 0.0), wind: tuple::vector(-0.01, 0.0, 0.0)};

    tick_loop(initial_projectile, environment);
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

fn tick_loop(mut projectile: Projectile, environment: Environment) {
    while projectile.position.y.value() > 0.0 {
//        println!("Projectile pos: {:?}  vel: {:?}", projectile.position, projectile.velocity);
        println!("Projectile pos: {:?}", projectile.position);
        projectile = tick(projectile, &environment);
    }
}

fn tick(projectile: Projectile, environment: &Environment) -> Projectile {
    let position = &projectile.position + &projectile.velocity;
    let velocity = &projectile.velocity + &environment.gravity + &environment.wind;
    Projectile {position, velocity}
}
