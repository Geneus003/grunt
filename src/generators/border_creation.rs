use crate::generators::generators_params::Params3D;

use crate::generators::border_3d::random_gen::random_layer_creation_3d;

pub fn create_layers_borders_3d(params: &Params3D) -> Option<Vec<Vec<Vec<i32>>>> {
    let layers_count = params.default_layers_dist().get_len();
    let x_size = params.x_axis().get_len();
    let y_size = params.y_axis().get_len();

    let mut layers_borders = vec![vec![vec![0i32; x_size]; y_size]; layers_count];

    for i in 0..layers_count {
        let res = random_layer_creation_3d(params, &mut layers_borders[i], i);
        match res {
            Err(err_text) => {
                println!("{err_text}");
                return None
            },
            _ => ()
        }
    }

	return Some(layers_borders)
}
