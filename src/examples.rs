/// # Examples
/// `examples` is a module that runs various examples of the raytracer's capabilities


use crate::tuple::Tuple;
use crate::canvas::Canvas;
use crate::color::Color;
use std::f64::consts::PI;
use crate::ray::Ray;
use crate::shape::sphere::Sphere;
use crate::shape;
use crate::shape::Shape;
use crate::intersection::{hit};
use crate::tuple::{point, vector};
use crate::material::{Material, CmpPerlin};
use crate::light::Light;
use crate::transformation::{scaling, translation, rotation_y, rotation_x, view_transform};
use crate::float::Float;
use crate::world::World;
use crate::camera::Camera;
use crate::{file, transformation};
use crate::shape::plane::Plane;
use crate::pattern::stripe_pattern::StripePattern;
use crate::pattern::ring_pattern::RingPattern;
use crate::pattern::Pattern;
use crate::pattern::gradient_pattern::GradientPattern;
use crate::pattern::blended_pattern::BlendedPattern;
use crate::pattern::perturbed_pattern::PerturbedPattern;
use crate::shape::cube::Cube;
use crate::pattern::checker_pattern::CheckerPattern;
use crate::shape::cylinder::Cylinder;
use crate::shape::cone::Cone;
use crate::shape::group::Group;
use crate::shape::triangle::Triangle;
use crate::file::obj_loader::Parser;
use crate::shape::shape_list::ShapeList;
use crate::shape::csg::CSG;
use rand::Rng;
use crate::matrix::Matrix4;
use noise::Perlin;

//--------------------------------------------------
//--------------------------------------------------


pub fn draw_combined_scene() {
    // Options
    let canvas_width = 1000;
    let canvas_height = 1000;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let shape_list = &mut ShapeList::new();

    let mut floor = Plane::new(shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    let pattern_a = RingPattern::new(Color::from_hex("726DA8"), Color::from_hex("A0D2DB"));
//    let pattern_b = StripePattern::new(Color::from_hex("0000FF"), Color::black());
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("FFE2BA");
    material.specular = Float(0.0);
    material.reflective = Float(0.4);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut glass_sphere = Sphere::new(shape_list);
    glass_sphere.transform = translation(-0.5, 0.45, -2.0) * scaling(0.45, 0.45, 0.45);
//    let mut material = Material::new();
    let mut material = Material::glass();
    material.normal_perturb = Some(String::from("sin_y"));
    material.normal_perturb_factor = Some(20.0);
    glass_sphere.material = material;
    world.objects.push(Box::new(glass_sphere));

    let mut middle_sphere = Sphere::new(shape_list);
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    material.normal_perturb = Some(String::from("perlin"));
    material.normal_perturb_factor = Some(0.2);
    material.normal_perturb_perlin = Some(CmpPerlin {perlin: Perlin::new()});
    let pattern_a = RingPattern::new(Color::from_hex("F24236"), Color::from_hex("564138"));
//    let pattern_a = RingPattern::new(Color::from_hex("679289"), Color::black());
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1) * transformation::rotation_y(PI/6.0) * transformation::rotation_x(-PI/6.0));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));


    // Fractal
    let material = Material::glass();
//    let mut material = Material::new();
//    material.color = Color::from_hex("FF0000");
//    material.color = Color::from_hex("FF0000");
//    material.transparency = Float(0.8);
    let mut fractal = fractal(material, 3, shape_list);
//    fractal.set_transform(translation(0.0, 3.0, 0.0) * scaling(1.5, 1.5, 1.5), shape_list);
    fractal.set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5) * rotation_y(PI/3.0) * rotation_x(-PI/12.0), shape_list);
    world.objects.push(fractal);

    let mut left_sphere = Sphere::new(shape_list);
    left_sphere.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    let mut material = Material::mirror();
    material.color = Color::from_hex("6F2DBD");
//    material.diffuse = Float(0.7);
//    material.specular = Float(0.3);
//    material.reflective = Float(0.7);
    left_sphere.material = material;
    world.objects.push(Box::new(left_sphere));


    // Background shapes

    // Cone cluster
//    let material = Material::mirror();
    let material = Material::mirror();
//    let mut material = Material::new();
//    material.color = Color::from_hex("FF0000");

    let mut shape = Cone::new_bounded(-1.0, 0.0);
    shape.closed = true;
    shape.transform = translation(0.5, 0.5, -0.1) * scaling(0.1, 0.5, 0.1);
    shape.material = material.clone();
    world.objects.push(Box::new(shape));

    let mut shape = Cylinder::new_bounded(-1.0, 0.0);
    shape.closed = true;
    shape.transform = translation(0.3, 0.4, 0.08) * scaling(0.1, 0.4, 0.1);
    shape.material = material.clone();
    world.objects.push(Box::new(shape));

    let mut shape = Cube::new(shape_list);
    shape.transform = translation(0.1, 0.2, -0.27) * scaling(0.04, 0.2, 0.04);
    shape.material = material.clone();
    world.objects.push(Box::new(shape));


//    let light = Light::point_light(&point(-10.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
    let light = Light::area_light(&point(-10.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0), 1.0);
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(0.0, 1.5, -5.0), point(0.0, 1.0, 0.0), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world, shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("combined_scene.ppm"))
}

//--------------------------------------------------

