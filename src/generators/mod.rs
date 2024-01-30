use log::trace;

use crate::types::generation_params::Params3D;

pub mod border_creation;
pub mod border_3d;
pub mod fill;

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

    let model = fill::fill_3d::create_raw_model(&params, borders);

    return Ok(())
}
