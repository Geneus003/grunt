use log::{trace, error};

use crate::types::generation_params::Params3D;

pub fn create_raw_model(params: &Params3D, borders: Vec<Vec<Vec<i32>>>) -> Result<Vec<Vec<Vec<i32>>>, &'static str> {
    #[cfg(debug_assertions)]
    trace!("Starting filling model");

    let model: Vec<Vec<Vec<i32>>> = Vec::with_capacity(2);

    for y_cord in &borders {
        for x_cord in y_cord {
            for _depth in x_cord {
                
            }
        }
    }

    #[cfg(debug_assertions)]
    trace!("Model was filled succesfully");
    return Err("Nothing")
}
