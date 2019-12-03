/// # world
/// `world` is a module to represent the collection of objects that make up a scene

use crate::light::Light;
use crate::shape::Shape;
use crate::shape::sphere::Sphere;
use crate::material::Material;
use crate::color::Color;
use crate::float::Float;
use crate::{transformation, light, intersection, tuple};
use crate::tuple::{point, Tuple};
use crate::ray::Ray;
use crate::intersection::{Intersection, PrecomputedData};

const DEFAULT_RAY_BOUNCES: i32 = 4;

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>,
    pub max_recursion: i32,
}

impl World {
    pub fn new() -> World {
        World {objects: vec![], lights: vec![], max_recursion: DEFAULT_RAY_BOUNCES}
    }

    pub fn default_world() -> World {
        let light = Light::point_light(&point(-10.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));

        let mut material = Material::new();
        material.color = Color::new(0.8, 1.0, 0.6);
        material.diffuse = Float(0.7);
        material.specular = Float(0.2);
        let sphere1 = Sphere::new_with_material(material);

        let mut sphere2 = Sphere::new();
        sphere2.set_transform(transformation::scaling(0.5, 0.5, 0.5));

        World {objects: vec![Box::new(sphere1), Box::new(sphere2)], lights: vec![light], max_recursion: DEFAULT_RAY_BOUNCES}
    }

    pub fn contains_object(&self, object: &Box<dyn Shape>) -> bool {
        self.objects.contains(object)
    }

    pub fn contains_light(&self, light: &Light) -> bool {
        self.lights.contains(light)
    }

    pub fn intersects(&self, ray: &Ray) -> Vec<Intersection<Box<dyn Shape>>> {
        let mut intersections = vec![];

        for object in self.objects.iter() {
            intersections.append(&mut object.intersects(&ray));
        }
        // Sort intersections ascending by t value
        intersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        intersections
    }

    /// Returns the color in the world at what the ray is intersecting with
    /// uses the default max_recursion value and is a wrapper for color_at_impl
    /// # Arguments
    /// * `ray` Ray to shoot into the world
    pub fn color_at(&self, ray: &Ray) -> Color {
        self.color_at_impl(ray, self.max_recursion)
    }

    /// Returns the color in the world at what the ray is intersecting with
    /// # Arguments
    /// * `ray` Ray to shoot into the world
    /// * `remaining` Remaining amount of recursions allowed
    pub fn color_at_impl(&self, ray: &Ray, remaining: i32) -> Color {
        let intersections = self.intersects(ray);
        let hit = intersection::hit(intersections.clone());
        if hit == None {return Color::new(0.0, 0.0, 0.0)}  // Return black of no hits
        let comps = intersection::prepare_computations(hit.unwrap(), ray, intersections);
        self.shade_hit_impl(comps, remaining)
    }

    /// Returns the color of a point in the world taking into account shadow and reflection
    /// uses the default max_recursion value and is a wrapper for shade_hit_impl
    /// # Arguments
    /// * `comps` Precomputed data of a ray intersection
    pub fn shade_hit(&self, comps: PrecomputedData<Box<dyn Shape>>) -> Color {
        self.shade_hit_impl(comps, self.max_recursion)
    }

    /// Returns the color of a point in the world taking into account shadow and reflection
    /// # Arguments
    /// * `comps` Precomputed data of a ray intersection
    /// * `remaining` Remaining amount of recursions allowed
    pub fn shade_hit_impl(&self, comps: PrecomputedData<Box<dyn Shape>>, remaining: i32) -> Color {
        // One light implementation for now
        let is_shadowed = self.is_shadowed(comps.over_point);
        let reflected = self.reflected_color_impl(comps.clone(), remaining);
        let refracted = self.refracted_color_impl(comps.clone(), remaining);
        let surface = light::lighting(&comps.object.material(), Some(comps.object), &self.lights[0], &comps.point, &comps.eyev, &comps.normalv, is_shadowed);
        surface + reflected + refracted
    }

    /// Returns the color at a reflected ray in the world
    /// uses the default max_recursion value and is a wrapper for reflected_color_impl
    /// # Arguments
    /// * `comps` Precomputed data of a ray intersection
    pub fn reflected_color(&self, comps: PrecomputedData<Box<dyn Shape>>) -> Color {
        self.reflected_color_impl(comps, self.max_recursion)
    }

