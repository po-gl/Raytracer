//! # Main
//! `main` drives the program

#[macro_use] extern crate impl_ops;
#[macro_use] extern crate lazy_static;
extern crate num_traits;

use std::env;

const FLOAT_THRESHOLD: f64 = 0.00001;

pub mod float;
pub mod tuple;
pub mod matrix;
pub mod transformation;
pub mod ray;
pub mod intersection;
pub mod color;
pub mod material;
pub mod pattern;
pub mod normal_perturber;
pub mod shape;
pub mod bounds;
pub mod light;
pub mod world;
pub mod camera;
pub mod canvas;
pub mod examples;
pub mod file;


fn main() {
    let args: Vec<String> = env::args().collect();
    let example: &String;
    if args.len() > 1 {
        example = &args[1];
    } else {
        example = &args[0]; // set to invalid example
    }

    match example.as_str() {
        "draw-arch" => {
            println!("Running Example \"{}\"", example);
            examples::draw_arch();
        },
        "draw-clock" => {
            println!("Running Example \"{}\"", example);
            examples::draw_clock();
        },
        "draw-circle" => {
            println!("Running Example \"{}\"", example);
            examples::draw_circle();
        },
        "draw-rand-circle" => {
            println!("Running Example \"{}\"", example);
            examples::draw_uniform_rand_circle();
        },
        "draw-shaded-circle" => {
            println!("Running Example \"{}\"", example);
            examples::draw_shaded_circle();
        },
        "draw-first-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_first_scene();
        },
        "draw-scene-on-a-plane" => {
            println!("Running Example \"{}\"", example);
            examples::draw_scene_on_a_plane();
        },
        "draw-patterned-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_patterned_scene();
        },
        "draw-blended-patterned-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_blended_patterned_scene();
        },
        "draw-perturbed-patterned-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_perturbed_patterned_scene();
        },
        "draw-reflected-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_reflected_scene();
        },
        "draw-refracted-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_refracted_scene();
        },
        "draw-cylinder-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_cylinder_scene();
        },
        "draw-cylinder-refracted-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_cylinder_refracted_scene();
        },
        "draw-cone-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_cone_scene();
        },
        "draw-hexagon-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_hexagon_scene();
        },
        "draw-obj-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_obj_scene();
        },
        "draw-csg-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_csg_scene();
        },
        "draw-soft-shadows-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_soft_shadow_scene();
        },
        "draw-perturbed-normal-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_perturbed_normal_scene();
        },
        "draw-fractal-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_fractal_scene();
        },
        "draw-combine-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_combined_scene();
        },
        "draw-bounds-scene" => {
            println!("Running Example \"{}\"", example);
            examples::draw_bounds_scene();
        },
        _ => println!("No valid argument.")
    }
}
