use rand::Rng;

use super::types::{Axis, LayersDist, LayersBorder};
use super::types::generation_params::Params3D;
use super::generators::border_creation::create_layers_borders_3d;

#[test]
fn random_gen_layers_borders_tests(){
    let gen_count = 5000;
    let mut rng = rand::thread_rng();

    for _ in 0..gen_count {
        let mut params = Params3D::new();

        params.set_x_axis(Axis::generate_axis(0i16, rng.gen_range(0..25) as i16, None));
        params.set_y_axis(Axis::generate_axis(0i16, rng.gen_range(0..25) as i16, None));

        let layer_num: u32 = rng.gen_range(2..20);
        let layer_min: i32 = rng.gen_range(1..20);
        let layer_max: i32 = rng.gen_range(layer_min..layer_min+20);
        let layer_sum: Option<i32> = Some(rng.gen_range(layer_min*layer_num as i32..layer_max*layer_num as i32 + 1));

        params.set_layers_dist(LayersDist::generate_from_params(layer_num, layer_min, layer_max, layer_sum).
            expect("Impossible to generate, recheck test"));

        let mut borders = LayersBorder::new();
        let _ = borders.set_border_deviation(rng.gen_range(0..100) as f32 / 10.0).expect("Error");
        let _ = borders.set_border_max_step(Some(rng.gen_range(0..100)));
        params.set_layers_border(borders);

        let res = create_layers_borders_3d(&params);
        assert_eq!(res.is_ok(), true);
    }
}