    /// Returns the color at a reflected ray in the world
    /// # Arguments
    /// * `comps` Precomputed data of a ray intersection
    /// * `remaining` Remaining amount of recursions allowed
    pub fn reflected_color_impl(&self, comps: PrecomputedData<Box<dyn Shape>>, remaining: i32) -> Color {
        // If no more rays remain, return black
        if remaining < 1 {
            return Color::black();
        }

        let reflective = comps.object.material().reflective;
        if reflective == Float(0.0) {
            return Color::black()
        }

        // Shoot a new reflected ray out into the world
        let reflected_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at_impl(&reflected_ray, remaining-1); // decrement remaining ray value

        color * reflective.value()
    }

    /// Returns the color at a refracted ray in the world
    /// uses the default max_recursion value and is a wrapper for reflected_color_impl
    /// # Arguments
    /// * `comps` Precomputed data of a ray intersection
    pub fn refracted_color(&self, comps: PrecomputedData<Box<dyn Shape>>) -> Color {
        self.refracted_color_impl(comps, self.max_recursion)
    }

    /// Returns the color at a refracted ray in the world
    /// # Arguments
    /// * `comps` Precomputed data of a ray intersection
    /// * `remaining` Remaining amount of recursions allowed
    pub fn refracted_color_impl(&self, comps: PrecomputedData<Box<dyn Shape>>, remaining: i32) -> Color {
        // If no more rays remain, return black
        if remaining < 1 {
            return Color::black();
        }

        // Check for transparency
        let transparency = comps.object.material().transparency;
        if transparency == Float(0.0) {
            return Color::black();
        }

        // Check for total refraction, if so return black
        // First find ratio of the 2 indices of refraction
        let n_ratio = comps.n1 / comps.n2;

        let cos_i = tuple::dot(&comps.eyev, &comps.normalv);
        // via trig identity
        let sin2_t = n_ratio * n_ratio * (1.0 - cos_i * cos_i);
        if sin2_t > Float(1.0) {
            return Color::black();
        }

        // Find cos(theta_t)
        let cos_t = (1.0 - sin2_t).sqrt();

        // Compute direction of the refracted ray
        let direction = comps.normalv * (n_ratio * cos_i - cos_t).value() - comps.eyev * n_ratio.value();

        // Create the refracted ray
        let refract_ray = Ray::new(comps.under_point, direction);

        // Find the color of the refracted ray in the world
        let color = self.color_at_impl(&refract_ray, remaining-1);

        color * transparency.value()
    }