pub fn fractal(material: Material, recursion_depth: i32, shape_list: &mut ShapeList) -> Box<dyn Shape> {
    let mut group = Group::new(shape_list);

    // main sphere
    let mut new_sphere: Box<dyn Shape> = Box::new(Sphere::new_with_material(material.clone(), shape_list));
    group.add_child(&mut new_sphere, shape_list);

    fractal_node(&mut group, Matrix4::identity(), &material, recursion_depth, shape_list);


    Box::new(group)
}

pub fn fractal_node(node_group: &mut Group, transform: Matrix4, material: &Material,
                    remaining: i32, shape_list: &mut ShapeList) {
    if remaining <= 0 {
        return
    }
    let scale = 0.4;

    let current_width = 1.0 * scale;

    // Create 6 spheres
    // Top
    let mut new_sphere: Box<dyn Shape> = Box::new(Sphere::new_with_material(material.clone(), shape_list));
    let new_transform = transform * translation(0.0, 1.0 + current_width, 0.0) * scaling(current_width, current_width, current_width);
    new_sphere.set_transform(new_transform, shape_list);
    node_group.add_child(&mut new_sphere, shape_list);

    fractal_node(node_group, new_transform, material, remaining-1, shape_list);

    // Bottom
    let mut new_sphere: Box<dyn Shape> = Box::new(Sphere::new_with_material(material.clone(), shape_list));
    let new_transform = transform * translation(0.0, -(1.0 + current_width), 0.0) * scaling(current_width, current_width, current_width);
    new_sphere.set_transform(new_transform, shape_list);
    node_group.add_child(&mut new_sphere, shape_list);

    fractal_node(node_group, new_transform, material, remaining-1, shape_list);

    // Left
    let mut new_sphere: Box<dyn Shape> = Box::new(Sphere::new_with_material(material.clone(), shape_list));
    let new_transform = transform * translation(-(1.0 + current_width), 0.0, 0.0) * scaling(current_width, current_width, current_width);
    new_sphere.set_transform(new_transform, shape_list);
    node_group.add_child(&mut new_sphere, shape_list);

    fractal_node(node_group, new_transform, material, remaining-1, shape_list);

    // Right
    let mut new_sphere: Box<dyn Shape> = Box::new(Sphere::new_with_material(material.clone(), shape_list));
    let new_transform = transform * translation(1.0 + current_width, 0.0, 0.0) * scaling(current_width, current_width, current_width);
    new_sphere.set_transform(new_transform, shape_list);
    node_group.add_child(&mut new_sphere, shape_list);

    fractal_node(node_group, new_transform, material, remaining-1, shape_list);

    // Forwards
    let mut new_sphere: Box<dyn Shape> = Box::new(Sphere::new_with_material(material.clone(), shape_list));
    let new_transform = transform * translation(0.0, 0.0, -(1.0 + current_width)) * scaling(current_width, current_width, current_width);
    new_sphere.set_transform(new_transform, shape_list);
    node_group.add_child(&mut new_sphere, shape_list);

    fractal_node(node_group, new_transform, material, remaining-1, shape_list);

    // Backwards
    let mut new_sphere: Box<dyn Shape> = Box::new(Sphere::new_with_material(material.clone(), shape_list));
    let new_transform = transform * translation(0.0, 0.0, 1.0 + current_width) * scaling(current_width, current_width, current_width);
    new_sphere.set_transform(new_transform, shape_list);
    node_group.add_child(&mut new_sphere, shape_list);

    fractal_node(node_group, new_transform, material, remaining-1, shape_list);
}



