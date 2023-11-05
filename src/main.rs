pub mod generators;
pub mod types;

use generators::generate_3d;
use generators::generators_params::Params3D;

fn main() {
    let mut params = Params3D::new();

    for _ in 0..1000 {
        let target = 220u32;
        let temp = types::DefaultLayersDist::generate_layers_dist(3, 70, 90, Some(target));

        match temp {
            Some(values) => {
                let sum: u32 = values.iter().sum();
                if sum == target {
                    continue;
                }
                print!("Err: ");
                for i in values {
                    print!("{} ", i);
                }
                println!();
            },
            None => println!("NONE"),
        }
    }
    params.set_model_name(String::from("My model"));

    generate_3d(params);
}

#[cfg(test)]
pub mod tests;
