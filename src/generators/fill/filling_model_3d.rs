use log::trace;
use rand::Rng;

fn generate_consts(borders: &Vec<Vec<Vec<i32>>>) -> (i32, usize, usize, usize) {
    let mut max_elem = 0;
    for y_cord in borders {
        for x_cord in y_cord {
            for depth in x_cord {
                if *depth > max_elem {
                    max_elem = *depth;
                } 
            }
        } }

    (max_elem, borders.len(), borders[0].len(), borders[0][0].len())
}

pub fn create_full_model_with_mask(
    borders: &Vec<Vec<Vec<i32>>>,
    fill_values: &Vec<Vec<i32>>
) -> (Vec<Vec<Vec<i32>>>, Vec<Vec<Vec<usize>>>) {
    #[cfg(debug_assertions)]
    trace!("Starting filling model: model and mask");

    let (max_elem, layers_count, y_size, x_size) = generate_consts(borders);

    let mut model: Vec<Vec<Vec<i32>>> = Vec::with_capacity(max_elem.try_into().expect("Capacity overfill"));
    let mut model_mask: Vec<Vec<Vec<usize>>> = Vec::with_capacity(max_elem.try_into().expect("Capacity overfill"));
    let mut rng = rand::thread_rng();

    let mut now_next_depth: Vec<Vec<i32>> = borders[0].clone();
    let mut now_next_index: Vec<Vec<usize>> = vec![vec![0; x_size]; y_size];

    for depth in 0..max_elem {

        let mut now_depth: Vec<Vec<i32>> = Vec::with_capacity(y_size);
        let mut now_depth_mask: Vec<Vec<usize>> = Vec::with_capacity(y_size);

        for y_cord in 0..y_size {
            
            let mut now_y_line: Vec<i32> = Vec::with_capacity(x_size);
            let mut now_y_line_mask: Vec<usize> = Vec::with_capacity(x_size);

            for x_cord in 0..x_size {
                let mut now_index = now_next_index[y_cord][x_cord];
                if depth >= now_next_depth[y_cord][x_cord] && now_index < layers_count - 1 {
                    loop {
                        now_next_index[y_cord][x_cord] += 1;
                        now_index += 1;
                        now_next_depth[y_cord][x_cord] = borders[now_index][y_cord][x_cord];
                        if now_next_depth[y_cord][x_cord] > depth || now_index == layers_count - 1 {
                            break;
                        }
                    }
                }

                now_y_line.push(rng.gen_range(fill_values[now_index][0]..fill_values[now_index][1] + 1));
                now_y_line_mask.push(now_index);
            }

            now_depth.push(now_y_line);
            now_depth_mask.push(now_y_line_mask);
        }

        model.push(now_depth);
        model_mask.push(now_depth_mask);
    }

    #[cfg(debug_assertions)]
    trace!("Model and mask ware filled succesfully");
    
    (model, model_mask)
}

pub fn create_full_model_without_mask(
    borders: &Vec<Vec<Vec<i32>>>,
    fill_values: &Vec<Vec<i32>>
) -> Vec<Vec<Vec<i32>>> {
    #[cfg(debug_assertions)]
    trace!("Starting filling only model");

    let (max_elem, layers_count, y_size, x_size) = generate_consts(borders);

    let mut model: Vec<Vec<Vec<i32>>> = Vec::with_capacity(max_elem.try_into().expect("Capacity overfill"));
    let mut rng = rand::thread_rng();

    let mut now_next_depth: Vec<Vec<i32>> = borders[0].clone();
    let mut now_next_index: Vec<Vec<usize>> = vec![vec![0; x_size]; y_size];

    for depth in 0..max_elem {

        let mut now_depth: Vec<Vec<i32>> = Vec::with_capacity(y_size);

        for y_cord in 0..y_size {
            
            let mut now_y_line: Vec<i32> = Vec::with_capacity(x_size);

            for x_cord in 0..x_size {
                let mut now_index = now_next_index[y_cord][x_cord];
                if depth >= now_next_depth[y_cord][x_cord] && now_index < layers_count - 1 {
                    loop {
                        now_next_index[y_cord][x_cord] += 1;
                        now_index += 1;
                        now_next_depth[y_cord][x_cord] = borders[now_index][y_cord][x_cord];
                        if now_next_depth[y_cord][x_cord] > depth || now_index == layers_count - 1 {
                            break;
                        }
                    }
                }
                now_y_line.push(rng.gen_range(fill_values[now_index][0]..fill_values[now_index][1] + 1));
            }

            now_depth.push(now_y_line);
        }

        model.push(now_depth);
    }

    #[cfg(debug_assertions)]
    trace!("Model was filled succesfully");
    
    model
}

pub fn create_only_mask(
    borders: &Vec<Vec<Vec<i32>>>,
) -> Vec<Vec<Vec<usize>>> {
    #[cfg(debug_assertions)]
    trace!("Starting filling model: model and mask");

    let (max_elem, layers_count, y_size, x_size) = generate_consts(borders);

    let mut model_mask: Vec<Vec<Vec<usize>>> = Vec::with_capacity(max_elem.try_into().expect("Capacity overfill"));

    let mut now_next_depth: Vec<Vec<i32>> = borders[0].clone();
    let mut now_next_index: Vec<Vec<usize>> = vec![vec![0; x_size]; y_size];

    for depth in 0..max_elem {

        let mut now_depth_mask: Vec<Vec<usize>> = Vec::with_capacity(y_size);

        for y_cord in 0..y_size {
            
            let mut now_y_line_mask: Vec<usize> = Vec::with_capacity(x_size);

            for x_cord in 0..x_size {
                let mut now_index = now_next_index[y_cord][x_cord];
                if depth >= now_next_depth[y_cord][x_cord] && now_index < layers_count - 1 {
                    loop {
                        now_next_index[y_cord][x_cord] += 1;
                        now_index += 1;
                        now_next_depth[y_cord][x_cord] = borders[now_index][y_cord][x_cord];
                        if now_next_depth[y_cord][x_cord] > depth || now_index == layers_count - 1 {
                            break;
                        }
                    }
                }
                now_y_line_mask.push(now_index);
            }

            now_depth_mask.push(now_y_line_mask);
        }

        model_mask.push(now_depth_mask);
    }

    #[cfg(debug_assertions)]
    trace!("Model and mask ware filled succesfully");
    
    model_mask
}
