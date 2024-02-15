use std::fs::File;
use std::io::Write;

use numtoa::NumToA;

use crate::types::models::Model3D;
use crate::types::generation_params::Params3D;

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

impl Model3D {
    pub fn export_model(self: &Self, name: &str, _save_model: bool, _save_mask: bool, _save_borders: bool) {
        let mut result = String::from("{\"model\":[");
        let mut buf = [0u8; 20];
        for depth in &self.model {
            result += "{\"depth\"}:[";
            for y_axis in depth {
                result += "{\"y_line\":[";
                for x in y_axis {
                    result.push_str(x.numtoa_str(10, &mut buf));
                    result.push(',');
                }
                result += "]},"
            }
            result += "]},";
        }

        let mut file = File::create("model.json").expect("model");
        file.write_all(result.as_bytes()).expect("gdsgsdg");

    }
}
