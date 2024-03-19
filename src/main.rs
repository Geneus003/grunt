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

    params.set_x_axis(Axis::generate_axis(1.0, 100.0, None));
    params.set_y_axis(Axis::generate_axis(1.0, 100.0, None));

    params.set_layers_dist(LayersDist::create_from_vec([20, 30, 20].to_vec()).unwrap_or(LayersDist::new()));

    let mut borders = LayersBorder::new();
    borders.set_border_deviation(5.0).unwrap();
    borders.set_border_max_step(Some(1));
    // borders.set_border_mod_func(Some(_test_function));
    params.set_layers_border(borders);

    let mut fill = LayersFill::new();
    fill.set_is_preset_odreder(true);
    params.set_layers_fill(fill);

    let mut shift = Shift3D::new();
    shift.set_pos_y(50.0);
    shift.set_angle_y(130.0).unwrap();
    shift.set_pos_x(-30.0);
    shift.set_angle_x(135.0).unwrap();
    shift.set_angle_z(85.0).unwrap();
    shift.set_shift_force(10);
    shift.set_shift_type(types::shifts::ShiftTypes::InnerDescent);
    shift.set_main_region(1).unwrap();
    params.add_shift(shift);

    let mut shift = Shift3D::new();
    shift.set_pos_y(20.0);
    shift.set_angle_y(134.0).unwrap();
    shift.set_pos_x(-1.0);
    shift.set_angle_x(135.0).unwrap();
    shift.set_angle_z(80.0).unwrap();
    shift.set_shift_force(10);
    shift.set_shift_type(types::shifts::ShiftTypes::InnerLift);
    shift.set_main_region(1).unwrap();
    params.add_shift(shift);

    let mut shift = Shift3D::new();
    shift.set_pos_y(80.0);
    shift.set_angle_y(110.0).unwrap();
    shift.set_pos_x(40.0);
    shift.set_angle_x(90.0).unwrap();
    shift.set_angle_z(90.0).unwrap();
    shift.set_shift_force(8);
    shift.set_shift_type(types::shifts::ShiftTypes::InnerLift);
    shift.set_main_region(3).unwrap();
    params.add_shift(shift);


    println!("{:?}", params);
    let model = generate_3d(params).unwrap();

    use std::time::Instant;
    let now = Instant::now();

    model.export_model_num("my_model", true, true, true, true).unwrap();

    println!("{:?}", model.get_by_num(0, 0));

    let elapsed = now.elapsed();

    if !(cfg!(test)) {
        println!("Elapsed export: {:.2?}", elapsed);
    }
}

#[cfg(test)]
mod tests;
