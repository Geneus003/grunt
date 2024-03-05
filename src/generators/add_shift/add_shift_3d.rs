use log::trace;

use crate::types::generation_params::Params3D;
use crate::types::shifts::ShiftTypes;

pub fn add_3d(params: &Params3D, borders: &mut Vec<Vec<Vec<i32>>>, shift_num: usize) {
    let now_shift = params.shifts()[shift_num].clone();

    let target_state = now_shift.main_region();
    let shift_force = now_shift.shift_force();
    let shift_type = now_shift.shift_type();
    let now_shift_angle_z = now_shift.angle_z();

    let (crossed_point_x, crossed_point_y) = {
        let y_line_coef = 1.0 / ((180.0 - now_shift.angle_y()).to_radians().tan());
        let x_line_coef = (180.0 - now_shift.angle_x()).to_radians().tan();

        let x_cross = (now_shift.pos_y() + x_line_coef * now_shift.pos_x()) / (x_line_coef - y_line_coef);
        let y_cross = y_line_coef * x_cross + now_shift.pos_y();
        
        (x_cross.round() as i32, y_cross.round() as i32)
    };
    
    #[cfg(debug_assertions)]
    trace!("Crossing point for shift -> x: {}, y: {}", crossed_point_x, crossed_point_y);


    for y in 0..borders[0].len() {
        for x in 0..borders[0][0].len() {
            // State 1 - left lower part
            // State 2 - right lower part
            // State 3 - left upper part
            // State 4 - right upper part
            let mut state = 0;

            state += if (y as i32) < crossed_point_y { 1 } else { 3 };
            state += if (x as i32) < crossed_point_x { 0 } else { 1 };

            match shift_type {
                ShiftTypes::InnerLift | ShiftTypes::InnerDescent => if state != target_state { continue; },
                ShiftTypes::OuterLift | ShiftTypes::OuterDescent => if state == target_state { continue; },
            }

            for z in 0..borders.len() {
                let now_border = &mut borders[z][y][x];

                let surface_lengts =
                    (((x as i32 - crossed_point_x).pow(2) + (y as i32 - crossed_point_y).pow(2)) as f32).sqrt();

                let shift_z = (now_shift_angle_z.to_radians().tan() * surface_lengts).round() as i32;

                if now_shift_angle_z < 90.0 && shift_z < *now_border { break; }

                match shift_type {
                    ShiftTypes::InnerLift | ShiftTypes::OuterLift => {
                        *now_border -= shift_force;
                    }
                    ShiftTypes::InnerDescent | ShiftTypes::OuterDescent => {
                        *now_border += shift_force;
                    }
                }
            }
        }
    }

}