pub fn draw_fractal_scene() {
    // Options
    let canvas_width = 200;
    let canvas_height = 200;
    let fov = PI/2.0;

    // Construct world
    let mut world = World::new();
    let shape_list = &mut ShapeList::new();

    let mut floor = Plane::new(shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
//    material.reflective = Float(0.4);
    material.ambient = Float(0.15);
    material.specular = Float(0.0);
    let pattern_a = RingPattern::new(Color::from_hex("726DA8"), Color::from_hex("A0D2DB"));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(rotation_y(PI/3.0) * scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    floor.material = material;
    shape_list.update(Box::new(floor.clone()));
    world.objects.push(Box::new(floor));

    let scale = 0.4;
    let current = 1.0 * scale;
    let trans = translation(0.0, 1.0 + current, 0.0) * scaling(current, current, current);

    let mut s1 = Sphere::new(shape_list);
    let mut material = Material::new();
    material.color = Color::from_hex("0000FF");
    s1.set_transform( trans * translation(-(1.0 + current), 0.0, 0.0) * scaling(current, current, current), shape_list);
    s1.set_material(material, shape_list);
//    world.objects.push(Box::new(s1));

    let material = Material::glass();
//    material.color = Color::from_hex("FF0000");
//    material.transparency = Float(0.8);

    let mut fractal = fractal(material, 3, shape_list);
    fractal.set_transform(translation(0.0, 3.0, 0.0) * scaling(1.5, 1.5, 1.5), shape_list);

    world.objects.push(fractal);


//    let light = Light::area_light(&point(-2.5, 4.6, -2.5), &Color::new(1.0, 1.0, 1.0), 0.5);
    let light = Light::point_light(&point(-2.5, 4.6, -2.5), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(1.7, 5.0, -4.5), point(0.4, 3.0, -0.7), vector(0.0, 2.0, 0.0));
//    camera.transform = view_transform(point(0.0, 2.0, -2.0), point(0.0, 1.0, 0.0), vector(0.0, 2.0, 0.0));

    let canvas = camera.render(world, shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("fractal.ppm"))
}


//--------------------------------------------------

pub fn draw_perturbed_normal_scene() {
    // Options
    let canvas_width = 500;
    let canvas_height = 500;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let shape_list = &mut ShapeList::new();

    let mut floor = Plane::new(shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    let pattern_a = RingPattern::new(Color::from_hex("726DA8"), Color::from_hex("A0D2DB"));
//    let pattern_b = StripePattern::new(Color::from_hex("0000FF"), Color::black());
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("FFE2BA");
    material.specular = Float(0.0);
    material.reflective = Float(0.4);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut glass_sphere = Sphere::new(shape_list);
    glass_sphere.transform = translation(-0.5, 0.45, -2.0) * scaling(0.45, 0.45, 0.45);
    let mut material = Material::glass();
    material.normal_perturb = Some(String::from("sin_y"));
    material.normal_perturb_factor = Some(20.0);
    glass_sphere.material = material;
    world.objects.push(Box::new(glass_sphere));

    let mut middle_sphere = Sphere::new(shape_list);
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    material.normal_perturb = Some(String::from("perlin"));
    material.normal_perturb_factor = Some(0.2);
    material.normal_perturb_perlin = Some(CmpPerlin {perlin: Perlin::new()});
    let pattern_a = RingPattern::new(Color::from_hex("F24236"), Color::from_hex("564138"));
//    let pattern_a = RingPattern::new(Color::from_hex("679289"), Color::black());
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1) * transformation::rotation_y(PI/6.0) * transformation::rotation_x(-PI/6.0));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));

    let mut right_sphere = Sphere::new(shape_list);
    right_sphere.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let mut material = Material::mirror();
    material.reflective = Float(0.4);
    let mut pattern = StripePattern::new(Color::white(), Color::black());
    pattern.set_transform(transformation::rotation_z(-PI/12.0) * transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("56D8CD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    right_sphere.material = material;
    world.objects.push(Box::new(right_sphere));

    let mut left_sphere = Sphere::new(shape_list);
    left_sphere.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    let mut material = Material::mirror();
    material.color = Color::from_hex("6F2DBD");
//    material.diffuse = Float(0.7);
//    material.specular = Float(0.3);
//    material.reflective = Float(0.7);
    left_sphere.material = material;
    world.objects.push(Box::new(left_sphere));

    let light = Light::point_light(&point(-10.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(0.0, 1.5, -5.0), point(0.0, 1.0, 0.0), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world, shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("perturbed_normal_scene2.ppm"))
}

//--------------------------------------------------


pub fn draw_soft_shadow_scene() {
    // Options
    let canvas_width = 100;
    let canvas_height = 100;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let shape_list = &mut ShapeList::new();

    let mut floor = Plane::new(shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    material.ambient = Float(0.15);
    material.specular = Float(0.0);
    floor.material = material;
    shape_list.update(Box::new(floor.clone()));
    world.objects.push(Box::new(floor));


    let mut s1 = Sphere::new(shape_list);
    let mut material = Material::new();
    material.color = Color::from_hex("0000FF");
//    material.normal_perturb = Some(String::from("sin_y"));
//    material.normal_perturb_factor = Some(20.0);
    s1.set_transform(translation(0.0, 1.0, 0.0), shape_list);
    s1.set_material(material, shape_list);
    world.objects.push(Box::new(s1));

    let mut s2 = Sphere::new(shape_list);
    s2.set_transform(translation(0.5, 0.9, -1.4) * scaling(0.2, 0.2, 0.2), shape_list);
    let mut material = Material::new();
    material.color = Color::from_hex("FF0000");
    s2.set_material(material, shape_list);
    world.objects.push(Box::new(s2));

    let mut c1 = Cube::new(shape_list);
    c1.set_transform(translation(0.5, 0.3, -1.4) * scaling(0.02, 0.5, 0.02), shape_list);
    let mut material = Material::new();
    material.color = Color::from_hex("445544");
    c1.set_material(material, shape_list);
    world.objects.push(Box::new(c1));


//    let mut light = Light::area_light(&point(-2.5, 4.6, -2.5), &Color::new(1.0, 1.0, 1.0), 0.2);
//    light.radius = Some(20.0);
    let light = Light::point_light(&point(-2.5, 4.6, -2.5), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(0.4, 2.0, -3.0), point(0.4, 1.0, -0.7), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world, shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("soft_shadows_scene.ppm"))
}


//--------------------------------------------------

pub fn draw_csg_scene() {
    // Options
    let canvas_width = 500;
    let canvas_height = 500;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let shape_list = &mut ShapeList::new();

    let mut floor = Plane::new(shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let pattern_a = RingPattern::new(Color::from_hex("726DA8"), Color::from_hex("A0D2DB"));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(rotation_y(PI/3.0) * scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.ambient = Float(0.15);
    material.specular = Float(0.0);
    floor.material = material;
    shape_list.update(Box::new(floor.clone()));
    world.objects.push(Box::new(floor));


    let mut s1 = Cube::new(shape_list);
    let mut material = Material::glass();
    material.color = Color::from_hex("FFFFFF");
    s1.set_material(material, shape_list);

    let mut s2 = Sphere::new(shape_list);
    s2.set_transform(translation(0.3, 0.5, -0.5) * scaling(1.0, 1.0, 1.0), shape_list);
    let mut material = Material::new();
    material.color = Color::from_hex("FFFF00");
    s2.set_material(material, shape_list);

    let mut csg = CSG::new_with_operation("difference", s1.id(), s2.id(), shape_list);
    csg.set_transform(translation(0.0, 1.0, 0.0) * scaling(1.0, 1.0, 1.0), shape_list);

    world.objects.push(Box::new(csg));


    let p1 = point(0.0, 1.0, 0.0);
    let p2 = point(-1.0, 0.0, 0.0);
    let p3 = point(1.0, 0.0, 0.0);
    let mut tri = Triangle::new(p1, p2, p3, shape_list);
    tri.transform = translation(0.0, 0.0, 22.0) * scaling(6.0, 6.0, 6.0);
    let mut material = Material::new();
    material.color = Color::from_hex("FF0000");
    tri.material = material;
    shape_list.update(Box::new(tri.clone()));
    world.objects.push(Box::new(tri));

    let light = Light::point_light(&point(-10.0, 16.0, -10.0), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(-1.0, 2.5, -5.0), point(0.0, 1.0, 0.0), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world, shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("csg_scene.ppm"))
}


//--------------------------------------------------

pub fn draw_obj_scene() {
    // Options
    let canvas_width = 100;
    let canvas_height = 100;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let pattern_a = RingPattern::new(Color::from_hex("726DA8"), Color::from_hex("A0D2DB"));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(rotation_y(PI/3.0) * scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.ambient = Float(0.15);
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let p1 = point(0.0, 1.0, 0.0);
    let p2 = point(-1.0, 0.0, 0.0);
    let p3 = point(1.0, 0.0, 0.0);
    let mut tri = Triangle::new(p1, p2, p3, &mut shape_list);
    tri.transform = translation(0.0, 0.0, 36.0) * scaling(6.0, 6.0, 6.0);
    let mut material = Material::new();
    material.color = Color::from_hex("FF0000");
    tri.material = material;
    world.objects.push(Box::new(tri));

    let parser = Parser::parse_obj_file("Obj/cat.obj", &mut shape_list);
    let mut tri_group = parser.unwrap().default_group;
    tri_group.transform = translation(0.0, 1.0, -2.0) * scaling(1.0, 1.0, 1.0) * rotation_y(PI/6.0) * rotation_x(PI/6.0);
    let mut material = Material::glass();
    material.color = Color::from_hex("FF8800");
    tri_group.material = material;

    println!("Total shapes list {}\n{:#?}", &shape_list.len(), &shape_list);

    world.objects.push(Box::new(tri_group));

    let light = Light::point_light(&point(-10.0, 16.0, -10.0), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(0.0, 1.5, -5.0), point(0.0, 1.0, 0.0), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("obj_scene.ppm"))
}

//--------------------------------------------------

//pub fn hexagon(shape_list: &mut ShapeList) -> Box<dyn Shape> {
//    let mut hex = Group::new(shape_list);
//
//    for i in 0..5 {
//        let mut side = hexagon_side(Box::new(hex.clone()), shape_list);
//        println!("Children: {:#?}", side);
//        println!();
//        side.set_transform(rotation_y(i as f64 * PI/3.0), shape_list);
//        hex.add_child(&mut side)
//    }
////    println!("Children: {:#?}", hex);
//    Box::new(hex)
//}
//
//pub fn hexagon_side(parent: Box<dyn Shape>, shape_list: &mut ShapeList) -> Box<dyn Shape> {
//    let mut side = Group::new(shape_list);
//    side.add_child(&mut hexagon_corner(Box::new(side.clone()), shape_list));
//    println!("  side1: {:#?}", side);
//    side.add_child(&mut hexagon_edge(Box::new(side.clone()), shape_list));
//    println!("  side2: {:#?}", side);
//
//    Box::new(side).set_parent(parent)
//}
//
//pub fn hexagon_edge(parent: Box<dyn Shape>, shape_list: &mut ShapeList) -> Box<dyn Shape> {
//    let mut edge = Cylinder::new(shape_list);
//    edge.minimum = 0.0;
//    edge.maximum = 0.0;
//    edge.transform = translation(0.0, 0.0, -1.0) *
//        rotation_y(-PI/6.0) *
//        rotation_z(-PI/2.0) *
//        scaling(0.25, 1.0, 0.25);
//    shape_list.update(Box::new(edge.clone()));
//
//    Box::new(edge).set_parent(parent)
//}
//
//pub fn hexagon_corner(parent: Box<dyn Shape>, shape_list: &mut ShapeList) -> Box<dyn Shape> {
//    let mut corner = Sphere::new(shape_list);
//    corner.transform = translation(0.0, 0.0, -1.0) * scaling(0.25, 0.25, 0.25);
//
//    Box::new(corner).set_parent(parent)
//}

pub fn draw_hexagon_scene() {
    // Options
    let canvas_width = 100;
    let canvas_height = 100;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let pattern_b = RingPattern::new(Color::from_hex("FFE4C6"), Color::from_hex("B5BD89"));
//    let pattern_b = StripePattern::new(Color::from_hex("EFEF56"), Color::from_hex("FCEFEF"));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_b), 0.15);
    pattern.set_transform(rotation_y(PI/3.0) * scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.ambient = Float(0.15);
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    // Okay so nested groups are not working
    // Properly right now
    // I blame rust's lack of cyclic references
//    let mut hexagon = hexagon();
//    hexagon.set_transform(translation(0.0, 1.0, 0.0));
////    let material = Material::mirror();
//    let mut material = Material::new();
//    material.color = Color::from_hex("729EA1");
//    hexagon.set_material(material);
//    world.objects.push(hexagon);

    let mut s1: Box<dyn Shape> = Box::new(Sphere::new(&mut shape_list));
    let mut s2: Box<dyn Shape> = Box::new(Sphere::new(&mut shape_list));
    let mut s3: Box<dyn Shape> = Box::new(Sphere::new(&mut shape_list));

    s1.set_transform(translation(1.5, 1.0, 0.0), &mut shape_list);
    s2.set_transform(translation(0.0, 1.0, 0.0), &mut shape_list);
    s3.set_transform(translation(-1.5, 1.0, 0.0), &mut shape_list);

    let mut g = Group::new(&mut shape_list);
    g.add_child(&mut s1, &mut shape_list);
    g.add_child(&mut s2, &mut shape_list);
    g.add_child(&mut s3, &mut shape_list);
    world.objects.push(Box::new(g));

//    world.objects.push(Box::new(s1));
//    world.objects.push(Box::new(s2));
//    world.objects.push(Box::new(s3));


    let light = Light::point_light(&point(-10.0, 16.0, -10.0), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(0.0, 1.5, -5.0), point(0.0, 1.0, 0.0), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("hexagon_scene.ppm"))
}

//--------------------------------------------------

pub fn draw_cone_scene() {
    // Options
    let canvas_width = 500;
    let canvas_height = 500;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let pattern_b = RingPattern::new(Color::from_hex("FFE4C6"), Color::from_hex("B5BD89"));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_b), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.ambient = Float(0.15);
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut middle_cone = Cone::new_bounded(-1.0, 1.0);
    middle_cone.closed = true;
    middle_cone.transform = translation(0.0, 2.0, 0.0) * scaling(1.0, 2.0, 1.0);
    let material = Material::mirror();
//    let mut material = Material::new();
//    material.color = Color::from_hex("729EA1");
    middle_cone.material = material;
    world.objects.push(Box::new(middle_cone));

    let colors = vec![
        Color::from_hex("FF0000"),
        Color::from_hex("FF00FF"),
        Color::from_hex("0000FF"),
        Color::from_hex("00cc00"),
        Color::from_hex("FFFF00"),

        Color::from_hex("FF0000"),
        Color::from_hex("FF00FF"),
        Color::from_hex("0000FF"),
        Color::from_hex("00cc00"),
        Color::from_hex("FFFF00"),

        Color::from_hex("FF0000"),
    ];
    for i in 0..colors.len() {
        let rotation = PI/6.0 + PI/6.0 * i as f64;
        let mut cylinder = Cylinder::new_bounded(0.0, 2.0);
        cylinder.closed = true;
        cylinder.transform = rotation_y(rotation) * translation(0.0, 1.0, -3.0) * scaling(0.4, 1.0, 0.4);
//        let material = Material::mirror();
        let mut material = Material::new();
        material.color = colors[i];
        cylinder.material = material;
        world.objects.push(Box::new(cylinder));

        let mut glass_sphere = Sphere::new(&mut shape_list);
        glass_sphere.transform = rotation_y(rotation) * translation(0.0, 3.5, -3.0) * scaling(0.2, 0.2, 0.2);
        let material = Material::glass();
        glass_sphere.material = material;
        world.objects.push(Box::new(glass_sphere));

        glass_sphere = Sphere::new(&mut shape_list);
        glass_sphere.transform = rotation_y(rotation) * translation(0.0, 0.2, -3.0) * scaling(0.2, 0.2, 0.2);
        let material = Material::glass();
        glass_sphere.material = material;
        world.objects.push(Box::new(glass_sphere));
    }

    let light = Light::point_light(&point(-10.0, 16.0, -10.0), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(0.0, 3.5, -6.5), point(0.0, 2.0, 0.0), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("cone_scene.ppm"))
}

//--------------------------------------------------


pub fn draw_cylinder_refracted_scene() {
    // Options
    let canvas_width = 1000;
    let canvas_height = 1000;
    let fov = PI/4.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let pattern_b = RingPattern::new(Color::from_hex("FFE4C6"), Color::from_hex("B5BD89"));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_b), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut middle_cylinder = Cylinder::new_bounded(0.0, 3.0);
    middle_cylinder.closed = true;
//    middle_cylinder.transform = scaling(0.7, 1.0, 0.7);
    let material = Material::glass();
//    let mut material = Material::new();
//    material.color = Color::from_hex("729EA1");
    middle_cylinder.material = material;
    world.objects.push(Box::new(middle_cylinder));

    let colors = vec![
        Color::from_hex("FF0000"),
        Color::from_hex("FF00FF"),
        Color::from_hex("0000FF"),
        Color::from_hex("00cc00"),
    ];
    for i in 0..colors.len() {
        let mut cylinder = Cylinder::new_bounded(0.0, 2.0);
        cylinder.closed = true;
        cylinder.transform = rotation_y(PI - PI/6.0 * i as f64) * translation(0.0, 0.0, -3.0) * scaling(0.4, 1.0, 0.4);
//        let material = Material::mirror();
        let mut material = Material::new();
        material.color = colors[i];
        cylinder.material = material;
        world.objects.push(Box::new(cylinder));

        let mut glass_sphere = Sphere::new(&mut shape_list);
        glass_sphere.transform = rotation_y(PI - PI/6.0 * i as f64) * translation(0.0, 2.5, -3.0) * scaling(0.2, 0.2, 0.2);
        let material = Material::glass();
        glass_sphere.material = material;
        world.objects.push(Box::new(glass_sphere));
    }

    let colors = vec![
        Color::from_hex("FCEFEF"),
        Color::from_hex("7FD8BE"),
        Color::from_hex("A1FCDF"),
        Color::from_hex("FCD29F"),
    ];
    for i in 0..colors.len() {
        let mut cylinder = Cylinder::new_bounded(0.0, 0.4);
        let height = (i as f64 + 1.0) * 0.44;
        let width =  (i as f64 + 1.0) * -0.4;
        cylinder.transform = rotation_y(-PI/9.0) * translation(0.0, 0.0, -3.5) * scaling(2.0 + width, 1.0 + height, 2.0 + width);
//        cylinder.transform = rotation_y(PI/6.0 * i as f64) * translation(0.0, 0.0, -3.0) * scaling(0.4, 1.0, 0.4);
//        cylinder.transform = rotation_y(PI/3.0) * translation(0.0, 0.0, -3.5) * scaling(2.0, 1.0, 2.0);
//        let material = Material::mirror();
        let mut material = Material::new();
        material.color = colors[i];

        if i == colors.len()-1 {
            cylinder.closed = true;
            material.shininess = Float(300.0);
            material.reflective = Float(0.9)
        }

        cylinder.material = material;
        world.objects.push(Box::new(cylinder));
    }

    let light = Light::point_light(&point(-10.0, 16.0, -10.0), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(0.0, 5.0, -10.0), point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("cylinder_refracted_scene.ppm"))
}

//--------------------------------------------------

pub fn draw_cylinder_scene() {
    // Options
    let canvas_width = 1000;
    let canvas_height = 1000;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let pattern_b = RingPattern::new(Color::from_hex("FFE4C6"), Color::from_hex("B5BD89"));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_b), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut middle_cylinder = Cylinder::new_bounded(0.0, 3.0);
    middle_cylinder.closed = true;
//    middle_cylinder.transform = scaling(0.7, 1.0, 0.7);
    let material = Material::mirror();
//    let mut material = Material::new();
//    material.color = Color::from_hex("729EA1");
    middle_cylinder.material = material;
    world.objects.push(Box::new(middle_cylinder));

    let colors = vec![
        Color::from_hex("FF0000"),
        Color::from_hex("FF00FF"),
        Color::from_hex("0000FF"),
        Color::from_hex("00cc00"),
    ];
    for i in 0..colors.len() {
        let mut cylinder = Cylinder::new_bounded(0.0, 2.0);
        cylinder.closed = true;
        cylinder.transform = rotation_y(PI/6.0 * i as f64) * translation(0.0, 0.0, -3.0) * scaling(0.4, 1.0, 0.4);
//        let material = Material::mirror();
        let mut material = Material::new();
        material.color = colors[i];
        cylinder.material = material;
        world.objects.push(Box::new(cylinder));

        let mut glass_sphere = Sphere::new(&mut shape_list);
        glass_sphere.transform = rotation_y(PI/6.0 * i as f64) * translation(0.0, 2.5, -3.0) * scaling(0.2, 0.2, 0.2);
        let material = Material::glass();
        glass_sphere.material = material;
        world.objects.push(Box::new(glass_sphere));
    }


    let light = Light::point_light(&point(-10.0, 16.0, -10.0), &Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    // Create camera and render scene
    let mut camera = Camera::new(canvas_width, canvas_height, fov);
    camera.transform = view_transform(point(0.0, 4.5, -5.0), point(0.0, 1.0, 0.0), vector(0.0, 1.0, 0.0));

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("cylinder_scene.ppm"))
}

//--------------------------------------------------

pub fn draw_refracted_scene() {
    // Options
    let canvas_width = 100;
    let canvas_height = 100;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let pattern_b = RingPattern::new(Color::from_hex("FF0000"), Color::new(0.2, 0.2, 0.6));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_b), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("FFE2BA");
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut glass_sphere = Sphere::new(&mut shape_list);
    glass_sphere.transform = translation(-0.5, 0.45, -2.0) * scaling(0.45, 0.45, 0.45);
    let material = Material::glass();
    glass_sphere.material = material;
    world.objects.push(Box::new(glass_sphere));

    let mut pedestal = Cube::new(&mut shape_list);
    pedestal.transform = translation(0.8, 1.0, -1.0) * rotation_y(PI/6.0) * scaling(0.2, 1.0, 0.5);
    let mut material = Material::glass();
    material.diffuse = Float(0.01);
    material.refractive_index = Float(1.8);
    pedestal.material = material;
    world.objects.push(Box::new(pedestal));

    let mut middle_sphere = Sphere::new(&mut shape_list);
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    let pattern_a = RingPattern::new(Color::from_hex("F4C095"), Color::from_hex("679289"));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1) * transformation::rotation_y(PI/6.0) * transformation::rotation_x(-PI/6.0));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));

    let mut right_sphere = Sphere::new(&mut shape_list);
    right_sphere.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let mut pattern = StripePattern::new(Color::white(), Color::black());
    pattern.set_transform(transformation::rotation_z(-PI/12.0) * transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("56D8CD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    right_sphere.material = material;
    world.objects.push(Box::new(right_sphere));

    let mut left_sphere = Sphere::new(&mut shape_list);
    left_sphere.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    let mut material = Material::new();
    material.reflective = Float(0.7);
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

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("refracted_scene.ppm"))
}

