/// # light
/// `light` is a module to represent the kinds of lights we could have in our scene

use super::tuple::Tuple;
use super::color::Color;
use crate::material::Material;
use crate::{tuple, intersection};
use crate::float::Float;
use crate::shape::Shape;
use rand::{Rng};
use crate::world::World;
use crate::shape::shape_list::ShapeList;
use crate::ray::Ray;

const DEFAULT_RAY_COUNT: usize = 100;

#[derive(Debug, PartialEq)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
    pub radius: Option<f64>,
    pub ray_count: usize,
}

impl Light {
    pub fn point_light(position: &Tuple, intensity: &Color) -> Light {
        Light {
            position: *position, intensity: *intensity,
            radius: None, ray_count: DEFAULT_RAY_COUNT,
        }
    }
    pub fn area_light(position: &Tuple, intensity: &Color, radius: f64) -> Light {
        Light {
            position: *position, intensity: *intensity,
            radius: Some(radius), ray_count: DEFAULT_RAY_COUNT,
        }
    }

    fn compute_average_rays_to(&self, point: &Tuple, world: &World, shape_list: &mut ShapeList) -> Color {
        let mut rng = rand::thread_rng();
        let mut ray_hits: i32 = 0;
        for _ in 0..self.ray_count {
            let mut x = rng.gen::<f64>() - 0.5;
            let mut y = rng.gen::<f64>() - 0.5;
            let mut z = rng.gen::<f64>() - 0.5;
            let magnitude = (x*x + y*y + z*z).sqrt();
            x /= magnitude;
            y /= magnitude;
            z /= magnitude;

            let distance = rng.gen::<f64>().cbrt() * self.radius.unwrap();
            let random_point = self.position + tuple::point(x * distance, y * distance, z * distance);
            let mut vector = random_point - point;
            vector.w = Float(0.0);
            let to_light_distance = vector.magnitude();
            let direction = vector.normalize();

            let ray = Ray::new(*point, direction);
            let intersections = world.intersects(&ray, shape_list);
            let hit = intersection::hit(intersections);

            // If there is a hit and the t value is less than the distance to the light,
            // add a hit counter
            if hit.is_some() {
                if hit.unwrap().t < Float(to_light_distance) {
                    ray_hits += 1;
                }
            }
        }
        let average_ray_hits = (self.ray_count as i32 - ray_hits) as f64 / self.ray_count as f64;
        let ret = Color::new(average_ray_hits, average_ray_hits, average_ray_hits);
        ret
    }


    pub fn lighting(material: &Material,
                    object: Option<Box<dyn Shape>>,
                    world: Option<&World>,
                    light_source: &Light,
                    point: &Tuple,
                    over_point: Option<&Tuple>,
                    eye_v: &Tuple,
                    normal_v: &Tuple,
                    in_shadow: bool,
                    shape_list: Option<&mut ShapeList>) -> Color {

        let color: Color;
        if object != None && material.pattern != None {
            color = material.pattern.clone().unwrap().pattern_at_object(object.clone().unwrap(), point);
        } else {
            color = material.color.clone();
        }

        // Combine surface color with the light's color
        let effective_color = color * light_source.intensity;

        // Find the direction to the light source
        let light_v = (light_source.position - point).normalize();

        // Compute ambient
        let ambient = effective_color * material.ambient.value();

        let diffuse: Color;
        let specular: Color;

        let light_intensity: Color;

        // Find cosine of the angle between light_v and normal_v
        // A negative number means the light is on the other side of the surface
        let light_dot_normal = Float(tuple::dot(&light_v, &normal_v));

        // If the light does not have soft shadows
        if light_source.radius == None {

            // If light misses the surface or the surface is in shadow,
            // ignore diffuse and specular components
            if light_dot_normal < Float(0.0) || in_shadow {
                diffuse = Color::new(0.0, 0.0, 0.0); // black
                specular = Color::new(0.0, 0.0, 0.0); // black
                return ambient + diffuse + specular
            }
            light_intensity = light_source.intensity;
        } else {
            // Compute light intensity for soft shadows by averaging ray misses
            light_intensity = light_source.compute_average_rays_to(over_point.unwrap(), world.unwrap(), shape_list.unwrap());
        }

        // Compute diffuse
        diffuse = color * light_intensity * material.diffuse.value() * light_dot_normal.value();

        // Find cosine of the angle between reflect_v and eye_v
        // a negative number means the light reflects away from the eye
        let reflect_v = (-light_v).reflect(normal_v);
        let reflect_dot_eye = Float(tuple::dot(&reflect_v, &eye_v));

        if reflect_dot_eye <= Float(0.0) {
            specular = Color::new(0.0, 0.0, 0.0); // black
        } else {
            // Compute Specular
            let factor = reflect_dot_eye.value().powf(material.shininess.value());
            specular = light_intensity * material.specular.value() * factor;
        }

        ambient + diffuse + specular
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{point, vector};

    #[test]
    fn light_point_light_creation() {
        let i = Color::new(1.0, 1.0, 1.0);
        let p = point(0.0, 0.0, 0.0);
        let light = Light::point_light(&p, &i);
        assert_eq!(light.position, p);
        assert_eq!(light.intensity, i);
    }

    #[test]
    fn light_lighting() {
        let m = Material::new();
        let position = point(0.0, 0.0, 0.0);

        let in_shadow = false;

        // Lighting with the eye between the light and the surface
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 0.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let result = Light::lighting(&m, None, None, &light, &position, None, &eye_v, &normal_v, in_shadow, None);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));

        // Lighting with the eye between the light and surface, eye offset 45 degrees
        let eye_v = vector(0.0, 2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 0.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let result = Light::lighting(&m, None, None, &light, &position, None, &eye_v, &normal_v, in_shadow, None);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));

        // Lighting with eye opposite surface, light offset 45 degrees
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let result = Light::lighting(&m, None, None, &light, &position, None, &eye_v, &normal_v, in_shadow, None);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));

        // Lighting with eye in the path of the reflection vector
        let eye_v = vector(0.0, -2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let result = Light::lighting(&m, None, None, &light, &position, None, &eye_v, &normal_v, in_shadow, None);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));

        // Lighting with the light behind the surface
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 0.0, 10.0), &Color::new(1.0, 1.0, 1.0));
        let result = Light::lighting(&m, None, None, &light, &position, None, &eye_v, &normal_v, in_shadow, None);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn light_lighting_shadows() {
        let m = Material::new();
        let position = point(0.0, 0.0, 0.0);
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 0.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = Light::lighting(&m, None, None, &light, &position, None, &eye_v, &normal_v, in_shadow, None);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
