use rand::Rng;
use log::trace;

use crate::types::generation_params::Params3D;

pub mod filling_model_3d;

pub fn fill_3d(
    params: &Params3D,
    borders: &Vec<Vec<Vec<i32>>>
) -> (Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<usize>>>, Vec<Vec<i32>>) {
    #[cfg(debug_assertions)]
    trace!("Preparing for model fill");

    let mut fill_values = params.layers_fill().values_preset().clone();
    let deviation = params.layers_fill().values_deviation();
    let model_size = *params.layers_dist().get_layers_dist().last().unwrap_or(&0);

    // Recalculating fill_values using deviation
    for fill_value in &mut fill_values {
        if fill_value.len() == 2 {
            continue; }
        if deviation.is_some() {
            let mut deviation = deviation.unwrap();
            if deviation < 1.0 {
                deviation = deviation * model_size as f32;
            }
            let deviation = deviation as i32;
            fill_value.push(fill_value[0] + deviation);
            fill_value[0] = if deviation > fill_value[0] {
                0
            } else {
                fill_value[0] - deviation
            };
        } else {
            fill_value.push(fill_value[0]);
        }
    }

    #[cfg(debug_assertions)]
    trace!("Filling values for layers were recalculated, using deviation: {:?}", fill_values);

    // Reodering and adding values to Vec for making generation after easier
    let mut new_fill_values: Vec<Vec<i32>> = Vec::with_capacity(borders.len());

    if params.layers_fill().is_preset_ordered() {
        for i in 0..borders.len() {
            new_fill_values.push(fill_values[i % fill_values.len()].clone())
        }
    } else {
        let mut ran = rand::thread_rng();

        let mut last_index = ran.gen_range(0..fill_values.len());
        let mut new_index = ran.gen_range(0..fill_values.len());

        while new_fill_values.len() != borders.len() {
            if last_index != new_index {
                new_fill_values.push(fill_values[new_index].clone());
                last_index = new_index;
            }
            new_index = ran.gen_range(0..fill_values.len());
        }
    }

    #[cfg(debug_assertions)]
    trace!("Filling values for model: {:?}", new_fill_values);

    let (model, model_mask) = if params.mask_needed() && params.model_needed() {
        filling_model_3d::create_full_model_with_mask(borders, &new_fill_values)
    } else if params.model_needed() {
        (filling_model_3d::create_full_model_without_mask(borders, &new_fill_values), Vec::new())
    } else {
        (Vec::new(), filling_model_3d::create_only_mask(borders))
    };

    (model, model_mask, fill_values)
}
