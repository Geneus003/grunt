pub mod slice3d;

#[derive(Debug, Clone)]
pub enum ShiftTypes {
    InnerDescent,
    OuterDescent,
    InnerLift,
    OuterLift,
}

#[derive(Debug, Clone)]
pub struct Slice3D {
    pos_x: f32,
    pos_y: f32,
    angle_x: f32,
    angle_y: f32,
    main_region: i32, // can be 1, 2, 3 or 4, means part of surface that is splitted by two lines
    angle_z: f32,
    shift_force: i32,
    shift_type: ShiftTypes,
}
