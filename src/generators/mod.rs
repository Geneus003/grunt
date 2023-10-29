pub mod create_base_axises;
pub mod generators_params;

use create_base_axises::create_axis_3d;

pub fn generate_3d(params: generators_params::Params3D) -> () {
    create_axis_3d(&params);
    println!("{:?}", params);
    return ()
}
