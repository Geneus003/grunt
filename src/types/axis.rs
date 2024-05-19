use numtoa::NumToA;

use crate::types::Axis;
use crate::types::AxisExportType;

impl Default for Axis {
    fn default() -> Axis {
        Axis::new()
    }
}

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
        let step = step.map(|step| step.into());

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
    pub fn find_element_smaller(&self, target: f32) -> Option<usize> {
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
                } else if inc_order {
                    bigger_border = now_el;
                } else {
                    smaller_border = now_el;
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

    pub fn convert_to_perp_ax(pos: f32, angle: f32) {
        let new_angle = ((180.0 - angle) * 1000.0).round() / 1000.0;

    }
}

impl Axis {
    fn export_num_ax(&self, result: &mut String) {
        let mut buf = [0u8; 20];
        for i in 0..self.axis.len() {
            *result += i.numtoa_str(10, &mut buf);
            *result += ",";
        }
        *result += self.axis.len().numtoa_str(10, &mut buf)
    }

    fn export_scale_ax(&self, result: &mut String, scale_value: f32) {
        for i in self.axis[..self.axis.len() - 1].iter() {
            *result += &(i * scale_value).to_string();
            *result += ",";
        }
        *result += &(self.axis[self.axis.len() - 1] * scale_value).to_string();
    }

    fn export_self_ax(&self, result: &mut String) {
        for i in self.axis[..self.axis.len() - 1].iter() {
            *result += &i.to_string();
            *result += ",";
        }
        *result += &self.axis[self.axis.len() - 1].to_string();
    }

    pub fn export_axis(&self, export_type: &AxisExportType, result: &mut String) {
        match export_type {
            AxisExportType::AsNum => self.export_num_ax(result),
            AxisExportType::AsSelf => self.export_self_ax(result),
            AxisExportType::Scale(scale_param) => self.export_scale_ax(result, *scale_param),
            // I know, i know, its bad but it's kinda unvirsal
            AxisExportType::CustomAxis(ax) => Axis::create_from_vec(ax.to_vec()).unwrap().export_self_ax(result),
        }
    }
}

impl Axis {
    pub fn get_full_axis_data(&self) -> (f32, f32, Option<f32>, &Vec<f32>) {
        (self.start, self.end, self.step, &self.axis)
    }

    pub fn get_axis(&self) -> &Vec<f32> {
        &self.axis
    }

    pub fn get_axis_len(&self) -> usize {
        self.axis.len()
    }

    pub fn ordered(&self) -> bool {
        self.ordered
    }
}
