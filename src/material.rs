/// # material
/// `material` is a module to represent the kinds of materials we could have in our scene

use crate::float::Float;
use super::color::Color;
use crate::pattern::Pattern;
use noise::Perlin;

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
    pub reflective: Float,
    pub transparency: Float,
    pub refractive_index: Float,
    pub pattern: Option<Box<dyn Pattern + Send>>,
    pub normal_perturb: Option<String>,
    pub normal_perturb_factor: Option<f64>,
    pub normal_perturb_perlin: Option<CmpPerlin>,
}

impl Material {
    pub fn new() -> Material {
        Material {color: Color::new(1.0, 1.0, 1.0),
                  ambient: Float(0.1),
                  diffuse: Float(0.9),
                  specular: Float(0.9),
                  shininess: Float(200.0),
                  reflective: Float(0.0),
                  transparency: Float(0.0),
                  refractive_index: Float(1.0),
                  pattern: None, normal_perturb: None,
                  normal_perturb_factor: None, normal_perturb_perlin: None}
    }

    pub fn set_pattern(&mut self, pattern: Box<dyn Pattern + Send>) {
        self.pattern = Some(pattern)
    }

    // Common materials

    pub fn glass() -> Material {
        Material {color: Color::new(1.0, 1.0, 1.0),
            ambient: Float(0.1),
            diffuse: Float(0.1),
            specular: Float(1.0),
            shininess: Float(300.0),
            reflective: Float(0.8),
            transparency: Float(1.0),
            refractive_index: Float(1.5),
            pattern: None, normal_perturb: None,
            normal_perturb_factor: None, normal_perturb_perlin: None}
}

pub fn mirror() -> Material {
        Material {color: Color::new(0.9, 0.9, 1.0),
            ambient: Float(0.1),
            diffuse: Float(0.1),
            specular: Float(0.2),
            shininess: Float(400.0),
            reflective: Float(1.0),
            transparency: Float(0.0),
            refractive_index: Float(1.0),
            pattern: None, normal_perturb: None,
            normal_perturb_factor: None, normal_perturb_perlin: None}
}


// Common material values

    pub fn water_refractive_index() -> Float {
        Float(1.333)
    }

    pub fn glass_refractive_index() -> Float {
        Float(1.52)
    }

    pub fn diamond_refractive_index() -> Float {
        Float(2.417)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{vector, point};
    use crate::light::{Light};
    use crate::shape::sphere::Sphere;
    use crate::pattern::stripe_pattern::StripePattern;
    use crate::shape::shape_list::ShapeList;

    #[test]
    fn material_creation() {
        let m = Material::new();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    #[test]
    fn material_pattern() {
        let mut shape_list = ShapeList::new();
        let mut m = Material::new();
        m.pattern = Some(Box::new(StripePattern::new(Color::white(), Color::black())));
        m.ambient = Float(1.0);
        m.diffuse = Float(0.0);
        m.specular = Float(0.0);

        let object = Sphere::new(&mut shape_list);
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 0.0, -10.0), &Color::white());
        let c1 = Light::lighting(&m, Some(Box::new(object.clone())), None, &light, &point(0.9, 0.0, 0.0), None, &eyev, &normalv, false, None);
        let c2 = Light::lighting(&m, Some(Box::new(object.clone())), None, &light, &point(1.1, 0.0, 0.0), None, &eyev, &normalv, false, None);
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }

    #[test]
    fn material_reflective() {
        let m = Material::new();
        assert_eq!(m.reflective, 0.0);
    }

    #[test]
    fn material_refraction() {
        let m = Material::new();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}


#[derive(Debug, Clone)]
pub struct CmpPerlin {
    pub perlin: Perlin
}

// To get Perlin to work
impl PartialEq for CmpPerlin {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}
