/// # world
/// `world` is a module to represent the collection of objects that make up a scene

use crate::light::Light;
use crate::shape::Shape;
use crate::shape::sphere::Sphere;
use crate::material::Material;
use crate::color::Color;
use crate::float::Float;
use crate::transformation;
use crate::tuple::point;
use crate::ray::Ray;
use crate::intersection::Intersection;

pub struct World {
    pub objects: Vec<Box<dyn Shape>>,
    pub lights: Vec<Light>
}

impl World {
    pub fn new() -> World {
        World {objects: vec![], lights: vec![]}
    }

    pub fn default_world() -> World {
        let light = Light::point_light(&point(-10.0, -10.0, -10.0), &Color::new(1.0, 1.0, 1.0));

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

}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::tuple::vector;

    #[test]
    fn world_creation() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);


        let w = World::default_world();
        let light = Light::point_light(&point(-10.0, -10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
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
}