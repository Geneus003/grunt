mod params3d;
use crate::types::LayersDist;
use crate::types::LayersBorder;
use crate::types::Axis;

#[derive(Debug)]
pub struct Params3D {
    model_name: String,
    // Axes parameters 
    x_ax: Axis,
    y_ax: Axis,
    // Base layers parameters
    layers_dist: LayersDist,
    // How to modify layers
    layers_border: LayersBorder,
}
