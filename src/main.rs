use generators::generate_3d;
use types::{Axis, LayersDist, LayersBorder, LayersFill};

pub mod generators;
pub mod types;

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    fn test_function(_x_cord: usize, _y_cord: usize, _z_value: usize, _layers_num: i32) -> i32 {
        return 10
    }

    let mut params = types::generation_params::Params3D::new();

    // params.set_x_axis(Axis::generate_axis(0.0, 10.0, None));
    // params.set_y_axis(Axis::generate_axis(0.0, 10.0, None));

    // params.set_layers_dist(LayersDist::create_from_vec([12, 12, 12].to_vec()).unwrap_or(LayersDist::new()));

    // let mut borders = LayersBorder::new();
    // let _ = borders.set_border_deviation(10.0);
    // let _ = borders.set_border_max_step(Some(5));
    // borders.set_border_mod_func(Some(test_function));
    // params.set_layers_border(borders);

    params.set_x_axis(Axis::generate_axis(0.0, 1.0, None));
    params.set_y_axis(Axis::generate_axis(0.0, 1.0, None));

    params.set_layers_dist(LayersDist::create_from_vec([1, 2, 1].to_vec()).unwrap_or(LayersDist::new()));

    let mut borders = LayersBorder::new();
    let _ = borders.set_border_deviation(0.0);
    let _ = borders.set_border_max_step(None);
    params.set_layers_border(borders);

    let mut fill = LayersFill::new();
    fill.set_is_preset_odreder(true);
    params.set_layers_fill(fill);

    println!("{:?}", params);
    let model = generate_3d(params).unwrap();

    if !(cfg!(test)) {
        for i in model.model() {
            println!("\n\n");
            for j in i {
                println!("{:?}", j)
            }
        }
    }

    if !(cfg!(test)) {
        for i in model.model_mask() {
            println!("\n");
            for j in i {
                println!("{:?}", j)
            }
        }
    }
}

#[cfg(test)]
pub mod tests;

// TODO: Check generation of layers dist with only one layers (it should be allowed and work)
