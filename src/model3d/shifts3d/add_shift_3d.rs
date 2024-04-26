#[cfg(debug_assertions)]
use log::trace;

use crate::types::generation_params::Params3D;
use crate::types::shifts::{Shift3D, ShiftTypes};

pub fn add_shift(params: &Params3D, borders: &mut [Vec<Vec<i32>>], now_shift: &Shift3D, max_depth: &mut i32) {
    #[cfg(debug_assertions)]
    trace!("Starting generating slice");

    let now_shift_angle_y = now_shift.angle_y();
    let now_shift_angle_x = now_shift.angle_x();
    let y_line_y_pos = now_shift.pos_y();
    let x_line_x_pos = now_shift.pos_x();

    let target_state = now_shift.main_region();
    let shift_force = now_shift.shift_force();
    let shift_type = now_shift.shift_type();
    let now_shift_angle_z = now_shift.angle_z();

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

    let is_inner: bool = match shift_type {
        ShiftTypes::InnerLift | ShiftTypes::InnerDescent => true,
        ShiftTypes::OuterLift | ShiftTypes::OuterDescent => false,
    };

    let is_lift: bool = match shift_type {
        ShiftTypes::InnerLift | ShiftTypes::OuterLift => true,
        ShiftTypes::InnerDescent | ShiftTypes::OuterDescent => false,
    };

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

            if (is_inner && state != target_state) || (!is_inner && state == target_state) {
                continue;
            }

            let x_minimal_len = (*x - x_line_x_point).abs();
            let y_minimal_len = (*y - y_line_y_point).abs();

            // This block determines minimal distinance beetween point and working target state.
            let minimal_len = if !is_inner {
                let state_status = state + target_state;
                match state_status {
                    3 | 7 => x_minimal_len,
                    4 | 6 => y_minimal_len,
                    _ =>  {
                        ((*x - crossed_point_x as f32).abs().powi(2) + (*y - crossed_point_y as f32).abs().powi(2))
                            .sqrt()
                    }
                }
            } else if x_minimal_len < y_minimal_len {
                x_minimal_len
            } else {
                y_minimal_len
            };

            let mut slice_depth = ((now_shift_angle_z_tan * minimal_len).round() as i32).abs();
            if is_lift { slice_depth = *max_depth - slice_depth}

            for border in borders.iter_mut() {
                let now_border = &mut border[y_num][x_num];

                if is_lift {
                    if *now_border < slice_depth {
                        continue;
                    }
                    let mut now_shift_force = *now_border - slice_depth;
                    if now_shift_force > shift_force {
                        now_shift_force = shift_force
                    }
                    *now_border -= now_shift_force;
                } else {
                    if *now_border > slice_depth {
                        continue;
                    }
                    let mut now_shift_force = slice_depth - *now_border;
                    if now_shift_force > shift_force {
                        now_shift_force = shift_force
                    }
                    *now_border += now_shift_force;
                    if *now_border > *max_depth {
                        *max_depth = *now_border
                    }
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    trace!("Slice generation has finished");
}
