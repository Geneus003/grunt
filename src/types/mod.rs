pub mod default_layers_dist;
pub mod axis;
pub mod layers_borders_gen;
pub mod generation_params;
pub mod layers_filling_gen;

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
    layers_dist_summed: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct LayersBorder {
    border_deviation: f32,
    border_mod_func: Option<fn()>, // FIX THIS, REWRITE IT FOR MULTIPLE ARGUMENTS!!
    border_type: String,
    border_max_step: Option<i32>,
    layers_same_deviation: bool,
}

#[derive(Debug, Clone)]
pub struct LayersFill {
    values_preset: Vec<Vec<i32>>,
    is_preset_ordered: bool,
    values_deviation: Option<f32>,
    values_smooth: Option<u32>,
    values_offset: Option<u32>,
}