    pub fn is_shadowed(&self, point: Tuple) -> bool {
        // One light implementation for now
        let vector = self.lights[0].position - point;
        let distance = vector.magnitude();
        let direction = vector.normalize();

        let ray = Ray::new(point, direction);
        let intersections = self.intersects(&ray);

        let hit = intersection::hit(intersections);

        // If there is a hit and the t value is less than the distance to the light, return true
        if hit != None {
            if hit.unwrap().t < Float(distance) {
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::tuple::vector;
    use crate::intersection;
    use crate::transformation::translation;
    use crate::intersection::{prepare_computations_single_intersection, prepare_computations};
    use crate::shape::plane::Plane;
    use crate::pattern::test_pattern::TestPattern;

    #[test]
    fn world_creation() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);


        let w = World::default_world();
        let light = Light::point_light(&point(-10.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
//        let mut material = Material::new();
//        material.color = Color::new(0.8, 1.0, 0.6);
//        material.diffuse = Float(0.7);
//        material.specular = Float(0.2);
//        let sphere1 = Sphere::new_with_material(material);
//        let mut sphere2 = Sphere::new();
//        sphere2.set_transform(transformation::scaling(0.5, 0.5, 0.5));
        assert!(w.contains_light(&light));
        // Shapes have unique ids
//        assert!(w.contains_object(&CommonShape::Sphere(sphere1)));
//        assert!(w.contains_object(&CommonShape::Sphere(sphere2)));
    }

    #[test]
    fn world_intersections() {
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = w.intersects(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn world_shading() {
        // Shading an intersection
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[0].clone();
        let i = Intersection::new(4.0, shape);
        let comps = intersection::prepare_computations_single_intersection(i, &r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));

        // Shading an intersection from the inside
        let mut w = World::default_world();
        w.lights[0] = Light::point_light(&point(0.0, 0.25, 0.0), &Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = intersection::prepare_computations_single_intersection(i, &r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));

        // shade hit is given an intersection in shadow (SHADOWS!)
        let mut w = World::new();
        w.lights.push(Light::point_light(&point(0.0, 0.0, -10.0), &Color::new(1.0, 1.0, 1.0)));
        let s1 = Sphere::new();
        w.objects.push(Box::new(s1));
        let mut s2 = Sphere::new();
        s2.transform = translation(0.0, 0.0, 10.0);
        w.objects.push(Box::new(s2));
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        let i = Intersection::new(4.0, shape);
        let comps = intersection::prepare_computations_single_intersection(i, &r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn world_color_at() {
        // Ray doesn't intersect anything
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));

        // Intersects outermost sphere
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));

        // Pointing at inner sphere from inside outer sphere
        let mut w = World::default_world();
        let outer = &mut w.objects[0];
        let mut material = outer.material();
        material.ambient = Float(1.0);
        outer.set_material(material);

        let inner = &mut w.objects[1];
        let mut material = inner.material();
        material.ambient = Float(1.0);
        inner.set_material(material);
        let inner_color = inner.material().color;

        let r = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = w.color_at(&r);

        assert_eq!(c, inner_color);
    }

    #[test]
    fn world_is_shadowed() {
        // There is no shadow when nothing is collinear with point and light
        let w = World::default_world();
        let p = point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(p), false);

        // The shadow when an object is between the point and the light
        let w = World::default_world();
        let p = point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(p), true);

        // No shadow when an object is behind the light
        let w = World::default_world();
        let p = point(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(p), false);

        // No shadow when an object is behind the point
        let w = World::default_world();
        let p = point(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(p), false);
    }

