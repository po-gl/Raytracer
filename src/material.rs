/// # material
/// `material` is a module to represent the kinds of materials we could have in our scene

use crate::float::Float;
use super::color::Color;
use crate::pattern::Pattern;

#[derive(Debug, PartialEq, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
    pub reflective: Float,
    pub pattern: Option<Box<dyn Pattern>>,
}

impl Material {
    pub fn new() -> Material {
        Material {color: Color::new(1.0, 1.0, 1.0),
                  ambient: Float(0.1),
                  diffuse: Float(0.9),
                  specular: Float(0.9),
                  shininess: Float(200.0),
                  reflective: Float(0.0),
                  pattern: None}
    }

    pub fn set_pattern(&mut self, pattern: Box<dyn Pattern>) {
        self.pattern = Some(pattern)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{vector, point};
    use crate::light::{Light, lighting};
    use crate::shape::sphere::Sphere;
    use crate::pattern::stripe_pattern::StripePattern;

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
        let mut m = Material::new();
        m.pattern = Some(Box::new(StripePattern::new(Color::white(), Color::black())));
        m.ambient = Float(1.0);
        m.diffuse = Float(0.0);
        m.specular = Float(0.0);

        let object = Sphere::new();
        let eyev = vector(0.0, 0.0, -1.0);
        let normalv = vector(0.0, 0.0, -1.0);
        let light = Light::point_light(&point(0.0, 0.0, -10.0), &Color::white());
        let c1 = lighting(&m, Some(Box::new(object.clone())), &light, &point(0.9, 0.0, 0.0), &eyev, &normalv, false);
        let c2 = lighting(&m, Some(Box::new(object.clone())), &light, &point(1.1, 0.0, 0.0), &eyev, &normalv, false);
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }

    #[test]
    fn material_reflective() {
        let m = Material::new();
        assert_eq!(m.reflective, 0.0);
    }
}

