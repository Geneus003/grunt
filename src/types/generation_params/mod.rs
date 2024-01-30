use crate::types::{Axis, LayersDist, LayersBorder, LayersFill};

mod params3d;

#[derive(Debug)]
pub struct Params3D {
    // Axes parameters 
    x_ax: Axis,
    y_ax: Axis,
    // Base layers parameters
    layers_dist: LayersDist,
    // How to modify layers
    layers_border: LayersBorder,
    // How to fill layers
    layers_fill: LayersFill
}
