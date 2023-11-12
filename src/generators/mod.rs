pub mod generators_params;
pub mod border_creation;
pub mod border_3d;

pub fn generate_3d(params: generators_params::Params3D) -> () {
    border_creation::create_layers_borders_3d(&params);
    return ()
}