    #[test]
    fn world_reflected_color() {
        // Reflecting a non-reflective color
        let w = World::default_world();
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let mut shape = w.objects[1].clone();
        let mut material = shape.material();
        material.ambient = Float(1.0);
        shape.set_material(material);
        let i = Intersection::new(1.0, shape);
        let comps = prepare_computations_single_intersection(i, &r);
        let color = w.reflected_color(comps);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));

        // The reflected color for a reflective material
        let mut w = World::default_world();
        let mut p = Plane::new();
        p.material.reflective = Float(0.5);
        p.transform = translation(0.0, -1.0, 0.0);
        let shape: Box<dyn Shape> = Box::new(p);
        w.objects.push(shape.clone());
        let r = Ray::new(point(0.0, 0.0, -3.0), vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
        let i = Intersection::new(2.0f64.sqrt(), shape);
        let comps = prepare_computations_single_intersection(i, &r);
        let color = w.reflected_color(comps);
        assert_eq!(color, Color::new(0.19033, 0.237915, 0.14274));
    }

    #[test]
    fn world_shade_hit_reflected() {
        // Shade hit with a reflective material
        let mut w = World::default_world();
        let mut p = Plane::new();
        p.material.reflective = Float(0.5);
        p.transform = translation(0.0, -1.0, 0.0);
        let shape: Box<dyn Shape> = Box::new(p);
        w.objects.push(shape.clone());
        let r = Ray::new(point(0.0, 0.0, -3.0), vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
        let i = Intersection::new(2.0f64.sqrt(), shape);
        let comps = prepare_computations_single_intersection(i, &r);
        let color = w.shade_hit(comps);
        assert_eq!(color, Color::new(0.87675, 0.92434, 0.82917));
    }

    #[test]
    fn world_reflected_recursion() {
        // Test if infinite recursion will break the program
        let mut w = World::new();
        let light = Light::point_light(&point(0.0, 0.0, 0.0), &Color::new(1.0, 1.0, 1.0));
        w.lights.push(light);
        let mut lower = Plane::new();
        lower.material.reflective = Float(1.0);
        lower.transform = translation(0.0, -1.0, 0.0);
        w.objects.push(Box::new(lower));
        let mut upper = Plane::new();
        upper.material.reflective = Float(1.0);
        upper.transform = translation(0.0, 1.0, 0.0);
        w.objects.push(Box::new(upper));
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 1.0, 0.0));
        let _c = w.color_at(&r);
        assert!(true); // The previous line terminated properly!
    }

    #[test]
    fn world_reflected_recursion_limit() {
        let mut w = World::default_world();
        let mut p = Plane::new();
        p.material.reflective = Float(0.5);
        p.transform = translation(0.0, -1.0, 0.0);
        let shape: Box<dyn Shape> = Box::new(p);
        w.objects.push(shape.clone());
        let r = Ray::new(point(0.0, 0.0, -3.0), vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
        let i = Intersection::new(2.0f64.sqrt(), shape);
        let comps = prepare_computations_single_intersection(i, &r);
        let color = w.reflected_color_impl(comps, 0);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn world_refracted() {
        let w = World::default_world();
        let shape = w.objects[0].clone();
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = vec![Intersection::new(4.0, shape.clone()), Intersection::new(6.0, shape.clone())];
        let comps = prepare_computations(xs[0].clone(), &r, xs.clone());
        let c = w.refracted_color_impl(comps, 5);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn world_refracted_recursion_limit() {
        // Refracted color at the max depth (should be black)
        let w = World::default_world();
        let mut shape = w.objects[0].clone();
        let mut material = Material::new();
        material.transparency = Float(1.0);
        material.refractive_index = Float(1.5);
        shape.set_material(material);
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = vec![Intersection::new(4.0, shape.clone()), Intersection::new(6.0, shape.clone())];
        let comps = prepare_computations(xs[0].clone(), &r, xs.clone());
        let c = w.refracted_color_impl(comps, 0);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    
    #[test]
    fn world_refracted_total_reflection() {
        let w = World::default_world();
        let mut shape = w.objects[0].clone();
        let mut material = Material::new();
        material.transparency = Float(1.0);
        material.refractive_index = Float(1.5);
        shape.set_material(material);
        let r = Ray::new(point(0.0, 0.0, 2.0f64.sqrt()/2.0), vector(0.0, 1.0, 0.0));
        let xs = vec![Intersection::new(-2.0f64.sqrt()/2.0, shape.clone()), Intersection::new(2.0f64.sqrt()/2.0, shape.clone())];
        // Note we're inside the sphere, so only the second intersection matters to us
        let comps = prepare_computations(xs[1].clone(), &r, xs.clone());
        let c = w.refracted_color_impl(comps, 5);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn world_refracted_finding_color() {
        let w = World::default_world();
        let mut shape_a = w.objects[0].clone();
        let mut material = Material::new();
        material.ambient = Float(1.0);
        material.pattern = Some(Box::new(TestPattern::new()));
        shape_a.set_material(material);
        let mut shape_b = w.objects[1].clone();
        let mut material = Material::new();
        material.transparency = Float(1.0);
        material.refractive_index = Float(1.5);
        shape_b.set_material(material);
        let r = Ray::new(point(0.0, 0.0, 0.1), vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-0.9899, shape_a.clone()),
            Intersection::new(-0.4899, shape_b.clone()),
            Intersection::new(0.4899, shape_b.clone()),
            Intersection::new(0.9899, shape_a.clone()),
        ];
        let comps = prepare_computations(xs[2].clone(), &r, xs.clone());
        let c = w.refracted_color_impl(comps, 5);
//        assert_eq!(c, Color::new(0.0, 0.99888, 0.04725));
        assert_eq!(c, Color::new(0.08, 0.1, 0.06));
    }

    #[test]
    fn world_refracted_shade_hit() {
        let mut w = World::default_world();
        let mut p = Plane::new();
        p.material.transparency = Float(0.5);
        p.material.refractive_index = Float(1.5);
        p.transform = translation(0.0, -1.0, 0.0);
        let shape_p: Box<dyn Shape> = Box::new(p);
        w.objects.push(shape_p.clone());
        let mut b = Plane::new();
        b.material.color = Color::new(1.0, 0.0, 0.0);
        b.material.ambient = Float(0.5);
        b.transform = translation(0.0, -3.5, -0.5);
        let shape_b: Box<dyn Shape> = Box::new(b);
        w.objects.push(shape_b.clone());
        let r = Ray::new(point(0.0, 0.0, -3.0), vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
        let xs = vec![Intersection::new(2.0f64.sqrt(), shape_p)];
        let comps = prepare_computations(xs[0].clone(), &r, xs.clone());
        let color = w.shade_hit_impl(comps, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }
}