use crate::types::Axis;

impl Axis {
    pub fn new() -> Axis {
        Axis {
            start: 0.0,
            end: 100.0,
            step: Some(1.0),
            axis: Axis::calculate_axis(0.0, 100.0, 1.0),
            ordered: true,
        }
    }

    pub fn generate_axis<T: Into<f32>>(start: T, end: T, step: Option<T>) -> Axis {
        let (start, end) = (start.into(), end.into());
        let step = match step {
            Some(step) => Some(step.into()),
            None => None,
        };

        let new_step = step.unwrap_or(if start < end {1.0} else {-1.0});

        Axis {
            start,
            end,
            step,
            axis: Axis::calculate_axis(start, end, new_step),
            ordered: true,
        }
    }

    pub fn create_from_vec(axis: Vec<f32>) -> Result<Axis, &'static str> {
        if axis.is_empty() {
            return Err("Axis must contain at least one element");
        }

        let mut always_smaller = true;
        let mut always_bigger = true;
        let mut pr_el = axis[0];

        for i in axis[1..].iter() {
            if *i < pr_el {
                always_smaller &= true;
                always_bigger &= false;
            } else if *i > pr_el {
                always_smaller &= false;
                always_bigger &= true;
            }
            pr_el = *i
        }

        Ok(Axis{
            start: axis[0],
            end: axis[axis.len() - 1],
            step: None,
            axis,
            ordered: always_bigger || always_smaller,
        })
    }

    fn calculate_axis(start: f32, end: f32, step: f32) -> Vec<f32> {
        (0..(((end-start)/step * 1000.0).round() / 1000.0 + 1.0).floor() as i32)
            .map(|num| ((start + num as f32 * step) * 1000.0).round() / 1000.0).collect()
    }
}

impl Axis {
    // Function to find id of first element which is smaller than target or equal
    pub fn find_first_element_smaller(self: &Self, target: f32) -> Option<usize> {
        let mut right_border = self.axis.len();
        let mut left_border = 0usize;

        loop {
            println!("{left_border}, {right_border}");
            let now_el = (right_border + left_border) / 2;
            if target > self.axis[now_el] {
                left_border = now_el
            } else {
                right_border = now_el
            } 

            if left_border + 1 == right_border {
                return Some(left_border)
            } else if left_border == right_border {
                if self.axis[left_border] == target {
                    return Some(left_border)
                }
                return None
            }
        }
    }
}

impl Axis {
    pub fn get_full_axis_data(self: &Self) -> (f32, f32, Option<f32>, &Vec<f32>) {
        (self.start, self.end, self.step, &self.axis)
    }

    pub fn get_axis(self: &Self) -> &Vec<f32> {
        &self.axis
    }

    pub fn get_axis_len(self: &Self) -> usize {
        self.axis.len()
    }

    pub fn ordered(self: &Self) -> bool {
        self.ordered
    }
}
