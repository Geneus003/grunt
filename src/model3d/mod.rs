#[cfg(debug_assertions)]
use log::trace;

use crate::types::generation_params::Params3D;

pub mod borders3d;
pub mod shifts3d;
pub mod fill3d;
pub mod export;
pub mod convert_data;

pub fn generate_model(params: Params3D) -> Result<Model3D, &'static str> {
    #[cfg(debug_assertions)]
    trace!("Starting generating 3D model");

    let mut borders = borders3d::create_layers_borders_3d(&params)?;

    if params.shifts().len() > 0 {
        #[cfg(debug_assertions)]
        trace!("{} shifts found", params.shifts().len());

        for i in 0..params.shifts().len() {
            shifts3d::add_shift_3d::add_shift(&params, &mut borders, i);
        }
    }

    let (model, model_mask, fill_values) = if params.model_needed() || params.mask_needed() {
        fill3d::fill(&params, &borders)
    } else {
        (Vec::new(), Vec::new(), Vec::new())
    };

    let final_model = Model3D::new(model, model_mask, borders, fill_values, params);

    Ok(final_model)
}

#[derive(Debug, Clone)]
pub struct Model3D {
    model: Vec<Vec<Vec<i32>>>,
    model_mask: Vec<Vec<Vec<u8>>>,
    borders: Vec<Vec<Vec<i32>>>,
    layers_filling_values: Vec<Vec<i32>>,
    params: Params3D,
}

impl Model3D {
    pub fn new(
        model: Vec<Vec<Vec<i32>>>,
        model_mask: Vec<Vec<Vec<u8>>>,
        borders: Vec<Vec<Vec<i32>>>,
        layers_filling_values: Vec<Vec<i32>>,
        params: Params3D) -> Model3D {
        Model3D {
            model,
            model_mask,
            borders,
            layers_filling_values,
            params,
        }
    } 
}

impl Model3D {
    pub fn model(self: &Self) -> &Vec<Vec<Vec<i32>>> {
        &self.model
    }

    pub fn model_mask(self: &Self) -> &Vec<Vec<Vec<u8>>> {
        &self.model_mask
    }
    
    pub fn borders(self: &Self) -> &Vec<Vec<Vec<i32>>> {
        &self.borders
    }

    pub fn layers_filling_values(self: &Self) -> &Vec<Vec<i32>> {
        &self.layers_filling_values
    }

    pub fn params(self: &Self) -> &Params3D {
        &self.params
    }
}
