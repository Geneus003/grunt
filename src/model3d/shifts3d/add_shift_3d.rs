#[cfg(debug_assertions)]
use log::trace;

use crate::types::generation_params::Params3D;
use crate::types::shifts::ShiftTypes;

pub fn add_shift(params: &Params3D, borders: &mut Vec<Vec<Vec<i32>>>, shift_num: usize) {
    #[cfg(debug_assertions)]
    trace!("Starting generating slice");

    let now_shift = params.shifts()[shift_num].clone();

    let now_shift_angle_y = now_shift.angle_y();
    let now_shift_angle_x = now_shift.angle_x();
    let y_line_y_pos = now_shift.pos_y();
    let x_line_x_pos = now_shift.pos_x();

    let target_state = now_shift.main_region();
    let shift_force = now_shift.shift_force();
    let shift_type = now_shift.shift_type();
    let now_shift_angle_z = now_shift.angle_z();

    #[cfg(debug_assertions)]
    let (crossed_point_x, crossed_point_y) = {
        let y_line_coef = 1.0 / ((180.0 - now_shift_angle_y).to_radians().tan());
        let x_line_coef = (180.0 - now_shift_angle_x).to_radians().tan();

        let x_cross = (y_line_y_pos + x_line_coef * x_line_x_pos) / (x_line_coef - y_line_coef);
        let y_cross = y_line_coef * x_cross + y_line_y_pos;
        
        (x_cross.round() as i32, y_cross.round() as i32)
    };
    
    let new_angle_y_tan = (if now_shift_angle_y <= 90.0 {
        now_shift_angle_y
    } else {
        180.0 - now_shift_angle_y
    }).to_radians().tan();

    let new_angle_x_tan = (if now_shift_angle_x <= 90.0 {
        now_shift_angle_x
    } else {
        180.0 - now_shift_angle_x
    }).to_radians().tan();

    let now_shift_angle_z_tan = now_shift_angle_z.to_radians().tan();

    #[cfg(debug_assertions)]
    trace!("Crossing point for shift -> x: {}, y: {}", crossed_point_x, crossed_point_y);

    let mut x_line_x_points: Vec<f32> = Vec::with_capacity(borders[0].len());
    for y in params.y_axis().get_axis() {
        let x_line_x_delt = ((y / new_angle_x_tan) * 10.0).round() / 10.0;
        x_line_x_points.push(if now_shift_angle_x <= 90.0 {
            ((x_line_x_pos - x_line_x_delt) * 10.0).round() / 10.0
        } else {
            ((x_line_x_pos + x_line_x_delt) * 10.0).round() / 10.0
        });
    }

    let mut y_line_y_points: Vec<f32> = Vec::with_capacity(borders[0][0].len());
    for x in params.x_axis().get_axis() {
        let y_line_y_delt = ((x / new_angle_y_tan) * 10.0).round() / 10.0;
        y_line_y_points.push(if now_shift_angle_y <= 90.0 {
            ((y_line_y_pos - y_line_y_delt) * 10.0).round() / 10.0
        } else {
            ((y_line_y_pos + y_line_y_delt) * 10.0).round() / 10.0
        });
    }

    for (y_num, y) in params.y_axis().get_axis().iter().enumerate() {
        for (x_num, x) in params.x_axis().get_axis().iter().enumerate() {
            // State 1 - left lower part
            // State 2 - right lower part
            // State 3 - left upper part
            // State 4 - right upper part
            let mut state = 0;

            let y_line_y_point = y_line_y_points[x_num];
            let x_line_x_point = x_line_x_points[y_num];

            state += if *y <= y_line_y_point { 1 } else { 3 };
            state += if *x <= x_line_x_point { 0 } else { 1 };

            match shift_type {
                ShiftTypes::InnerLift | ShiftTypes::InnerDescent => if state != target_state { continue; },
                ShiftTypes::OuterLift | ShiftTypes::OuterDescent => if state == target_state { continue; },
            }

            let minimal_len = if (*x - x_line_x_point).abs() < (*y - y_line_y_point).abs() {
                (*x - x_line_x_point).abs()
            } else {
                (*y - y_line_y_point).abs()
            };

            let slice_depth = ((now_shift_angle_z_tan * minimal_len).round() as i32).abs();

            for z in 0..borders.len() {
                let now_border = &mut borders[z][y_num][x_num];

                match shift_type {
                    ShiftTypes::InnerLift | ShiftTypes::OuterLift => {
                        if *now_border > slice_depth {
                            continue;
                        }
                        let mut now_shift_force = slice_depth - *now_border;
                        if slice_depth - *now_border > shift_force {
                            now_shift_force = shift_force
                        }
                        *now_border -= now_shift_force;
                    }
                    ShiftTypes::InnerDescent | ShiftTypes::OuterDescent => {
                        if *now_border > slice_depth {
                            continue;
                        }
                        let mut now_shift_force = slice_depth - *now_border;
                        if slice_depth - *now_border > shift_force {
                            now_shift_force = shift_force
                        }
                        *now_border += now_shift_force;
                    }
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    trace!("Slice generation has finished");
}
