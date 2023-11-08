use crate::types::Axis;
use crate::types::DefaultLayersDist;

#[derive(Debug)]
pub struct Params3D {
    model_name: String,
    x_ax: Axis,
    y_ax: Axis,
    layers_dist: DefaultLayersDist
}

impl Params3D {
    pub fn new() -> Params3D {
        Params3D {
            model_name: String::from("3D Model"),
            // Axes parameters 
            x_ax: Axis::new(),
            y_ax: Axis::new(),
            // Base layers parameters
            layers_dist: DefaultLayersDist::new(),
            // How to modify layers
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

    pub fn set_default_dayers_dist(self: &mut Self, layers: DefaultLayersDist) {
        self.layers_dist = layers;
    }

    pub fn default_layers_dist(self: &Self) -> &DefaultLayersDist {
        &self.layers_dist
    }
}
