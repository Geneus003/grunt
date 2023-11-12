pub mod generators;
pub mod types;

use generators::generate_3d;
use generators::generators_params::Params3D;

use types::Axis;

fn main() {
    let mut params = Params3D::new();

    params.set_model_name(String::from("My model"));
    params.set_x_axis(Axis::generate_axis(0.0, 10.0, None));
    params.set_y_axis(Axis::generate_axis(0.0, 10.0, None));
    generate_3d(params);
}

#[cfg(test)]
pub mod tests;
