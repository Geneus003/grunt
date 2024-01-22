use log::trace;

use crate::types::generation_params::Params3D;

pub mod border_creation;
pub mod border_3d;

pub fn generate_3d(params: Params3D) -> () {
    #[cfg(debug_assertions)]
    trace!("Starting generating 3D model");

    let gen_result = border_creation::create_layers_borders_3d(&params);

    for i in gen_result.unwrap() {
        println!("\n\n");
        for j in i {
            println!("{:?}", j)
        }
    }
    return ()
}
