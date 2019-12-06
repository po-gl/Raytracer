/// # Normal Perturber
/// `normal_perturber` is a module to change normals for effect

use crate::tuple::{Tuple, vector};
use crate::material::CmpPerlin;
use noise::NoiseFn;


pub struct NormalPerturber;

impl NormalPerturber {

    pub fn perturb_normal(command: String, point: &Tuple, factor: Option<f64>, perlin: Option<CmpPerlin>) -> Tuple {
        match command.as_ref() {
            "sin_y" => NormalPerturber::perturb_sin_y(point, factor.unwrap()),
            "perlin" => NormalPerturber::perlin(point, factor.unwrap(), perlin.unwrap()),
            _ => point.clone()
        }
    }

    pub fn perturb_sin_y(point: &Tuple, factor: f64) -> Tuple {
        vector(0.0, (point.y * factor).value().sin(), 0.0)
    }

    pub fn perlin(point: &Tuple, factor: f64, perlin: CmpPerlin) -> Tuple {
        let perlin_x = perlin.perlin.get([point.x.value(), point.y.value(), point.z.value()]) * factor;
        let perlin_y = perlin.perlin.get([point.x.value(), point.y.value(), point.z.value()]) * factor;
        let perlin_z = perlin.perlin.get([point.x.value(), point.y.value(), point.z.value()]) * factor;
        vector(perlin_x, perlin_y, perlin_z)
    }
}
