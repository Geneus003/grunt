use log::trace;

use crate::types::generation_params::Params3D;

pub mod random_gen;

pub fn validate_model(params: &Params3D, layer: &Vec<Vec<i32>>, now_layer_id: usize) -> Result<(), &'static str> {
    let default_value = params.layers_dist().get_layers_dist_summed()[now_layer_id];
    let max_step = params.layers_border().border_max_step();

    let upper_limit: i32 = if params.layers_border().border_divation() >= 1.0 {
        default_value + params.layers_border().border_divation() as i32
    } else {
        let model_size_value = *params.layers_dist().get_layers_dist().last().unwrap_or(&0);
        default_value + (params.layers_border().border_divation() * model_size_value as f32) as i32
    };

    let lower_limit: i32 = if params.layers_border().border_divation() >= 1.0 {
        default_value.checked_sub(params.layers_border().border_divation() as i32).unwrap_or(0)
    } else {
        let model_size_value =*params.layers_dist().get_layers_dist().last().unwrap_or(&0);
        default_value
            .checked_sub((params.layers_border().border_divation() * model_size_value as f32) as i32).unwrap_or(0)
    };

    trace!("Starting validating model");
    trace!("Model params: max_step-{:?}, upper_limit-{upper_limit}, lower_limit-{lower_limit}", max_step);

    todo!("WRITE VALIDATION");

    return Ok(())
}
