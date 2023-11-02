use crate::types::Axis;

#[derive(Debug)]
pub struct Params3D {
    model_name: String,
    x_ax: Axis,
    y_ax: Axis,
}

impl Params3D {
    pub fn new() -> Params3D {
        Params3D {
            model_name: String::from("3D Model"),
            x_ax: Axis::new(),
            y_ax: Axis::new(),
        }
    }
}

impl Params3D {
    pub fn set_model_name(self: &mut Self, name: String) {
        self.model_name = name;
    }

    pub fn get_model_name(self: &Self) -> &String {
        &self.model_name
    }

    pub fn set_x_axis(self: &mut Self, axis: Axis) {
        self.x_ax = axis
    }
    
    pub fn get_x_axis(self: Self) -> Axis {
        self.x_ax
    }

    pub fn set_y_axis(self: &mut Self, axis: Axis) {
        self.y_ax = axis
    }

    pub fn get_y_axis(self: Self) -> Axis {
        self.y_ax
    }
}
