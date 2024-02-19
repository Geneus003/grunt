use crate::types::{Axis, LayersDist, LayersBorder, LayersFill};
use crate::types::shifts::Shift3D;
use crate::types::generation_params::Params3D;

impl Params3D {
    pub fn new() -> Params3D {
        Params3D {
            x_ax: Axis::new(),
            y_ax: Axis::new(),
            layers_dist: LayersDist::new(),
            layers_border: LayersBorder::new(),
            layers_fill: LayersFill::new(),
            shifts: Vec::new(),
            model_needed: true,
            mask_needed: true,
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

    pub fn add_shift(self: &mut Self, shift: Shift3D) {
        self.shifts.push(shift)
    }

    pub fn shifts(self: &Self) -> &Vec<Shift3D> {
        &self.shifts
    }

    pub fn set_model_needed(self: &mut Self, is_full_model: bool) {
        self.model_needed = is_full_model;
    }

    pub fn model_needed(self: &Self) -> bool {
        self.model_needed
    }

    pub fn set_mask_needed(self: &mut Self, is_mask: bool) {
        self.mask_needed = is_mask;
    }

    pub fn mask_needed(self: &Self) -> bool {
        self.mask_needed
    }
}
