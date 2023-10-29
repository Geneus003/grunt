#[derive(Debug)]
pub struct Axis {
    start: f32,
    end: f32,
    step: f32,
    axis: Vec<f32>,
}

impl Axis {
    pub fn new() -> Axis {
        Axis {
            start: 0.0,
            end: 100.0,
            step: 1.0,
            axis: Axis::calculate_axis(0.0, 100.0, 1.0),
        }
    }

    pub fn generate_axis(start: f32, end: f32, step: f32) -> Axis {
        Axis {
            start,
            end,
            step,
            axis: Axis::calculate_axis(start, end, step),
        }
    }

    fn calculate_axis(start: f32, end: f32, step: f32) -> Vec<f32> {
        (0..(((end-start)/step * 1000.0).round() / 1000.0 + 1.0).floor() as i32).map(|num| ((start + num as f32 * step) * 1000.0).round() / 1000.0).collect()
    }

    pub fn get_axis(self: &Self) -> (f32, f32, f32, &Vec<f32>) {
        (self.start, self.end, self.step, &self.axis)
    }
}