//--------------------------------------------------

pub fn draw_reflected_scene() {
    // Options
    let canvas_width = 1000;
    let canvas_height = 1000;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let pattern_b = RingPattern::new(Color::from_hex("FF0000"), Color::new(0.2, 0.2, 0.6));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_b), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("FFE2BA");
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut middle_sphere = Sphere::new(&mut shape_list);
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    let pattern_a = RingPattern::new(Color::from_hex("F4C095"), Color::from_hex("679289"));
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1) * transformation::rotation_y(PI/6.0) * transformation::rotation_x(-PI/6.0));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));

    let mut right_sphere = Sphere::new(&mut shape_list);
    right_sphere.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let mut material = Material::new();
    material.reflective = Float(0.4);
    let mut pattern = StripePattern::new(Color::white(), Color::black());
    pattern.set_transform(transformation::rotation_z(-PI/12.0) * transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("56D8CD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    right_sphere.material = material;
    world.objects.push(Box::new(right_sphere));

    let mut left_sphere = Sphere::new(&mut shape_list);
    left_sphere.transform = translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33);
    let mut material = Material::new();
    material.reflective = Float(0.7);
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

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("refracted_scene.ppm"))
}

//--------------------------------------------------

pub fn draw_perturbed_patterned_scene() {
    // Options
    let canvas_width = 100;
    let canvas_height = 100;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    let pattern_b = RingPattern::new(Color::from_hex("FF0000"), Color::black());
//    let pattern_b = StripePattern::new(Color::from_hex("0000FF"), Color::black());
    let mut pattern = PerturbedPattern::new(Box::new(pattern_b), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("FFE2BA");
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut middle_sphere = Sphere::new(&mut shape_list);
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    let pattern_a = RingPattern::new(Color::from_hex("F4C095"), Color::from_hex("679289"));
//    let pattern_a = RingPattern::new(Color::from_hex("679289"), Color::black());
    let mut pattern = PerturbedPattern::new(Box::new(pattern_a), 0.15);
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1) * transformation::rotation_y(PI/6.0) * transformation::rotation_x(-PI/6.0));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));

    let mut right_sphere = Sphere::new(&mut shape_list);
    right_sphere.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let mut material = Material::new();
    let mut pattern = StripePattern::new(Color::white(), Color::black());
    pattern.set_transform(transformation::scaling(0.5, 0.5, 0.5));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("56D8CD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    right_sphere.material = material;
    world.objects.push(Box::new(right_sphere));

    let mut left_sphere = Sphere::new(&mut shape_list);
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

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("patterned_scene_perturbed.ppm"))
}

