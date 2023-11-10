pub mod generators_params;
pub mod layers_borders;

pub fn generate_3d(params: generators_params::Params3D) -> () {
    layers_borders::create_layers_borders_3D(&params);
    return ()
}
