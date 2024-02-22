use generators::generate_3d;
use types::{Axis, LayersDist, LayersBorder, LayersFill};

pub mod generators;
pub mod types;
fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    fn _test_function(x_cord: usize, y_cord: usize, layers_num: usize, _z_value: i32) -> i32 {
        if x_cord >= 4 && x_cord <= 6 && y_cord >= 4 && y_cord <= 6 && layers_num < 2 {
            return -4
        }
        return 0
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

    params.set_x_axis(Axis::generate_axis(0.0, 10.0, None));
    params.set_y_axis(Axis::generate_axis(0.0, 10.0, None));

    params.set_layers_dist(LayersDist::create_from_vec([20, 30, 50].to_vec()).unwrap_or(LayersDist::new()));

    let mut borders = LayersBorder::new();
    let _ = borders.set_border_deviation(2.0);
    let _ = borders.set_border_max_step(Some(1));
    borders.set_border_mod_func(Some(_test_function));
    params.set_layers_border(borders);

    let mut fill = LayersFill::new();
    fill.set_is_preset_odreder(true);
    params.set_layers_fill(fill);

    println!("{:?}", params);
    let model = generate_3d(params).unwrap();

    use std::time::Instant;
    let now = Instant::now();

    model.export_model_num("my_model", true, true, true).unwrap();

    println!("{:?}", model.get_by_num(0, 0));

    let elapsed = now.elapsed();

    if !(cfg!(test)) {
        println!("Elapsed export: {:.2?}", elapsed);
    }

    // if !(cfg!(test)) {
    //     for i in model.model() {
    //         println!("\n\n");
    //         for j in i {
    //             println!("{:?}", j)
    //         }
    //     }
    // }
    //
    // if !(cfg!(test)) {
    //     for i in model.model_mask() {
    //         println!("\n");
    //         for j in i {
    //             println!("{:?}", j)
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests;
