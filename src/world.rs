/// # world
/// `world` is a module to represent the collection of objects that make up a scene

use crate::light::Light;
use crate::shape::Shape;
use crate::shape::sphere::Sphere;
use crate::material::Material;
use crate::color::Color;
use crate::float::Float;
use crate::{transformation, light, intersection};
use crate::tuple::{point, Tuple};
use crate::ray::Ray;
use crate::intersection::{Intersection, PrecomputedData};

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>
}

impl World {
    pub fn new() -> World {
        World {objects: vec![], lights: vec![]}
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

        World {objects: vec![Box::new(sphere1), Box::new(sphere2)], lights: vec![light]}
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

    pub fn shade_hit(&self, comps: PrecomputedData<Box<dyn Shape>>) -> Color {
        // One light implementation for now
        light::lighting(&comps.object.material(), Some(comps.object), &self.lights[0], &comps.point, &comps.eyev, &comps.normalv, self.is_shadowed(comps.over_point))
    }

    pub fn color_at(&self, ray: &Ray) -> Color {
        let intersections = self.intersects(ray);
        let hit = intersection::hit(intersections);
        if hit == None {return Color::new(0.0, 0.0, 0.0)}  // Return black of no hits
        let comps = intersection::prepare_computations(hit.unwrap(), ray);
        self.shade_hit(comps)
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
        let comps = intersection::prepare_computations(i, &r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));

        // Shading an intsersection from the inside
        let mut w = World::default_world();
        w.lights[0] = Light::point_light(&point(0.0, 0.25, 0.0), &Color::new(1.0, 1.0, 1.0));
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = intersection::prepare_computations(i, &r);
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
        let comps = intersection::prepare_computations(i, &r);
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
}