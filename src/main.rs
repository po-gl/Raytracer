//! # Main
//! `main` drives the program

const FLOAT_THRESHOLD: f64 = 0.00001;
const TEST_ARG: &str = "--draw-first-scene";

#[macro_use] extern crate impl_ops;
#[macro_use] extern crate lazy_static;

pub mod float;
pub mod tuple;
pub mod matrix;
pub mod transformation;
pub mod ray;
pub mod intersection;
pub mod color;
pub mod material;
pub mod shape;
pub mod light;
pub mod world;
pub mod camera;
pub mod canvas;
pub mod examples;
pub mod file;


fn main() {
    match TEST_ARG {
        "--draw-arch" => examples::draw_arch(),
        "--draw-clock" => examples::draw_clock(),
        "--draw-circle" => examples::draw_circle(),
        "--draw-shaded-circle" => examples::draw_shaded_circle(),
        "--draw-first-scene" => examples::draw_first_scene(),
        _ => println!("No valid argument.")
    }
}
