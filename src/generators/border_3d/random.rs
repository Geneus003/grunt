use crate::Params3D;

pub fn random_layer_creation_3d(params: &Params3D, layers: &mut Vec<Vec<u32>>, now_layer: usize) -> () {
    for i in layers.iter_mut() {
        for j in i.iter_mut() {
            *j = params.default_layers_dist().get_data()[now_layer];
        }
    }
}
