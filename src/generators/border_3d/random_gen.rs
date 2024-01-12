use crate::types::generation_params::Params3D;
use rand::Rng;

pub fn random_layer_creation_3d(params: &Params3D, layer: &mut Vec<Vec<i32>>, now_layer_id: usize) -> Result<(), &'static str> {
    let default_value = params.layers_dist().get_layers_dist_summed()[now_layer_id];
    let max_step = params.layers_border().border_max_step();
    let mut rng = rand::thread_rng();

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

    if max_step.is_none() {
        for layer_line in 0..layer.len() {
            for layer_el in 0..layer[layer_line].len() {
                layer[layer_line][layer_el] = if upper_limit == lower_limit {
                    upper_limit
                } else {
                    rng.gen_range(lower_limit..upper_limit)
                };
            }
        }
    }

    for layer_line in 0..layer.len() {
        for layer_el in 0..layer[layer_line].len() {
            if max_step.is_none() || (layer_line == 0 && layer_el == 0) {
                if upper_limit == lower_limit {
                    layer[layer_line][layer_el] = upper_limit;
                } else {
                    layer[layer_line][layer_el] = rng.gen_range(lower_limit..upper_limit);
                }
                continue;
            }

            let max_step = max_step.unwrap();

            let now_line = layer_line;

            let mut is_solved = false;

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

                let (max_limit, min_limit) = {
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

                    (t_min, t_max) = (t_max - max_step, t_min + max_step);

                    if t_min < lower_limit {
                        t_min = lower_limit
                    }
                    if t_max > upper_limit {
                        t_max = upper_limit
                    }
                    (t_max, t_min)
                };

                if max_limit >= min_limit {
                    layer[now_line][now_el] = if max_limit == min_limit {
                        max_limit
                    } else {
                        rng.gen_range(min_limit..max_limit)
                    };
                    is_solved = true;
                    break;
                }

                if upper_el.is_some() {
                    if right_el.is_none() {
                        layer[now_line][now_el] = rng.gen_range(upper_el.unwrap()-max_step..upper_el.unwrap()+max_step)
                    } else {
                        if right_el.unwrap() > upper_el.unwrap() {
                            let min_v = right_el.unwrap() - max_step;
                            let max_v = upper_el.unwrap() + max_step;
                            layer[now_line][now_el] = if min_v == max_v {
                                min_v
                            } else if max_v > min_v {
                                rng.gen_range(min_v..max_v)
                            } else {
                                break;
                            };
                        }
                    }
                } else if right_el.is_some() {
                    if left_el.is_some() {
                        if left_el.unwrap() > right_el.unwrap() {
                            layer[now_line][now_el] = rng.gen_range(right_el.unwrap()..right_el.unwrap()+max_step);
                        } else {
                            layer[now_line][now_el] = rng.gen_range(right_el.unwrap()-max_step..right_el.unwrap());
                        }
                    } else {
                        layer[now_line][now_el] = rng.gen_range(right_el.unwrap()-max_step..right_el.unwrap()+max_step)
                    }
                } else {
                    break;
                }
            }

            if !is_solved {
                return Err("Could not find solution")
            }
        }
    }

    Ok(())
}