//--------------------------------------------------

pub fn draw_blended_patterned_scene() {
    // Options
    let canvas_width = 500;
    let canvas_height = 500;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    let pattern_a = RingPattern::new(Color::from_hex("FF0000"), Color::black());
    let pattern_b = CheckerPattern::new(Color::from_hex("0000FF"), Color::black());
    let mut pattern = BlendedPattern::new(Box::new(pattern_a), Box::new(pattern_b));
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("FFE2BA");
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut middle_sphere = Sphere::new(&mut shape_list);
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    let mut pattern = GradientPattern::new(Color::from_hex("679289"), Color::from_hex("F4C095"));
    pattern.set_transform(transformation::scaling(2.0, 2.0, 2.0) * transformation::rotation_y(PI/2.0));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));

    let mut right_sphere = Sphere::new(&mut shape_list);
    right_sphere.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let mut material = Material::new();
    let mut pattern = StripePattern::new(Color::white(), Color::black());
    pattern.set_transform(transformation::scaling(0.5, 0.5, 0.5));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("56D8CD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    right_sphere.material = material;
    world.objects.push(Box::new(right_sphere));

    let mut left_sphere = Sphere::new(&mut shape_list);
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

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("patterned_scene_blended.ppm"))
}

//--------------------------------------------------

