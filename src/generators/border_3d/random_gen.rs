use crate::Params3D;
use rand::Rng;

pub fn random_layer_creation_3d(params: &Params3D, layer: &mut Vec<Vec<u32>>, now_layer_id: usize) -> () {
    let default_value = params.default_layers_dist().get_data()[now_layer_id];
    let mut rng = rand::thread_rng();
    let max_step = params.layers_border().border_max_step();
    let max_step_value = max_step.unwrap_or(0);

    const MAX_DEPTH: usize = 50;
    let upper_limit: u32 = if params.layers_border().border_divation() >= 1.0 {
        default_value + params.layers_border().border_divation() as u32
    } else {
        let model_size_value = *params.default_layers_dist().get_data().last().unwrap_or(&0);
        default_value + (params.layers_border().border_divation() * model_size_value as f32) as u32
    };
    let lower_limit: u32 = if params.layers_border().border_divation() >= 1.0 {
        default_value.checked_sub(params.layers_border().border_divation() as u32).unwrap_or(0)
    } else {
        let model_size_value = *params.default_layers_dist().get_data().last().unwrap_or(&0);
        default_value.checked_sub((params.layers_border().border_divation() * model_size_value as f32) as u32).unwrap_or(0)
    };

    for layer_line in 0..layer.len() {
        for layer_el in 0..layer[layer_line].len() {
            if max_step.is_none() {
                layer[layer_line][layer_el] = rng.gen_range(lower_limit..upper_limit);
                continue;
            }

            let mut now_element_ref = &layer[layer_line][layer_el];
            let mut now_line = layer_line;
            let mut reserved_el = layer_el;

            for i in 0..MAX_DEPTH {
                let upper_value = if now_line != 0 {
                    Some(layer[now_line-1][reserved_el])
                } else {
                    None
                };

                let left_value = if reserved_el != 0 {
                    Some(layer[now_line][reserved_el-1])
                } else {
                    None
                };

                if upper_value.is_some() && left_value.is_some() {
                    let upper_value = upper_value.unwrap();
                    let left_value = left_value.unwrap();

                    let (up_lim, down_lim) = if upper_value > left_value {
                        (left_value + max_step, upper_value - max_step) 
                    } else {
                        (upper_value + max_step, left_value - max_step) 
                    };

                    if up_lim >= down_lim {
                        now_element_ref = rng.gen_range(down_lim..up_lim)
                    }
                }
            }
        }
    }
}
