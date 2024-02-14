use log::trace;

use crate::types::generation_params::Params3D;
use crate::types::models::Model3D;

pub mod border_creation;
pub mod border_3d;
pub mod fill;
pub mod add_slice;

pub fn generate_3d(params: Params3D) -> Result<Model3D, &'static str> {
    #[cfg(debug_assertions)]
    trace!("Starting generating 3D model");

    let borders = border_creation::create_layers_borders_3d(&params)?;

    if params.slices().len() > 0 {
        #[cfg(debug_assertions)]
        trace!("{} slices found", params.slices().len());

        add_slice::add_slice_3d::add_3d(&params, borders);
        unimplemented!("ADD SLICES SUPPORT")
    }

    use std::time::Instant;
    let now = Instant::now();

    let (model, model_mask, fill_values) = if params.model_needed() || params.mask_needed() {
        fill::fill_3d(&params, &borders).expect("here")
    } else {
        (Vec::new(), Vec::new(), Vec::new())
    };

    let final_model = Model3D::new(model, model_mask, borders, fill_values, params);

    let elapsed = now.elapsed();

    if !(cfg!(test)) {
        println!("Elapsed: {:.2?}", elapsed);
    }

    return Ok(final_model)
}

// borders print
// #[cfg(debug_assertions)]
// if !(cfg!(test)) {
//     for i in &borders {
//         println!("\n\n");
//         for j in i {
//             println!("{:?}", j)
//         }
//     }
// }
