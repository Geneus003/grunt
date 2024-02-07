pub mod model3d;

use crate::types::generation_params::Params3D;

#[derive(Debug, Clone)]
pub struct Model3D {
    model: Option<Vec<Vec<Vec<i32>>>>,
    model_mask: Option<Vec<Vec<Vec<i32>>>>,
    layers_filling_values: Vec<Vec<i32>>,
    params: Params3D,
}
