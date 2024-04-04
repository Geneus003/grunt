#[cfg(debug_assertions)]
use log::trace;

use crate::types::generation_params::Params3D;
use crate::types::models::Model3D;

pub mod border_creation;
pub mod border_3d;
pub mod fill;
pub mod add_shift;

pub fn generate_3d(params: Params3D) -> Result<Model3D, &'static str> {
    #[cfg(debug_assertions)]
    trace!("Starting generating 3D model");

    let mut borders = border_creation::create_layers_borders_3d(&params)?;

    if params.shifts().len() > 0 {
        #[cfg(debug_assertions)]
        trace!("{} shifts found", params.shifts().len());

        for i in 0..params.shifts().len() {
            add_shift::add_shift_3d::add_3d(&params, &mut borders, i);
        }
    }

    let (model, model_mask, fill_values) = if params.model_needed() || params.mask_needed() {
        fill::fill_3d(&params, &borders)
    } else {
        (Vec::new(), Vec::new(), Vec::new())
    };

    let final_model = Model3D::new(model, model_mask, borders, fill_values, params);

    Ok(final_model)
}
