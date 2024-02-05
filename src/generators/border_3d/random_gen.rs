use rand::Rng;

use crate::types::generation_params::Params3D;

const REGENERATE_TRIES:i32 = 500;

pub fn random_layer_creation_3d(params: &Params3D, layer: &mut Vec<Vec<i32>>, now_layer_id: usize) -> Result<(), &'static str> {
    let default_value = params.layers_dist().get_layers_dist_summed()[now_layer_id];
    let max_step = params.layers_border().border_max_step();
    let mut failed_generation_count = 0;
    let mut rng = rand::thread_rng();

    // Generating limits of generation (max and min heights of layer)
    let upper_limit: i32 = if params.layers_border().border_deviation() >= 1.0 {
        default_value + params.layers_border().border_deviation() as i32
    } else {
        let model_size_value = *params.layers_dist().get_layers_dist().last().unwrap_or(&0);
        default_value + (params.layers_border().border_deviation() * model_size_value as f32) as i32
    };

    let mut lower_limit: i32 = if params.layers_border().border_deviation() >= 1.0 {
        default_value - (params.layers_border().border_deviation() as i32)
    } else {
        let model_size_value = *params.layers_dist().get_layers_dist().last().unwrap_or(&0);
        default_value - (params.layers_border().border_deviation() * model_size_value as f32) as i32
    };
    
    if lower_limit < 0 { lower_limit = 0; }

    if max_step.is_none() || upper_limit == lower_limit {
        for layer_line in 0..layer.len() {
            for layer_el in 0..layer[layer_line].len() {
                layer[layer_line][layer_el] = rng.gen_range(lower_limit..upper_limit+1);
            }
        }
    }
    let max_step = max_step.unwrap();

    for layer_line in 0..layer.len() {
        for layer_el in 0..layer[layer_line].len() {
            if layer_line == 0 && layer_el == 0 {
                layer[layer_line][layer_el] = rng.gen_range(lower_limit..upper_limit+1);
                continue;
            }

            let now_line = layer_line;
            let mut is_solved = false;

            while !(is_solved || failed_generation_count >= REGENERATE_TRIES)  {
                for i in 0..layer_el+1 {
                    let now_el = layer_el - i;

                    let upper_el = if now_line != 0 {
                        Some(layer[layer_line-1][now_el])
                    } else {
                        None
                    };

                    let left_el = if now_el != 0 {
                        Some(layer[layer_line][now_el-1])
                    } else {
                        None
                    };

                    let right_el = if now_el != layer[now_line].len() && i != 0 {
                        Some(layer[layer_line][now_el+1])
                    } else {
                        None
                    };

                    let (max_limit, min_limit) = get_limits(&upper_el, &right_el, &left_el, upper_limit, lower_limit, max_step);

                    if max_limit >= min_limit {
                        layer[now_line][now_el] = rng.gen_range(min_limit..max_limit+1);
                        is_solved = true;
                        break;
                    }

                    if upper_el.is_some() {
                        if right_el.is_none() {
                            layer[now_line][now_el] = rng.gen_range(upper_el.unwrap()-max_step..upper_el.unwrap()+max_step+1)
                        } else {
                            let right_el = right_el.unwrap();
                            let upper_el = upper_el.unwrap();
                            let mut min_v = upper_el - max_step;
                            let mut max_v = right_el + max_step;

                            if right_el > upper_el {
                                min_v = right_el - max_step;
                                max_v = upper_el + max_step;
                            } 

                            if min_v > max_v {
                                failed_generation_count += 1;
                                is_solved = false;
                                break;
                            }

                            layer[now_line][now_el] = rng.gen_range(min_v..max_v+1)
                        }
                    } else if right_el.is_some() {
                        if left_el.is_some() {
                            if left_el.unwrap() > right_el.unwrap() {
                                layer[now_line][now_el] = rng.gen_range(right_el.unwrap()..right_el.unwrap()+max_step+1);
                            } else {
                                layer[now_line][now_el] = rng.gen_range(right_el.unwrap()-max_step..right_el.unwrap()+1);
                            }
                        } else {
                            layer[now_line][now_el] = rng.gen_range(right_el.unwrap()-max_step..right_el.unwrap()+max_step+1)
                        }
                    } else { break; }
                }
            }

            if !is_solved {
                return Err("Could not find solution")
            }
        }
    }
    Ok(())
}

// This limits represents range of possile solutions for this element (layer_el-i).
// To do this block finds min and max values between 3 neighbours and applying
// max_step on them.
fn get_limits(upper_el: &Option<i32>, right_el: &Option<i32>, left_el: &Option<i32>, up_limit: i32, lo_limit: i32, ms: i32) -> (i32, i32) {
    let (mut t_max, mut t_min) = (0, i32::MAX);
    if upper_el.is_some() {
        (t_max, t_min) = (upper_el.unwrap(), upper_el.unwrap())
    }
    if right_el.is_some() {
        if t_max < right_el.unwrap() {
            t_max = right_el.unwrap()
        }
        if t_min > right_el.unwrap() {
            t_min = right_el.unwrap()
        }
    }
    if left_el.is_some() {
        if t_max < left_el.unwrap() {
            t_max = left_el.unwrap()
        }
        if t_min > left_el.unwrap() {
            t_min = left_el.unwrap()
        }
    }

    (t_min, t_max) = (t_max - ms, t_min + ms);

    if t_min < lo_limit {
        t_min = lo_limit
    }
    if t_max > up_limit {
        t_max = up_limit
    }
    (t_max, t_min)
}
