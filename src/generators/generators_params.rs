use crate::types::Axis;
use crate::types::LayersDist;
use crate::types::LayersBorder;

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

impl Params3D {
    pub fn new() -> Params3D {
        Params3D {
            model_name: String::from("3D Model"),
            // Axes parameters 
            x_ax: Axis::new(),
            y_ax: Axis::new(),
            // Base layers parameters
            layers_dist: LayersDist::new(),
            // How to modify layers
            layers_border: LayersBorder::new()
        }
    }
}

impl Params3D {
    pub fn set_model_name(self: &mut Self, name: String) {
        self.model_name = name;
    }

    pub fn model_name(self: &Self) -> &String {
        &self.model_name
    }

    pub fn set_x_axis(self: &mut Self, axis: Axis) {
        self.x_ax = axis;
    }
    
    pub fn x_axis(self: &Self) -> &Axis {
        &self.x_ax
    }

    pub fn set_y_axis(self: &mut Self, axis: Axis) {
        self.y_ax = axis;
    }

    pub fn y_axis(self: &Self) -> &Axis {
        &self.y_ax
    }

    pub fn set_default_dayers_dist(self: &mut Self, layers: LayersDist) {
        self.layers_dist = layers;
    }

    pub fn default_layers_dist(self: &Self) -> &LayersDist {
        &self.layers_dist
    }

    pub fn set_layers_border(self: &mut Self, layers_border: LayersBorder) {
        self.layers_border = layers_border
    }

    pub fn layers_border(self: &Self) -> &LayersBorder {
        &self.layers_border
    }
}
