/// # Normal Perturber
/// `normal_perturber` is a module to change normals for effect

use crate::tuple::{Tuple, vector};


pub struct NormalPerturber;

impl NormalPerturber {

    pub fn perturb_normal(command: String, point: &Tuple, factor: Option<f64>) -> Tuple {
        match command.as_ref() {
            "sin_y" => NormalPerturber::perturb_sin_y(point, factor.unwrap()),
            _ => point.clone()
        }
    }

    pub fn perturb_sin_y(point: &Tuple, factor: f64) -> Tuple {
        vector(0.0, (point.y * factor).value().sin(), 0.0)
    }
}
