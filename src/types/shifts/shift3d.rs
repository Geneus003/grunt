use crate::types::shifts::{Shift3D, ShiftTypes};

impl Shift3D {
    pub fn new() -> Shift3D {
        Shift3D {
            pos_x: 5.0,
            pos_y: 5.0,
            angle_x: 90.0,
            angle_y: 90.0,
            main_region: 2, // can be 1, 2, 3 or 4, means part of surface that is splitted by two lines
            angle_z: 90.0,
            shift_force: 20,
            shift_type: ShiftTypes::InnerDescent,
        }
    }
}

impl Shift3D {
    pub fn set_pos_x(self: &mut Self, pos_x: f32) {
        self.pos_x = pos_x
    }

    pub fn pos_x(self: &Self) -> f32 {
        self.pos_x
    }

    pub fn set_pos_y(self: &mut Self, pos_y: f32) {
        self.pos_y = pos_y
    }

    pub fn pos_y(self: &Self) -> f32 {
        self.pos_y
    }

    pub fn set_angle_x(self: &mut Self, angle_x: f32) -> Result<(), &'static str> {
        if angle_x < 0.0 || angle_x > 180.0 {
            return Err("Angle should be between 0.0 and 180.0");
        }
        self.angle_x = angle_x;
        Ok(())
    }

    pub fn angle_x(self: &Self) -> f32 {
        self.angle_x
    }

    pub fn set_angle_y(self: &mut Self, angle_y: f32) -> Result<(), &'static str> {
        if angle_y < 0.0 || angle_y > 180.0 {
            return Err("Angle should be between 0.0 and 180.0");
        }
        self.angle_y = angle_y;
        Ok(())
    }

    pub fn angle_y(self: &Self) -> f32 {
        self.angle_y
    }

    pub fn set_main_region(self: &mut Self, region: i32) -> Result<(), &'static str> {
        if region <= 0 || region > 4 {
            return Err("Region can be only between 1 and 4, where 1 is upper left part, and 4 - lower right")
        }
        self.main_region = region;
        Ok(())
    }

    pub fn main_region(self: &Self) -> i32 {
        self.main_region
    }

    pub fn set_angle_z(self: &mut Self, angle_z: f32) -> Result<(), &'static str> {
        if angle_z < 0.0 || angle_z > 180.0 {
            return Err("Angle_z should be between 0 and 180.0");
        }
        self.angle_z = angle_z;
        Ok(())
    }

    pub fn angle_z(self: &Self) -> f32 {
        self.angle_z
    }

    pub fn set_shift_force(self: &mut Self, shift_force: i32) {
        self.shift_force = shift_force
    }

    pub fn shift_force(self: &Self) -> i32 {
        self.shift_force
    }

    pub fn set_shift_type(self: &mut Self, shift_type: ShiftTypes) {
        self.shift_type = shift_type
    }

    pub fn shift_type(self: &Self) -> ShiftTypes {
        self.shift_type.clone()
    }
}