pub fn draw_patterned_scene() {
    // Options
    let canvas_width = 100;
    let canvas_height = 100;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    let mut pattern = RingPattern::new(Color::from_hex("EE2E31"), Color::black());
    pattern.set_transform(transformation::scaling(0.1, 0.1, 0.1));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("FFE2BA");
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut middle_sphere = Sphere::new(&mut shape_list);
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    let mut pattern = GradientPattern::new(Color::from_hex("679289"), Color::from_hex("F4C095"));
    pattern.set_transform(transformation::scaling(2.0, 2.0, 2.0) * transformation::rotation_y(PI/2.0));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));

    let mut right_sphere = Sphere::new(&mut shape_list);
    right_sphere.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let mut material = Material::new();
    let mut pattern = StripePattern::new(Color::white(), Color::black());
    pattern.set_transform(transformation::scaling(0.5, 0.5, 0.5));
    material.set_pattern(Box::new(pattern));
    material.color = Color::from_hex("56D8CD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    right_sphere.material = material;
    world.objects.push(Box::new(right_sphere));

    let mut left_sphere = Sphere::new(&mut shape_list);
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

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("patterned_scene.ppm"))
}

//--------------------------------------------------

pub fn draw_scene_on_a_plane() {
    // Options
    let canvas_width = 400;
    let canvas_height = 400;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Plane::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.color = Color::from_hex("FFE2BA");
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut middle_sphere = Sphere::new(&mut shape_list);
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));

    let mut right_sphere = Sphere::new(&mut shape_list);
    right_sphere.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let mut material = Material::new();
    material.color = Color::from_hex("56D8CD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    right_sphere.material = material;
    world.objects.push(Box::new(right_sphere));

    let mut left_sphere = Sphere::new(&mut shape_list);
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

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("scene_on_a_plane.ppm"))
}

