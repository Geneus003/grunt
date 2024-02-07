use log::trace;
use crate::types::generation_params::Params3D;

pub fn create_full_model(_params: &Params3D, borders: Vec<Vec<Vec<i32>>>, _fill_values: Vec<Vec<i32>>) -> Result<(Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<usize>>>), &'static str> {
    #[cfg(debug_assertions)]
    trace!("Starting filling model");

    let mut max_elem = 0;
    for y_cord in &borders {
        for x_cord in y_cord {
            for depth in x_cord {
                if *depth > max_elem {
                    max_elem = *depth;
                } 
            }
        } 
    }

    let layers_count = borders.len();
    let y_size = borders[0].len();
    let x_size = borders[0][0].len();

    let mut model: Vec<Vec<Vec<i32>>> = Vec::with_capacity(max_elem.try_into().expect("Capacity overfill"));
    let mut model_mask: Vec<Vec<Vec<usize>>> = Vec::with_capacity(max_elem.try_into().expect("Capacity overfill"));

    let mut now_next_depth: Vec<Vec<i32>> = borders[0].clone();
    let mut now_next_index: Vec<Vec<usize>> = vec![vec![0; x_size]; y_size];

    for depth in 0..max_elem+1 {

        let mut now_depth: Vec<Vec<i32>> = Vec::with_capacity(y_size);
        let mut now_depth_mask: Vec<Vec<usize>> = Vec::with_capacity(y_size);

        for y_cord in 0..y_size {
            
            let mut now_y_line: Vec<i32> = Vec::with_capacity(x_size);
            let mut now_y_line_mask: Vec<usize> = Vec::with_capacity(x_size);

            for x_cord in 0..x_size {

                if depth > now_next_depth[y_cord][x_cord] && now_next_index[y_cord][x_cord] < layers_count - 1 {
                    now_next_index[y_cord][x_cord] += 1;
                    now_next_depth[y_cord][x_cord] = borders[now_next_index[y_cord][x_cord]][y_cord][x_cord]
                }

                let _now_index = now_next_index[y_cord][x_cord];

                now_y_line.push(now_next_depth[y_cord][x_cord]);

                now_y_line_mask.push(now_next_index[y_cord][x_cord]);
            }

            now_depth.push(now_y_line);
            now_depth_mask.push(now_y_line_mask);
        }

        model.push(now_depth);
        model_mask.push(now_depth_mask);
    }

    #[cfg(debug_assertions)]
    trace!("Model was filled succesfully");

    return Ok((model, model_mask))
}
