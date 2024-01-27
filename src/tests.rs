use rand::Rng;

use super::*;

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

    let ax = types::Axis::generate_axis(0.2, -0.2, Some(-0.2));
    assert_eq!(ax.get_axis().clone(), vec![0.2, 0.0, -0.2]);

    let ax = types::Axis::generate_axis(0.2, -1.2, None);
    assert_eq!(ax.get_axis().clone(), vec![0.2, -0.8]);

    let ax = types::Axis::generate_axis(0.2, -0.3, None);
    assert_eq!(ax.get_axis().clone(), vec![0.2]);

    let ax = types::Axis::generate_axis(0.5, 2.0, None);
    assert_eq!(ax.get_axis().clone(), vec![0.5, 1.5]);

    let ax = types::Axis::generate_axis(1i16, 2i16, None);
    assert_eq!(ax.get_axis().clone(), vec![1.0, 2.0]);

    let ax = types::Axis::generate_axis(1i16, 5i16, Some(2i16));
    assert_eq!(ax.get_axis().clone(), vec![1.0, 3.0, 5.0]);

    let ax = types::Axis::generate_axis(1i16, 6i16, Some(2i16));
    assert_eq!(ax.get_axis().clone(), vec![1.0, 3.0, 5.0]);

    let ax = types::Axis::generate_axis(2i16, -2i16, None);
    assert_eq!(ax.get_axis().clone(), vec![2.0, 1.0, 0.0, -1.0, -2.0]);
}

#[test]
fn gen_layers_dist_tests() {
    let gen_count = 50000;
    let mut rng = rand::thread_rng();
    
    let layers_num: Vec<u32> = (0..gen_count).map(|_| rng.gen_range(2..20)).collect();
    let layers_min: Vec<i32> = (0..gen_count).map(|_| rng.gen_range(1..20)).collect();
    let layers_max: Vec<i32> = (0..gen_count).map(|i| rng.gen_range(layers_min[i]..40)).collect();
    let layers_sum: Vec<Option<i32>> = (0..gen_count)
        .map(|i| Some(rng.gen_range(layers_min[i]*layers_num[i] as i32..layers_max[i]*layers_num[i] as i32 + 1)))
        .collect();

    let mut answers: Vec<Result<Vec<i32>, &'static str>> = vec![];

    for i in 0..gen_count {
        answers.push(types::LayersDist::generate_layers_dist_vec(
            layers_num[i],
            layers_min[i],
            layers_max[i],
            layers_sum[i])
        );
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

#[test]
fn random_gen_layers_borders_tests(){
    let gen_count = 5000;
    let mut rng = rand::thread_rng();

    for _ in 0..gen_count {
        let mut params = types::generation_params::Params3D::new();

        params.set_x_axis(Axis::generate_axis(0i16, rng.gen_range(0..25) as i16, None));
        params.set_y_axis(Axis::generate_axis(0i16, rng.gen_range(0..25) as i16, None));

        let layer_num: u32 = rng.gen_range(2..20);
        let layer_min: i32 = rng.gen_range(1..20);
        let layer_max: i32 = rng.gen_range(layer_min..layer_min+20);
        let layer_sum: Option<i32> = Some(rng.gen_range(layer_min*layer_num as i32..layer_max*layer_num as i32 + 1));

        params.set_layers_dist(LayersDist::generate_from_params(layer_num, layer_min, layer_max, layer_sum).
            expect("Impossible to generate, recheck test"));

        let mut borders = LayersBorder::new();
        let _ = borders.set_border_divation(rng.gen_range(0..100) as f32 / 10.0).expect("Error");
        let _ = borders.set_border_max_step(Some(rng.gen_range(0..100)));
        params.set_layers_border(borders);

        let res = generators::generate_3d(params);
        assert_eq!(res.is_ok(), true);
    }
}
