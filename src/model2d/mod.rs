#[derive(Debug, Clone)]
pub struct Model2D {
    model: Vec<Vec<i32>>,
    model_mask: Vec<Vec<u8>>,
    borders: Vec<Vec<i32>>,
}

impl Model2D {
    pub fn new(
        model: Vec<Vec<i32>>,
        model_mask: Vec<Vec<u8>>,
        borders: Vec<Vec<i32>>) -> Model2D {
        Model2D {
            model,
            model_mask,
            borders,
        }
    } 
}

impl Model2D {
    pub fn model(&self) -> &Vec<Vec<i32>> {
        &self.model
    }

    pub fn model_mask(&self) -> &Vec<Vec<u8>> {
        &self.model_mask
    }
    
    pub fn borders(&self) -> &Vec<Vec<i32>> {
        &self.borders
    }
}
