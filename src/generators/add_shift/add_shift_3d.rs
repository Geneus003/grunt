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

    let (crossed_point_x, crossed_point_y) = {
        let y_line_coef = 1.0 / ((180.0 - now_shift_angle_y).to_radians().tan());
        let x_line_coef = (180.0 - now_shift_angle_x).to_radians().tan();

        let x_cross = (y_line_y_pos + x_line_coef * x_line_x_pos) / (x_line_coef - y_line_coef);
        let y_cross = y_line_coef * x_cross + y_line_y_pos;
        
        (x_cross.round() as i32, y_cross.round() as i32)
    };

    println!("CROSS! - {}, {}", crossed_point_x, crossed_point_y);

    let mut cords_to_mod = Vec::with_capacity(size_x * size_y / 2 + 1);
    for y in 0..size_y {
        for x in 0..size_x {
            let y_line_y_delt = ((x as f32 / new_angle_y.to_radians().tan()) * 10.0).round() / 10.0;
            let x_line_x_delt = ((y as f32 / new_angle_x.to_radians().tan()) * 10.0).round() / 10.0;

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

            // State 1 - left lower part
            // State 2 - right lower part
            // State 3 - left upper part
            // State 4 - right upper part
            let mut state = 0;
            // state += if (y as f32) < y_line_y_point { 1 } else { 3 };
            // state += if (x as f32) < x_line_x_point { 0 } else { 1 };
            state += if (y as i32) < crossed_point_y { 1 } else { 3 };
            state += if (x as i32) < crossed_point_x { 0 } else { 1 };

            if state == target_state {
                borders[0][y][x] += shift_force;
            }

            cords_to_mod.push(vec![y, x])
        }
    }

}
