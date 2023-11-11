use crate::generators::generators_params::Params3D;

pub fn create_layers_borders_3d(params: &Params3D ) -> () {
    let layers_count = params.default_layers_dist().get_len();
    let x_size = params.x_axis().get_len();
    let y_size = params.y_axis().get_len();

    let mut layers_borders = vec![vec![vec![0u32; x_size]; y_size]; layers_count];

    for i in 0..layers_count {
        for j in 0..y_size {
            let mut last_value = 0u32;
            for z in 0..x_size {
                layers_borders[i][j][z] = 0;
            }
        }
    }
    
	return ()
}
