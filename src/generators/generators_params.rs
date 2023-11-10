use crate::types::Axis;
use crate::types::DefaultLayersDist;

#[derive(Debug)]
pub struct Params3D {
    model_name: String,
    // Axes parameters 
    x_ax: Axis,
    y_ax: Axis,
    // Base layers parameters
    layers_dist: DefaultLayersDist,
    // How to modify layers
    border_divation: f32,
    border_mod_func: Option<fn()>, // FIX THIS REWRITE IT FOR MULTIPLE ARGUMENTS!!
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
            border_divation: 0.0,
            border_mod_func: None,
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

    pub fn set_border_divation(self: &mut Self, border_divation: f32) -> Result<(), &'static str> {
        if border_divation < 0.0 {
            return Err("Border divation must be bigger than zero.")
        }
        self.border_divation = border_divation;
        return Ok(())
    }

    pub fn border_divation(self: &Self) -> f32 {
        return self.border_divation
    }

    pub fn set_border_mod_func(self: &mut Self, mod_func: Option<fn()>) {
        self.border_mod_func = mod_func;
    }
}
