use log::{trace, info, error};

use crate::types::generation_params::Params3D;

pub mod random_gen;

pub fn validate_layer(params: &Params3D, layer: &Vec<Vec<i32>>, now_layer_id: usize) -> Result<(), &'static str> {
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

    #[cfg(debug_assertions)]
    trace!("Layer's params: max_step-{:?}, upper_limit-{upper_limit}, lower_limit-{lower_limit}", max_step);

    let mut err_elems = 0;

    for i in 0..layer.len() {
        for j in 0..layer[i].len() {
            if !(lower_limit <= layer[i][j] && layer[i][j] <= upper_limit) {
                #[cfg(debug_assertions)]
                info!("In layer {now_layer_id} elem {i}, {j} cannot be validated");
                err_elems += 1;
            } else if max_step.is_some(){
                let max_step = max_step.unwrap();

                if i != layer.len() - 1 {
                    if (layer[i][j] - layer[i+1][j]).abs() > max_step {
                        #[cfg(debug_assertions)]
                        info!("In layer {now_layer_id} elem {i}, {j} cannot be validated");
                        err_elems += 1;
                        continue;
                    }
                }

                if j != layer[i].len() - 1 {
                    if (layer[i][j] - layer[i][j+1]).abs() > max_step {
                        #[cfg(debug_assertions)]
                        info!("In layer {now_layer_id} elem {i}, {j} cannot be validated");
                        err_elems += 1;
                    }
                }
            }
        }
    }

    if err_elems != 0 {
        error!("There are {err_elems} wrong generated elements in {now_layer_id} layer");
        return Err("Errors while model validatiion");
    }

    return Ok(());
}
