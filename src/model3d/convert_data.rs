use crate::model3d::Model3D;
use crate::model2d::Model2D;

impl Model3D {
    pub fn get_by_num(&self, x: usize, y: usize) -> Result<Vec<i32>, &'static str> {
        if self.model.is_empty() { return Err("Model doesn't exists in object") };
        if self.model[0].len() < y || self.model[0][0].len() < x { return Err("X or Y out of bounds") };

        let mut output_vec = Vec::with_capacity(self.model.len());
        for i in 0..self.model.len() {
            output_vec.push(self.model[i][x][y]);
        }
        Ok(output_vec)
    }

    pub fn to_model_2d_by_angle(&self, pos_x: f32, angle: f32, resolution: usize) -> Result<(), &'static str> {
        let angle = (angle * 1000.0).round() / 1000.0;
        let is_acute = angle <= 90.0;
        let _pos_y = 0.0f32;

        if angle <= 0.0 || angle >= 180.0 {
            return Err("Angle must be between 0.0 and 180.0 degrees")
        }

        let x_ax = self.params.x_axis().get_axis();
        let _y_ax = self.params.y_axis().get_axis();

        let end_x = if is_acute {
            x_ax[0]
        } else {
            x_ax[x_ax.len()-1]
        };
        
        let (_delt_y, _delt_x) = if is_acute {
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

        let _model_2d: Vec<Vec<i32>> = Vec::with_capacity(resolution);
        let (_now_x, _now_y) = (pos_x, 0.0);
        for _ in 0..resolution {


        }

        Ok(())
    }

    pub fn form_2d_by_nums(&self, cords_x: Vec<usize>, cords_y: Vec<usize>) -> Result<Model2D, &'static str> {
        if cords_x.len() != cords_y.len() {
            return Err("Vectors cords_x and cords_y must be with same size")
        }

        let source_model = &self.model;
        let source_model_mask = &self.model_mask;
        let source_model_size_y = self.borders[0].len();
        let source_model_size_x = self.borders[0][0].len();
        let source_model_size_z = self.borders.len();

        let model_ex: bool = !self.model.is_empty();
        let mask_ex: bool = !self.model_mask.is_empty();

        let mut model: Vec<Vec<i32>> = Vec::with_capacity(if model_ex {cords_x.len()} else {0});
        let mut model_mask: Vec<Vec<u8>> = Vec::with_capacity(if mask_ex {cords_x.len()} else {0});

        for (num, x_cord) in cords_x.iter().enumerate() {
            let y_cord = cords_y[num];

            if y_cord >= source_model_size_y || *x_cord >= source_model_size_x {
                return Err("Invalid coodinates: it must be smaller than model size")
            }

            if model_ex{
                let mut temp_vec: Vec<i32> = Vec::with_capacity(source_model_size_z);
                for depth in source_model {
                    temp_vec.push(depth[y_cord][*x_cord]);
                }
                model.push(temp_vec);
            }

            if mask_ex {
                let mut temp_vec: Vec<u8> = Vec::with_capacity(source_model_size_z);
                for depth in source_model_mask {
                    temp_vec.push(depth[y_cord][*x_cord]);
                }
                model_mask.push(temp_vec);
            }

        }
        Err("Literally nothing")
    }
}
