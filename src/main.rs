pub mod generators;
pub mod types;

use generators::generate_3d;

use types::Axis;
use types::LayersDist;

fn main() {
    let mut params = types::generation_params::Params3D::new();

    params.set_model_name(String::from("My model"));
    params.set_x_axis(Axis::generate_axis(0.0, 10.0, None));
    params.set_y_axis(Axis::generate_axis(0.0, 10.0, None));

    params.set_layers_dist(LayersDist::generate_from_params(3, 89, 90, Some(270)).unwrap_or(LayersDist::new())); //HERE PROGRAM'S ERROR FIX IT!

    let mut borders = types::LayersBorder::new();
    let _ = borders.set_border_divation(10.0);
    let _ = borders.set_border_max_step(Some(5));

    params.set_layers_border(borders);
    println!("{:?}", params);
    generate_3d(params);
}

#[cfg(test)]
pub mod tests;
