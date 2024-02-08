use crate::types::{Axis, LayersDist, LayersBorder, LayersFill};
use crate::types::slices::Slice3D;
use crate::types::generation_params::Params3D;

impl Params3D {
    pub fn new() -> Params3D {
        Params3D {
            x_ax: Axis::new(),
            y_ax: Axis::new(),
            layers_dist: LayersDist::new(),
            layers_border: LayersBorder::new(),
            layers_fill: LayersFill::new(),
            slices: Vec::new(),
        }
    }
}

impl Params3D {
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

    pub fn set_layers_fill(self: &mut Self, layers_fill: LayersFill) {
        self.layers_fill = layers_fill
    }

    pub fn layers_fill(self: &Self) -> &LayersFill {
        &self.layers_fill
    }

    pub fn add_slice(self: &mut Self, slice: Slice3D) {
        self.slices.push(slice)
    }

    pub fn slices(self: &Self) -> &Vec<Slice3D> {
        &self.slices
    }
}
