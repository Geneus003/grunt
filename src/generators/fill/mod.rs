use rand::distributions::{Distribution, Uniform};

#[cfg(debug_assertions)]
use log::trace;

use crate::types::generation_params::Params3D;

pub mod filling_model_3d;

#[derive(Debug, Clone)]
pub enum GenerationTypes {
    GenerationRange(Uniform<i32>),
    GenerationExact(i32),
}

pub fn fill_3d(
    params: &Params3D,
    borders: &Vec<Vec<Vec<i32>>>
) -> (Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<u8>>>, Vec<Vec<i32>>) {
    #[cfg(debug_assertions)]
    trace!("Preparing for model fill");

    let fill_values = params.layers_fill().values_preset().clone();
    let deviation = params.layers_fill().values_deviation();
    let model_size = *params.layers_dist().get_layers_dist().last().unwrap_or(&0);

    let mut fill_values_gen_type: Vec<GenerationTypes> = Vec::with_capacity(fill_values.len());

    for fill_value in &fill_values {
        match fill_value.len() {
            1 => fill_values_gen_type.push(
                match deviation {
                    Some(dev_cof) => {
                        let deviation = if dev_cof < 1.0 {
                            (dev_cof * model_size as f32) as i32
                        } else {
                            dev_cof as i32
                        };

                        let first_dev = if deviation > fill_value[0] {
                            0
                        } else {
                            fill_value[0] - deviation
                        };

                        GenerationTypes::GenerationRange(Uniform::from(first_dev..deviation+fill_value[0]))
                    },
                    None => GenerationTypes::GenerationExact(fill_value[0])
                }
            ),
            2 => fill_values_gen_type.push(GenerationTypes::GenerationRange(
                Uniform::from(fill_value[0]..fill_value[1]+1)
            )),
            _ => unreachable!()
        }
    }

    #[cfg(debug_assertions)]
    trace!("Filling values for layers were recalculated, using deviation: {:?}", fill_values);

    // Reodering and adding values to Vec for making generation after easier

    let mut new_fill_values: Vec<GenerationTypes> = Vec::with_capacity(fill_values.len());
    if params.layers_fill().is_preset_ordered() {
        for i in 0..borders.len() {
            new_fill_values.push(fill_values_gen_type[i % fill_values.len()].clone())
        }
    } else {
        let mut rng = rand::thread_rng();

        let possible_index = Uniform::from(0..fill_values_gen_type.len());

        let mut last_index = possible_index.sample(&mut rng); 
        let mut new_index = possible_index.sample(&mut rng); 
        
        if fill_values_gen_type.len() > 1 {
            while new_fill_values.len() != borders.len() {
                if last_index != new_index {
                    new_fill_values.push(fill_values_gen_type[new_index].clone());
                    last_index = new_index;
                }
                new_index = possible_index.sample(&mut rng);
            }
        } else {
            for _ in 0..borders.len() {
                new_fill_values.push(fill_values_gen_type[0].clone())
            }
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
