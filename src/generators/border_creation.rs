use log::{trace, error};

use crate::types::generation_params::Params3D;
use crate::generators::border_3d::random_gen::random_layer_creation_3d;

pub fn create_layers_borders_3d(params: &Params3D) -> Option<Vec<Vec<Vec<i32>>>> {
    #[cfg(debug_assertions)]
    trace!("Starting creating 3D borders");

    let layers_count = params.layers_dist().get_layers_count();
    let x_size = params.x_axis().get_axis_len();
    let y_size = params.y_axis().get_axis_len();

    #[cfg(debug_assertions)]
    trace!("Generating with this params: layers_count-{layers_count}, x_size-{x_size}, y_size-{y_size}");

    let mut layers_borders = vec![vec![vec![0i32; x_size]; y_size]; layers_count];

    for i in 0..layers_count {
        let res = match params.layers_border().border_type().as_str() {
            "random" => random_layer_creation_3d(params, &mut layers_borders[i], i),
            _ => return None,
        };

        #[cfg(debug_assertions)]
        if let Err(err) = crate::generators::border_3d::validate_model(params, &layers_borders[i], i) {
            error!("Validating for layer {i} FAILED: {err}")
        } else {
            trace!("Validating for layer {i} completed succesfully")
        }

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
