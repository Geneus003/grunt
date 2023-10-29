pub mod generators;
pub mod types;

use generators::generate_3d;
use generators::generators_params::Params3D;

fn main() {
    let mut params = Params3D::new();

    params.set_model_name(String::from("My model"));

    generate_3d(params);
}
