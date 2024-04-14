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
    pub fn find_element_smaller(self: &Self, target: f32) -> Option<usize> {
        if self.ordered {
            let inc_order = self.axis[0] < self.axis[self.axis.len() - 1];

            let mut smaller_border = 0usize;
            let mut bigger_border = self.axis.len() - 1;

            loop {
                let now_el = (smaller_border + bigger_border) / 2;

                if self.axis[now_el] <= target {
                    if inc_order{
                        smaller_border = now_el;
                    } else {
                        bigger_border = now_el;
                    }
                } else {
                    if inc_order{
                        bigger_border = now_el;
                    } else {
                        smaller_border = now_el;
                    }
                }

                if smaller_border + 1 == bigger_border {
                    if inc_order{
                        if target < self.axis[smaller_border] {
                            return None
                        } else if target >= self.axis[bigger_border] {
                            return Some(bigger_border)
                        }
                        return Some(smaller_border);
                    } else {
                        if target < self.axis[bigger_border] {
                            return None
                        } else if target >= self.axis[smaller_border] {
                            return Some(smaller_border)
                        }
                        return Some(bigger_border);
                    }
                }
            }
        } else {
            for (num, el) in self.axis.iter().enumerate() {
                if *el <= target {
                    return Some(num)
                }
            }
            None
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
