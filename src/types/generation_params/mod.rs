use crate::types::{Axis, LayersDist, LayersBorder, LayersFill};
use crate::types::slices::Slice3D;

mod params3d;

#[derive(Debug, Clone)]
pub struct Params3D {
    // Axes parameters 
    x_ax: Axis,
    y_ax: Axis,
    // Base layers parameters
    layers_dist: LayersDist,
    // How to modify layers
    layers_border: LayersBorder,
    // How to fill layers
    layers_fill: LayersFill,
    slices: Vec<Slice3D>,
    // Optional params to reduce generation time
    model_needed: bool,
    mask_needed: bool,
}