//--------------------------------------------------

pub fn draw_first_scene() {
    // Options
    let canvas_width = 100;
    let canvas_height = 100;
    let fov = PI/3.0;

    // Construct world
    let mut world = World::new();
    let mut shape_list = ShapeList::new();

    let mut floor = Sphere::new(&mut shape_list);
    floor.transform = scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.color = Color::from_hex("F2E2BA");
    material.specular = Float(0.0);
    floor.material = material;
    world.objects.push(Box::new(floor));

    let mut left_wall = Sphere::new(&mut shape_list);
    left_wall.transform = translation(0.0, 0.0, 5.0) *
        rotation_y(-PI/4.0) * rotation_x(PI/2.0) *
        scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.color = Color::from_hex("D3F9FF");
    left_wall.material = material;
    world.objects.push(Box::new(left_wall));

    let mut right_wall = Sphere::new(&mut shape_list);
    right_wall.transform = translation(0.0, 0.0, 5.0) *
        rotation_y(PI/4.0) * rotation_x(PI/2.0) *
        scaling(10.0, 0.01, 10.0);
    let mut material = Material::new();
    material.color = Color::from_hex("D3F9FF");
    right_wall.material = material;
    world.objects.push(Box::new(right_wall));

    let mut middle_sphere = Sphere::new(&mut shape_list);
    middle_sphere.transform = translation(-0.5, 1.0, 0.5);
    let mut material = Material::new();
    material.color = Color::from_hex("7AC16C");
    material.diffuse = Float(0.8);
    material.specular = Float(0.7);
    middle_sphere.material = material;
    world.objects.push(Box::new(middle_sphere));

    let mut right_sphere = Sphere::new(&mut shape_list);
    right_sphere.transform = translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5);
    let mut material = Material::new();
    material.color = Color::from_hex("56D8CD");
    material.diffuse = Float(0.7);
    material.specular = Float(0.3);
    right_sphere.material = material;
    world.objects.push(Box::new(right_sphere));

    let mut left_sphere = Sphere::new(&mut shape_list);
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

    let canvas = camera.render(world, &mut shape_list);
    file::write_to_file(canvas.to_ppm(), String::from("first_scene.ppm"))
}

