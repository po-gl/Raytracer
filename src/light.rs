/// # light
/// `light` is a module to represent the kinds of lights we could have in our scene

use super::tuple::Tuple;
use super::color::Color;
use crate::material::Material;
use crate::tuple;
use crate::float::Float;

#[derive(Debug, PartialEq)]
pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
}

impl Light {
    pub fn point_light(position: &Tuple, intensity: &Color) -> Light {
        Light {position: *position, intensity: *intensity}
    }
}

pub fn lighting(material: &Material,
                light_source: &Light,
                point: &Tuple,
                eye_v: &Tuple,
                normal_v: &Tuple) -> Color {
    // Combine surface color with the light's color
    let effective_color = material.color * light_source.intensity;

    // Find the direction to the light source
    let light_v = (light_source.position - point).normalize();

    // Compute ambient
    let ambient = effective_color * material.ambient.value();

    let diffuse: Color;
    let specular: Color;

    // Find cosine of the angle between light_v and normal_v
    // A negative number means the light is on the other side of the surface
    let light_dot_normal = Float(tuple::dot(&light_v, &normal_v));

    if light_dot_normal < Float(0.0) {
        diffuse = Color::new(0.0, 0.0, 0.0); // black
        specular = Color::new(0.0, 0.0, 0.0); // black
    } else {
        // Compute diffuse
        diffuse = effective_color * material.diffuse.value() * light_dot_normal.value();

        // Find cosine of the angle between reflect_v and eye_v
        // a negative number means the light reflects away from the eye
        let reflect_v = (-light_v).reflect(normal_v);
        let reflect_dot_eye = Float(tuple::dot(&reflect_v, &eye_v));

        if reflect_dot_eye <= Float(0.0) {
            specular = Color::new(0.0, 0.0, 0.0); // black
        } else {
           // Compute Specular
            let factor = reflect_dot_eye.value().powf(material.shininess.value());
            specular = light_source.intensity * material.specular.value() * factor;
        }
    }
    ambient + diffuse + specular
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

        // Lighting with the eye between the light and the surface
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 0.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eye_v, &normal_v);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));

        // Lighting with the eye between the light and surface, eye offset 45 degrees
        let eye_v = vector(0.0, 2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 0.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eye_v, &normal_v);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));

        // Lighting with eye opposite surface, light offset 45 degrees
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eye_v, &normal_v);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));

        // Lighting with eye in the path of the reflection vector
        let eye_v = vector(0.0, -2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eye_v, &normal_v);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));

        // Lighting with the light behind the surface
        let eye_v = vector(0.0, 0.0, -1.0);
        let normal_v = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 0.0, 10.0), &Color::new(1.0, 1.0, 1.0));
        let result = lighting(&m, &light, &position, &eye_v, &normal_v);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
