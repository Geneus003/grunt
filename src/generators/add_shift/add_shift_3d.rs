use crate::types::generation_params::Params3D;

enum CordState {
    Lower,
    Upper,
}

pub fn add_3d(params: &Params3D, borders: &Vec<Vec<Vec<i32>>>, shift_num: usize) {
    let size_y = borders[0].len();
    let size_x = borders[0][0].len();

    let now_shift = params.shifts()[shift_num].clone();

    let new_angle_y = if now_shift.angle_y() <= 90.0 {
        now_shift.angle_y()
    } else {
        180.0 - now_shift.angle_y()
    };

    let new_angle_x = if now_shift.angle_x() <= 90.0 {
        now_shift.angle_x()
    } else {
        180.0 - now_shift.angle_x()
    };

    let mut cords_to_mod = Vec::with_capacity(size_x * size_y / 2 + 1);
    for y in 0..size_y {
        for x in 0..size_x {
            let x_line_x_delt = y as f32 / new_angle_x.to_radians().tan();
            let y_line_y_delt = x as f32 / new_angle_y.to_radians().tan();

            // For x line we use x_delt
            // Now you need to compare pos_x +- x_delt with x_now and apply this to CordState

            cords_to_mod.push(vec![y, x])
        }
    }

}