//--------------------------------------------------

pub fn draw_shaded_circle() {
    let canvas_pixels = 500;

    let mut shape_list = ShapeList::new();

    let mut material = Material::new();
    material.color = Color::from_hex("19647E");
    let shape = Sphere::new_with_material(material, &mut shape_list);

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
            let xs = shape.intersects(&ray, &mut shape_list);
            let hit = hit(xs);
            if hit != None {
                let point = &ray.position(hit.as_ref().unwrap().t.value());
                let normal = shape::normal_at(hit.as_ref().unwrap().object.clone(), *point, &mut shape_list);
                let eye = -&ray.direction;
                let object = hit.as_ref().unwrap().object.clone();

                let color = Light::lighting(&object.material(), Some(object), None, &light, point, None, &eye, &normal, false, None);
                canvas.write_pixel(x, y, &color);
            }
        }
    }
    file::write_to_file(canvas.to_ppm(), String::from("shaded_circle.ppm"))
}
//--------------------------------------------------

pub fn draw_uniform_rand_circle() {
    // Something similar could be used to create
    // soft shadows

    let mut shape_list = ShapeList::new();
    let color = Color::new(1.0, 0.6, 0.1);
    let center_color = Color::from_hex("00FF00");
    let shape = Sphere::new(&mut shape_list);
//    shape.set_transform(transformation::scaling(0.5, 1.0, 1.0));
    let canvas_pixels = 500;

    let ray_count = 10000;
    let ray_shot_radius = canvas_pixels as f64 * 0.5;

    let wall_z = 10.0;
    let wall_size = 7.0;

    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let center = canvas_pixels as f64 / 2.0;

    let ray_origin = point(0.0, 0.0, -5.0);
    let canvas = &mut Canvas::new(canvas_pixels, canvas_pixels);

    let mut rng = rand::thread_rng();

    // Color center
    canvas.write_pixel(center as i32, center as i32, &center_color);

    // Each row of pixels
    for _ in 0..ray_count {

        let a = rng.gen::<f64>() * 2.0 * PI;
        let r = 1.0 * rng.gen::<f64>().sqrt();
        let x = r * a.cos();
        let y = r * a.sin();

        let x_scale = x * ray_shot_radius;
        let y_scale = y * ray_shot_radius;

        let x_pix = center + x_scale;
        let y_pix = center + y_scale;

        let world_x = -half + pixel_size * x_pix;
        let world_y = half - pixel_size * y_pix;

        let position = point(world_x, world_y, wall_z);

        let ray = Ray::new(ray_origin, (position - ray_origin).normalize());
        let xs = shape.intersects(&ray, &mut shape_list);

        if hit(xs) != None {
            canvas.write_pixel(x_pix as i32, y_pix as i32, &color);
        }
    }
    file::write_to_file(canvas.to_ppm(), String::from("circle_rand.ppm"))
}

//--------------------------------------------------

pub fn draw_circle() {
    let mut shape_list = ShapeList::new();
    let color = Color::new(1.0, 0.6, 0.1);
    let shape = Sphere::new(&mut shape_list);
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
            let xs = shape.intersects(&ray, &mut shape_list);

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
