use log::trace;

use crate::types::generation_params::Params3D;

pub mod border_creation;
pub mod border_3d;

pub fn generate_3d(params: Params3D) -> Result<(), &'static str> {
    #[cfg(debug_assertions)]
    trace!("Starting generating 3D model");

    let gen_result = border_creation::create_layers_borders_3d(&params)?;

    if !(cfg!(test)) {
        for i in gen_result {
            println!("\n\n");
            for j in i {
                println!("{:?}", j)
            }
        }
    }

    return Ok(())
}
