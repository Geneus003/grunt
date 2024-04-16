use serde::{Deserialize, Serialize};

pub mod axis;
pub mod layers_borders_gen;
pub mod layers_filling_gen;
pub mod default_layers_dist;

pub mod shifts;
pub mod generation_params;

pub enum AxisExportType {
    IsNum,
    Scale(f32),
    CustomAxis(Vec<f32>),
}

pub type BorderModFuncParams = fn(usize, usize, usize, i32) -> i32;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Axis {
    start: f32,
    end: f32,
    step: Option<f32>,
    axis: Vec<f32>,
    ordered: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LayersDist {
    layers_num: u8,
    max_layer_size: i32,
    min_layer_size: i32,
    layers_sum: i32,
    layers_dist: Vec<i32>,
    layers_dist_summed: Vec<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LayersBorder {
    border_deviation: f32,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing)]
    border_mod_func: Option<BorderModFuncParams>, // xcord, ycord, zvalue, layer_num
    border_type: String,
    border_max_step: Option<i32>,
    border_step_prob: Option<f32>,
    layers_same_deviation: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LayersFill {
    values_preset: Vec<Vec<i32>>,
    is_preset_ordered: bool,
    values_deviation: Option<f32>,
    values_smooth: Option<u32>,
    values_offset: Option<u32>,
}
