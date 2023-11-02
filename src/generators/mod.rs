pub mod create_base_axises;
pub mod generators_params;

use create_base_axises::create_base_layers_distibution;

pub fn generate_3d(params: generators_params::Params3D) -> () {
    create_base_layers_distibution(&params);
    println!("{:?}", params);
    return ()
}
