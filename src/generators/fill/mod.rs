use crate::types::generation_params::Params3D;

pub mod filling_model_3d;

pub fn fill_3d(params: &Params3D, borders: Vec<Vec<Vec<i32>>>) -> Result<(Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<usize>>>), &'static str>  {

    // TODO: Second reorder fill_values using is_preset_ordered
    // TODO: pass it to only_fill_3d as argument

    let mut fill_values = params.layers_fill().values_preset().clone();
    let deviation = params.layers_fill().values_deviation();
    let model_size = *params.layers_dist().get_layers_dist().last().unwrap_or(&0);

    // Recalculating fill_values using deviation
    for fill_value in &mut fill_values {
        if fill_value.len() == 2 {
            continue;
        }
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

    println!("{:?}", fill_values);

    // let fill_values: Vec<Vec<i32>> = if params.layers_fill().is_preset_ordered() == true || fill_values.len() == 1{
    //     (0..layers_count).map(|i| fill_values[i % layers_count]).collect()
    // } else {
    //     let values: Vec<Vec<i32>> = Vec::with_capacity(layers_count);
    //     for i in 0..layers_count {
    //         values.push(fill_values[i].clone())
    //     }
    //     values
    // };

    return Err("fdafasdf");
}
