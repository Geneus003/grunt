use crate::model3d::Model3D;
use crate::model2d::Model2D;

impl Model3D {
    pub fn get_by_num(&self, x: usize, y: usize) -> Result<Vec<i32>, &'static str> {
        if self.model.is_empty() { return Err("Model doesn't exists in object") };
        if self.model.len() < x || self.model[0].len() < y { return Err("X or Y out of bounds") };

        Ok(self.model[x][y].clone())
    }

    pub fn to_model_2d_by_angle(&self, pos_x: f32, angle: f32, resolution: usize) -> Result<Model2D, &'static str> {
        let angle = (angle * 1000.0).round() / 1000.0;
        let is_acute = angle <= 90.0;

        if angle <= 0.0 || angle >= 180.0 {
            return Err("Angle must be between 0.0 and 180.0 degrees")
        }

        let x_ax_obj = self.params.x_axis();
        let y_ax_obj = self.params.y_axis();
        let x_ax = self.params.x_axis().get_axis();
        let pos_y = self.params.y_axis().get_axis()[0];

        let end_x = if is_acute {
            x_ax[0]
        } else {
            x_ax[x_ax.len()-1]
        };
        
        let (delt_y, delt_x) = if is_acute {
            if pos_x < end_x {
                return Err("pos_x must be bigger than start of axis with acute angle");
            }
            let delt_x = pos_x - end_x;
            (angle.to_radians().tan() * delt_x, delt_x)
        } else {
            if pos_x > end_x {
                return Err("pos_x must be smaller than end of axis with obtuse angle");
            }
            let delt_x = end_x - pos_x;
            ((-angle).to_radians().tan() * delt_x, delt_x)
        };

        let (mut now_x, mut now_y) = (pos_x, pos_y);
        let (delt_x, delt_y) = (delt_x / resolution as f32, delt_y / resolution as f32);

        let mut nums_x: Vec<usize> = Vec::with_capacity(resolution);
        let mut nums_y: Vec<usize> = Vec::with_capacity(resolution);

        println!("Delts: {delt_y}, {delt_y}");

        for _ in 0..resolution {
            println!("{now_x}, {now_y}");
            nums_x.push(x_ax_obj.find_element_smaller(now_x).unwrap());
            nums_y.push(y_ax_obj.find_element_smaller(now_y).unwrap());

            now_x = if is_acute {
                ((now_x - delt_x) * 1000.0).round() / 1000.0
            } else {
                ((now_x + delt_x) * 1000.0).round() / 1000.0
            };
            now_y = ((now_y + delt_y) * 1000.0).round() / 1000.0;
        }

        self.form_2d_by_nums(nums_x, nums_y)
    }

    pub fn form_2d_by_nums(&self, nums_x: Vec<usize>, nums_y: Vec<usize>) -> Result<Model2D, &'static str> {
        if nums_x.len() != nums_y.len() {
            return Err("Vectors cords_x and cords_y must be with same size")
        }

        let source_model = &self.model;
        let source_model_mask = &self.model_mask;

        // Borders format is Z->Y->X, using borders because model || mask can be empty
        let borders_model_size_z = self.borders.len();
        let source_model_size_y = self.borders[0].len();
        let source_model_size_x = self.borders[0][0].len();

        let model_ex: bool = !self.model.is_empty();
        let mask_ex: bool = !self.model_mask.is_empty();

        let mut borders: Vec<Vec<i32>> = Vec::with_capacity(nums_x.len());
        let mut model: Vec<Vec<i32>> = Vec::with_capacity(if model_ex {nums_x.len()} else {0});
        let mut model_mask: Vec<Vec<u8>> = Vec::with_capacity(if mask_ex {nums_x.len()} else {0});

        println!("{:?}, {:?}", nums_x, nums_y);

        for (num, x_num) in nums_x.iter().enumerate() {
            let y_num = nums_y[num];

            if y_num >= source_model_size_y || *x_num >= source_model_size_x {
                return Err("Invalid coodinates: it must be smaller than model size")
            }

            let mut borders_temp_vec: Vec<i32> = Vec::with_capacity(borders_model_size_z);
            for layer in &self.borders {
                borders_temp_vec.push(layer[y_num][*x_num]);
            }
            borders.push(borders_temp_vec);

            if model_ex{
                model.push(source_model[*x_num][y_num].clone());
            }

            if mask_ex {
                model_mask.push(source_model_mask[*x_num][y_num].clone());
            }
        }

        Ok(Model2D::new(
            model,
            model_mask,
            borders
        ))
    }
}
