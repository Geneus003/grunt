pub mod default_layers_dist;
pub mod axis;
pub mod layers_borders_gen;

#[derive(Debug, Clone)]
pub struct Axis {
    start: f32,
    end: f32,
    step: Option<f32>,
    axis: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct DefaultLayersDist {
    layers_num: u32,
    max_layer_size: u32,
    min_layer_size: u32,
    layers_sum: u32,
    layers_dist: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct LayersBorderGen {
    border_divation: f32,
    border_mod_func: Option<fn()>, // FIX THIS, REWRITE IT FOR MULTIPLE ARGUMENTS!!
    border_type: String,
    border_max_step: Option<u32>,
    layers_same_divation: bool,
}
