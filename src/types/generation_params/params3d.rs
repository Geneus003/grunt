use crate::types::generation_params::Params3D;
use crate::types::LayersDist;
use crate::types::LayersBorder;
use crate::types::Axis;

impl Params3D {
    pub fn new() -> Params3D {
        Params3D {
            model_name: String::from("3D Model"),
            x_ax: Axis::new(),
            y_ax: Axis::new(),
            layers_dist: LayersDist::new(),
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

    pub fn set_layers_dist(self: &mut Self, layers: LayersDist) {
        self.layers_dist = layers;
    }

    pub fn layers_dist(self: &Self) -> &LayersDist {
        &self.layers_dist
    }

    pub fn set_layers_border(self: &mut Self, layers_border: LayersBorder) {
        self.layers_border = layers_border
    }

    pub fn layers_border(self: &Self) -> &LayersBorder {
        &self.layers_border
    }
}
