use crate::types::Axis;

impl Axis {
    pub fn new() -> Axis {
        Axis {
            start: 0.0,
            end: 100.0,
            step: Some(1.0),
            axis: Axis::calculate_axis(0.0, 100.0, 1.0),
        }
    }

    pub fn generate_axis<T: Into<f32>>(start: T, end: T, step: Option<T>) -> Axis {
        let (start, end) = (start.into(), end.into());
        let step = match step {
            Some(step) => Some(step.into()),
            None => None,
        };

        Axis {
            start,
            end,
            step,
            axis: Axis::calculate_axis(start, end, step.unwrap_or(1.0))
        }
    }

    pub fn create_from_vec(axis: Vec<f32>) -> Result<Axis, &'static str> {
        if axis.is_empty() {
            return Err("Axis must contain at least one element");
        }
        Ok(Axis{
            start: axis[0],
            end: axis[axis.len()],
            step: None,
            axis,
        })
    }

    fn calculate_axis(start: f32, end: f32, step: f32) -> Vec<f32> {
        (0..(((end-start)/step * 1000.0).round() / 1000.0 + 1.0).floor() as i32)
            .map(|num| ((start + num as f32 * step) * 1000.0).round() / 1000.0).collect()
    }
}

impl Axis {
    pub fn get_full_axis(self: &Self) -> (f32, f32, Option<f32>, &Vec<f32>) {
        (self.start, self.end, self.step, &self.axis)
    }

    pub fn get_axis(self: &Self) -> &Vec<f32> {
        &self.axis
    }

    pub fn get_len(self: &Self) -> usize {
        self.axis.len()
    }
}
