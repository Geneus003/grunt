use log::trace;

use crate::types::generation_params::Params3D;
use crate::types::shifts::ShiftTypes;

pub fn add_3d(params: &Params3D, borders: &mut Vec<Vec<Vec<i32>>>, shift_num: usize) {
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

    let (crossed_point_x, crossed_point_y) = {
        let y_line_coef = 1.0 / ((180.0 - now_shift_angle_y).to_radians().tan());
        let x_line_coef = (180.0 - now_shift_angle_x).to_radians().tan();

        let x_cross = (now_shift.pos_y() + x_line_coef * now_shift.pos_x()) / (x_line_coef - y_line_coef);
        let y_cross = y_line_coef * x_cross + now_shift.pos_y();
        
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

    for y in 0..borders[0].len() {
        for x in 0..borders[0][0].len() {
            // State 1 - left lower part
            // State 2 - right lower part
            // State 3 - left upper part
            // State 4 - right upper part
            let mut state = 0;

            let y_line_y_delt = ((x as f32 / new_angle_y_tan) * 10.0).round() / 10.0;
            let x_line_x_delt = ((y as f32 / new_angle_x_tan) * 10.0).round() / 10.0;

            let y_line_y_point = if now_shift_angle_y <= 90.0 {
                ((y_line_y_pos - y_line_y_delt) * 10.0).round() / 10.0
            } else {
                ((y_line_y_pos + y_line_y_delt) * 10.0).round() / 10.0
            };

            let x_line_x_point = if now_shift_angle_x <= 90.0 {
                ((x_line_x_pos - x_line_x_delt) * 10.0).round() / 10.0
            } else {
                ((x_line_x_pos + x_line_x_delt) * 10.0).round() / 10.0
            };

            state += if (y as f32) <= y_line_y_point { 1 } else { 3 };
            state += if (x as f32) <= x_line_x_point { 0 } else { 1 };

            match shift_type {
                ShiftTypes::InnerLift | ShiftTypes::InnerDescent => if state != target_state { continue; },
                ShiftTypes::OuterLift | ShiftTypes::OuterDescent => if state == target_state { continue; },
            }

            let minimal_len = if (x as f32 - x_line_x_point).abs() < (y as f32 - y_line_y_point).abs() {
                (x as f32 - x_line_x_point).abs()
            } else {
                (y as f32 - y_line_y_point).abs()
            };

            let slice_depth = now_shift_angle_y.to_radians().tan() * minimal_len;

            let surface_lengts =
                (((x as i32 - crossed_point_x).pow(2) + (y as i32 - crossed_point_y).pow(2)) as f32).sqrt();

            for z in 0..borders.len() {
                let now_border = &mut borders[z][y][x];

                if *now_border < slice_depth.round() as i32 {
                    continue
                }

                let mut now_shift_force = shift_force ;
                if shift_force as f32 > shift_force as f32 * (minimal_len / shift_force as f32) {
                    now_shift_force = (shift_force as f32 * (minimal_len / shift_force as f32)).round() as i32
                };

                match shift_type {
                    ShiftTypes::InnerLift | ShiftTypes::OuterLift => {
                        *now_border -= now_shift_force;
                    }
                    ShiftTypes::InnerDescent | ShiftTypes::OuterDescent => {
                        *now_border += now_shift_force;
                    }
                }
            }
        }
    }

    #[cfg(debug_assertions)]
    trace!("Slice generation has finished");
}
