/// # material
/// `material` is a module to represent the kinds of materials we could have in our scene

use crate::float::Float;
use super::color::Color;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub specular: Float,
    pub shininess: Float,
}

impl Material {
    pub fn new() -> Material {
        Material {color: Color::new(1.0, 1.0, 1.0),
                  ambient: Float(0.1),
                  diffuse: Float(0.9),
                  specular: Float(0.9),
                  shininess: Float(200.0)}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_creation() {
        let m = Material::new();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }
}