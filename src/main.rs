pub mod generators;
pub mod types;

use generators::generate_3d;
use generators::generators_params::Params3D;

fn main() {
    let mut params = Params3D::new();

    let temp = types::DefaultLayersDist::generate_layers_dist(3, 70, 90, None);
    match temp {
        None => println!("NONE"),
        _ => println!("YEAH"),
    }

    params.set_model_name(String::from("My model"));

    generate_3d(params);
}

#[cfg(test)]
pub mod tests;
