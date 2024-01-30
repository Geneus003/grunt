use generators::generate_3d;
use types::{Axis, LayersDist, LayersBorder, LayersFill};

pub mod generators;
pub mod types;

fn main() {
    env_logger::init();

    let mut params = types::generation_params::Params3D::new();

    params.set_x_axis(Axis::generate_axis(0.0, 10.0, None));
    params.set_y_axis(Axis::generate_axis(0.0, 10.0, None));

    params.set_layers_dist(LayersDist::create_from_vec([70, 80, 90].to_vec()).unwrap_or(LayersDist::new()));

    let mut borders = LayersBorder::new();
    let _ = borders.set_border_divation(10.0);
    let _ = borders.set_border_max_step(Some(5));
    params.set_layers_border(borders);

    let fill = LayersFill::new();
    params.set_layers_fill(fill);

    println!("{:?}", params);
    let _ = generate_3d(params);
}

#[cfg(test)]
pub mod tests;
