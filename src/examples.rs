/// # Examples
/// `examples` is a module that runs various examples of the raytracer's capabilities


use crate::tuple::Tuple;
use crate::canvas::Canvas;
use crate::color::Color;
use std::f64::consts::PI;
use crate::ray::Ray;
use crate::shape::sphere::Sphere;
use crate::shape::Shape;
use crate::intersection::{hit};
use crate::tuple::{point, vector};
use crate::material::Material;
use crate::light;
use crate::light::Light;
use crate::transformation::{scaling, translation, rotation_y, rotation_x, view_transform};
use crate::float::Float;
use crate::world::World;
use crate::camera::Camera;
use crate::{file, transformation};


pub fn draw_first_scene() {
    // Options
    let canvas_width = 100;
    let canvas_height = 100;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();

    let mut floor = Sphere::new();
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.color = Color::from_hex("F2E2BA");
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut left_wall = Sphere::new();
    left_wall.transform = translation(0.0, 0.0, 5.0) *
        rotation_y(-PI/4.0) * rotation_x(PI/2.0) *
        scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.color = Color::from_hex("D3F9FF");
    left_wall.material = material;
    world.objects.push(Box::new(left_wall));

    let mut right_wall = Sphere::new();
    right_wall.transform = translation(0.0, 0.0, 5.0) *
        rotation_y(PI/4.0) * rotation_x(PI/2.0) *
        scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.color = Color::from_hex("D3F9FF");
    right_wall.material = material;
    world.objects.push(Box::new(right_wall));

    let mut middle_sphere = Sphere::new();
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));

    let mut right_sphere = Sphere::new();
    right_sphere.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let mut material = Material::new();
    material.color = Color::from_hex("56D8CD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    right_sphere.material = material;
    world.objects.push(Box::new(right_sphere));

    let mut left_sphere = Sphere::new();
    left_sphere.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    let mut material = Material::new();
    material.color = Color::from_hex("6F2DBD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    left_sphere.material = material;
    world.objects.push(Box::new(left_sphere));

    let light = Light::point_light(&point(-10.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(0.0, 1.5, -5.0), point(0.0, 1.0, 0.0), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world);
    file::write_to_file(canvas.to_ppm(), String::from("first_scene.ppm"))
}

//--------------------------------------------------

pub fn draw_shaded_circle() {
    let canvas_pixels = 500;

    let mut material = Material::new();
    material.color = Color::from_hex("19647E");
    let shape = Sphere::new_with_material(material);

    let light = Light::point_light(&point(-10.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));

    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let ray_origin = point(0.0, 0.0, -5.0);
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
            let position = point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersects(&ray);
            let hit = hit(xs);
            if hit != None {
                let point = &ray.position(hit.as_ref().unwrap().t.value());
                let normal = hit.as_ref().unwrap().object.normal_at(point);
                let eye = -&ray.direction;

                let color = light::lighting(&hit.as_ref().unwrap().object.material(), &light, point, &eye, &normal, false);
                canvas.write_pixel(x, y, &color);
            }
        }
    }
    file::write_to_file(canvas.to_ppm(), String::from("shaded_circle.ppm"))
}

//--------------------------------------------------

pub fn draw_circle() {
    let color = Color::new(1.0, 0.6, 0.1);
    let shape = Sphere::new();
//    shape.set_transform(transformation::scaling(0.5, 1.0, 1.0));
    let canvas_pixels = 500;

    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let ray_origin = point(0.0, 0.0, -5.0);
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
            let position = point(world_x, world_y, wall_z);

            let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = shape.intersects(&ray);

            if hit(xs) != None {
                canvas.write_pixel(x, y, &color);
            }
        }
    }
    file::write_to_file(canvas.to_ppm(), String::from("circle.ppm"))
}

//--------------------------------------------------

pub fn draw_clock() {
    let canvas = &mut Canvas::new(100, 100);
    let color = &Color::new(1.0, 0.0, 0.0);
    let radius = 3.0/8.0 * canvas.width as f64;

    let center_x = canvas.width / 2;
    let center_z = canvas.height / 2;
    let rotate_30_degrees = transformation::rotation_y(PI/6.0);

    let mut mark = point(0.0, 0.0, 1.0 * radius); // Initial point at 12 o'clock

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

pub fn draw_arch() {
    let initial_projectile = Projectile {position: point(0.0, 1.0, 0.0), velocity: vector(1.0, 1.8, 0.0).normalize() * 11.25};
    let environment = Environment {gravity: vector(0.0, -0.1, 0.0), wind: vector(-0.01, 0.0, 0.0)};

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
