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
pub struct LayersDist {
    layers_num: u32,
    max_layer_size: i32,
    min_layer_size: i32,
    layers_sum: i32,
    layers_dist: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct LayersBorder {
    border_divation: f32,
    border_mod_func: Option<fn()>, // FIX THIS, REWRITE IT FOR MULTIPLE ARGUMENTS!!
    border_type: String,
    border_max_step: Option<i32>,
    layers_same_divation: bool,
}
