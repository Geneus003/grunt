use log::trace;

use crate::types::generation_params::Params3D;

pub mod border_creation;
pub mod border_3d;
pub mod fill;
pub mod add_slice;

pub fn generate_3d(params: Params3D) -> Result<(), &'static str> {
    #[cfg(debug_assertions)]
    trace!("Starting generating 3D model");

    let borders = border_creation::create_layers_borders_3d(&params)?;

    if !(cfg!(test)) {
        for i in &borders {
            println!("\n\n");
            for j in i {
                println!("{:?}", j)
            }
        }
    }

    if params.slices().len() > 0 {
        #[cfg(debug_assertions)]
        trace!("{} slices found", params.slices().len());

        add_slice::add_slice_3d::add_3d(&params, borders);
        unimplemented!("ADD SLICES SUPPORT")
    }

    use std::time::Instant;
    let now = Instant::now();

    let (_model, _model_mask) = fill::fill_3d(&params, borders).expect("here");
    //println!("{:?}", model);

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    return Ok(())
}
