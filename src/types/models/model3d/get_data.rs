use crate::types::models::Model3D;

impl Model3D {
    pub fn get_by_num(self: &Self, x: usize, y: usize) -> Result<Vec<i32>, &'static str> {
        if self.model.is_empty() { return Err("Model doesn't exists in object") };
        if self.model[0].len() < y || self.model[0][0].len() < x { return Err("X or Y out of bounds") };

        let mut output_vec = Vec::with_capacity(self.model.len());
        for i in 0..self.model.len() {
            output_vec.push(self.model[i][x][y]);
        }
        Ok(output_vec)
    }
}
