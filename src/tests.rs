use super::*;

use rand::Rng;

#[test]
fn gen_axis_tests() {
    let ax = types::Axis::generate_axis(1.0, 3.0, Some(1.0));
    assert_eq!(ax.get_axis().clone(), vec![1.0, 2.0, 3.0]);

    let ax = types::Axis::generate_axis(0.5, 3.0, Some(1.0));
    assert_eq!(ax.get_axis().clone(), vec![0.5, 1.5, 2.5]);

    let ax = types::Axis::generate_axis(0.5, 2.0, Some(0.5));
    assert_eq!(ax.get_axis().clone(), vec![0.5, 1.0, 1.5, 2.0]);

    let ax = types::Axis::generate_axis(0.5, 2.1, Some(0.5));
    assert_eq!(ax.get_axis().clone(), vec![0.5, 1.0, 1.5, 2.0]);

    let ax = types::Axis::generate_axis(0.1, 0.3, Some(0.2));
    assert_eq!(ax.get_axis().clone(), vec![0.1, 0.3]);

    let ax = types::Axis::generate_axis(-0.2, 0.2, Some(0.2));
    assert_eq!(ax.get_axis().clone(), vec![-0.2, 0.0, 0.2]);

    let ax = types::Axis::generate_axis(-0.2, 0.2, Some(1.0));
    assert_eq!(ax.get_axis().clone(), vec![-0.2]);

    let ax = types::Axis::generate_axis(-0.2, 0.2, Some(-0.2));
    assert_eq!(ax.get_axis().clone(), vec![]);

    let ax = types::Axis::generate_axis(0.5, 2.0, None);
    assert_eq!(ax.get_axis().clone(), vec![0.5, 1.5]);

    let ax = types::Axis::generate_axis(1i16, 2i16, None);
    assert_eq!(ax.get_axis().clone(), vec![1.0, 2.0]);

    let ax = types::Axis::generate_axis(1i16, 5i16, Some(2i16));
    assert_eq!(ax.get_axis().clone(), vec![1.0, 3.0, 5.0]);

    let ax = types::Axis::generate_axis(1i16, 6i16, Some(2i16));
    assert_eq!(ax.get_axis().clone(), vec![1.0, 3.0, 5.0]);
}

#[test]
fn gen_layers_dist_tests() {
    let gen_count = 100000;
    let mut rng = rand::thread_rng();
    
    let layers_num: Vec<u32> = (0..gen_count).map(|_| rng.gen_range(2..20)).collect();
    let layers_min: Vec<u32> = (0..gen_count).map(|_| rng.gen_range(1..20)).collect();
    let layers_max: Vec<u32> = (0..gen_count).map(|i| rng.gen_range(layers_min[i]+1..40)).collect();
    let layers_sum: Vec<Option<u32>> = (0..gen_count).map(|i| Some(rng.gen_range(layers_min[i]*layers_num[i]..layers_max[i]*layers_num[i]))).collect();

    let mut answers: Vec<Result<Vec<u32>, &'static str>> = vec![];

    for i in 0..gen_count {
        answers.push(types::DefaultLayersDist::generate_layers_dist(layers_num[i], layers_min[i], layers_max[i], layers_sum[i]));
    }

    let mut errors = 0;
    'a: for (i, e)  in answers.iter().enumerate() {
        if e.is_err() { 
            errors += 1;
            continue;
        }

        let (mut new_layers_num, mut new_layers_sum) = (0, 0);

        for j in e.clone().unwrap() {
            new_layers_num += 1;
            new_layers_sum += j;
            if !(j >= layers_min[i] && j <= layers_max[i]) {
                errors += 1;
                continue 'a;
            }
        }

        if new_layers_num != layers_num[i] || new_layers_sum != layers_sum[i].unwrap() {
            errors += 1;
            continue;
        }
    }

    assert_eq!(errors, 0);
}
