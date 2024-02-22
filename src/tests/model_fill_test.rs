use rand::Rng;

use crate::generators::fill::filling_model_3d::*;

#[test]
fn fill_model_and_mask_tests() {
    let mut rnd = rand::thread_rng();
    let mut errors = 0;

    'a: for _ in 0..10000 {
        let model_size = rnd.gen_range(1..10);
        let model_x = rnd.gen_range(1..10);
        let model_y = rnd.gen_range(1..10);

        let mut borders: Vec<Vec<Vec<i32>>> = Vec::new();

        for i in 0..model_size {
            borders.push(Vec::new());
            for j in 0..model_y {
                borders[i].push(Vec::new());
                for k in 0..model_x {
                    let pr_value = if i != 0 {
                        borders[i-1][j][k]
                    } else {
                        1
                    };

                    borders[i][j].push(rnd.gen_range(pr_value..11));
                }
            }
        }

        let filling_values = vec![vec![1, 1], vec![2, 2], vec![3, 3], vec![4, 4], vec![5, 5], vec![6, 6], vec![7, 7], vec![8, 8], vec![9, 9], vec![10, 10], vec![11, 11]];

        let (model, model_mask) = create_full_model_with_mask(&borders, &filling_values);

        let model_x = create_full_model_without_mask(&borders, &filling_values);
        let mask_x = create_only_mask(&borders);

        if model_x != model || mask_x != model_mask {
            errors += 1;
            break 'a
        }

        for i in 0..model.len() {
            for j in 0..model[0].len() {
                for k in 0..model[0][0].len() {
                    let model_value = model[i][j][k];
                    
                    if (model_value - 1) as usize != model_mask[i][j][k] { errors += 1 }

                    if model_mask[i][j][k] == model_size - 1{
                        continue;
                    } else {
                        if borders[(model_value - 1) as usize][j][k] < i as i32 {
                            errors += 1;
                            break 'a
                        }
                    }
                }
            }
        }
    }

    assert_eq!(errors, 0);
}
