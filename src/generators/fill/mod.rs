pub mod filling_model_3d;

pub fn fill_3d(params: &Params3D, borders: Vec<Vec<Vec<i32>>>) -> Result<(Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<usize>>>), &'static str>  {

    // TODO: First recalculate fill_values using deviation
    // TODO: Second reorder fill_values using is_preset_ordered
    // TODO: pass it to only_fill_3d as argument

    let fill_values: Vec<Vec<i32>> = if params.layers_fill().is_preset_ordered() == true || fill_values.len() == 1{
        (0..layers_count).map(|i| fill_values[i % layers_count]).collect()
    } else {
        let values: Vec<Vec<i32>> = Vec::with_capacity(layers_count);
        for i in 0..layers_count {
            values.push(fill_values[i].clone())
        }
        values
    };

    return Err("fdafasdf");
}
