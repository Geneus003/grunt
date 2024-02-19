use crate::types::models::Model3D;
use crate::types::generation_params::Params3D;

pub mod export;
pub mod get_data;

impl Model3D {
    pub fn new(
        model: Vec<Vec<Vec<i32>>>,
        model_mask: Vec<Vec<Vec<usize>>>,
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
        return &self.model;
    }

    pub fn model_mask(self: &Self) -> &Vec<Vec<Vec<usize>>> {
        return &self.model_mask;
    }
    
    pub fn borders(self: &Self) -> &Vec<Vec<Vec<i32>>> {
        return &self.borders
    }

    pub fn layers_filling_values(self: &Self) -> &Vec<Vec<i32>> {
        return &self.layers_filling_values;
    }

    pub fn params(self: &Self) -> &Params3D {
        return &self.params
    }
}
