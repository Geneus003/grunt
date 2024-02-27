use crate::types::generation_params::Params3D;

pub fn add_3d(params: &Params3D, borders: &mut Vec<Vec<Vec<i32>>>, shift_num: usize) {
    let size_y = borders[0].len();
    let size_x = borders[0][0].len();

    let now_shift = params.shifts()[shift_num].clone();

    let now_shift_angle_y = now_shift.angle_y();
    let now_shift_angle_x = now_shift.angle_x();
    let y_line_y_pos = now_shift.pos_y();
    let x_line_x_pos = now_shift.pos_x();

    let target_state = now_shift.main_region();
    let shift_force = now_shift.shift_force();

    let new_angle_y = if now_shift_angle_y <= 90.0 {
        now_shift_angle_y
    } else {
        180.0 - now_shift_angle_y
    };

    let new_angle_x = if now_shift_angle_x <= 90.0 {
        now_shift_angle_x
    } else {
        180.0 - now_shift_angle_x
    };

    let mut cords_to_mod = Vec::with_capacity(size_x * size_y / 2 + 1);
    for y in 0..size_y {
        for x in 0..size_x {
            let y_line_y_delt = ((x as f32 / new_angle_y.to_radians().tan()) * 1000.0).round() / 1000.0;
            let x_line_x_delt = ((y as f32 / new_angle_x.to_radians().tan()) * 1000.0).round() / 1000.0;

            let y_line_y_point = if now_shift_angle_y <= 90.0 {
                ((y_line_y_pos - y_line_y_delt) * 100.0).round() / 100.0
            } else {
                ((y_line_y_pos + y_line_y_delt) * 100.0).round() / 100.0
            };

            let x_line_x_point = if now_shift_angle_x <= 90.0 {
                ((x_line_x_pos - x_line_x_delt) * 100.0).round() / 100.0
            } else {
                ((x_line_x_pos + x_line_x_delt) * 100.0).round() / 100.0
            };

            let mut state = 0;

            state += if (y as f32) < y_line_y_point {
                1
            } else {
                3
            };

            state += if (x as f32) < x_line_x_point {
                0
            } else {
                1
            };

            if state == target_state {
                borders[0][y][x] += shift_force;
            }
            // For x line we use x_delt
            // Now you need to compare pos_x +- x_delt with x_now and apply this to CordState

            cords_to_mod.push(vec![y, x])
        }
    }

}
