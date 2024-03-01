use generators::generate_3d;
use types::{Axis, LayersDist, LayersBorder, LayersFill};
use types::shifts::Shift3D;

pub mod generators;
pub mod types;

fn main() {
    #[cfg(debug_assertions)]
    env_logger::init();

    fn _test_function(x_cord: usize, y_cord: usize, _layers_num: usize, _z_value: i32) -> i32 {
        if x_cord <= 6 && y_cord >= 4 && y_cord <= 6 {
            return -6
        }
        0
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

    params.set_x_axis(Axis::generate_axis(1.0, 10.0, None));
    params.set_y_axis(Axis::generate_axis(1.0, 10.0, None));

    params.set_layers_dist(LayersDist::create_from_vec([4, 6, 5].to_vec()).unwrap_or(LayersDist::new()));

    let mut borders = LayersBorder::new();
    let _ = borders.set_border_deviation(0.0);
    let _ = borders.set_border_max_step(None);
    // borders.set_border_mod_func(Some(_test_function));
    params.set_layers_border(borders);

    let mut fill = LayersFill::new();
    fill.set_is_preset_odreder(true);
    params.set_layers_fill(fill);

    let mut shift = Shift3D::new();
    shift.set_pos_y(5.0);
    shift.set_angle_y(90.0).unwrap();
    shift.set_pos_x(5.0);
    shift.set_angle_x(90.0).unwrap();
    shift.set_main_region(2).unwrap();
    shift.set_shift_force(-2);
    shift.set_angle_z(60.0).unwrap();
    shift.set_shift_force(3);
    params.add_shift(shift);

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
    //
}

#[cfg(test)]
mod tests;
