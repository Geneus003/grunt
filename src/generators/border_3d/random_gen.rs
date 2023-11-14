use crate::Params3D;
use rand::Rng;

pub fn random_layer_creation_3d(params: &Params3D, layer: &mut Vec<Vec<u32>>, now_layer_id: usize) -> () {
    let default_value = params.default_layers_dist().get_data()[now_layer_id];
    let mut rng = rand::thread_rng();

    for layer_line in 0..layer.len() {
        for layer_el in 0..layer[layer_line].len() {
            generate_limits_3d(layer, layer_line, layer_el);
        }
    }
}

pub fn generate_limits_3d(layer: &mut Vec<Vec<u32>>, now_line: usize, now_el: usize) {
    let tries = 100;
    for i in 0..tries {

    }
    layer[0][0] = 0;
}
