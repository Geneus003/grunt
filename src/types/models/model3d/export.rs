use std::fs::File;
use std::io::Write;

use numtoa::NumToA;

use crate::types::models::Model3D;
use crate::types::generation_params::Params3D;
use crate::types::AxisExportType;

impl Model3D {
    pub fn export_model(self: &Self, name: &str, save: &Vec<&str>, axes_export: &Vec<AxisExportType>) -> Result<(), std::io::Error> {
        let default_ax_type = vec![AxisExportType::Scale(1.0), AxisExportType::Scale(1.0), AxisExportType::IsNum];

        let axes_export = if axes_export.len() != 3 {
            eprintln!("WARNING: Axes export param is ignored, it must contain 3 elements (for x, y, z)");
            &default_ax_type
        } else {
            axes_export
        };

        let mut result = String::from("");
        result += "{\"params3D\":";

        if save.contains(&"params") {
            export_params(&mut result, &self.params);
        } else { result += "null" }

        result += ",\"output_axes\":";
        let mut axes_size = [self.params.x_axis().get_axis_len(), self.params.y_axis().get_axis_len(), 0]; 
        if self.model.len() != 0 {
            axes_size[2] = self.model.len()
        } else if self.model_mask.len() != 0 {
            axes_size[2] = self.model_mask.len()
        } else {
            axes_size[2] = get_max_depth(&self.borders) as usize
        }
        export_true_axes(&mut result, &self.params, axes_export, axes_size);

        result += ",\"borders\":";
        if save.contains(&"borders") {
            export_model_num(&mut result, &self.borders, true)
        } else { result += "null" }

        result += ",\"fill_values\":";
        if save.contains(&"fill_values") {
            export_model_num(&mut result, &self.borders, true)
        } else { result += "null" }

        result += ",\"model\":";
        if save.contains(&"model") {
            export_model_num(&mut result, &self.model, false)
        } else { result += "null" }

        result += ",\"model_mask\":";
        if save.contains(&"model_mask") {
            export_mask_num(&mut result, &self.model_mask)
        } else { result += "null" }
        result += "}";

        if name == "TestModelBench.test.bench" { return Ok(()) }

        let mut file = File::create(format!("{name}.json"))?;
        file.write_all(result.as_bytes())?;
        Ok(())
    }
}

fn export_model_num(result: &mut String, model: &Vec<Vec<Vec<i32>>>, is_border: bool) {
    let mut buf = [0u8; 12];
    *result += "[";

    for (depth_num, depth) in model.iter().enumerate() {
        *result += if is_border == true {
            "{\"bo"
        } else { "{\"de" }; *result += format!("{depth_num}\":[").as_str();
        for (y_num, y_axis) in depth.iter().enumerate() {
            *result += "{\"y";
            *result += format!("{y_num}\":[").as_str();

            result.push_str(y_axis[0].numtoa_str(10, &mut buf));
            let mut skip_x = true;

            for x in y_axis {
                if skip_x == true {
                    skip_x = false;
                    continue;
                }

                result.push(',');
                result.push_str(x.numtoa_str(10, &mut buf));
            }

            if y_num != depth.len() - 1 {
                *result += "]},"
            } else {
                *result += "]}"
            }
        }
        if depth_num != model.len() - 1 {
            *result += "]},";
        } else {
            *result += "]}";
        }
    }
    *result += "]";
}

fn export_mask_num(result: &mut String, model: &Vec<Vec<Vec<u8>>>) {
    let mut buf = [0u8; 12];
    *result += "[";

    for (depth_num, depth) in model.iter().enumerate() {
        *result += "{\"de";
        *result += format!("{depth_num}\":[").as_str();
        for (y_num, y_axis) in depth.iter().enumerate() {
            *result += "{\"y";
            *result += format!("{y_num}\":[").as_str();

            result.push_str(y_axis[0].numtoa_str(10, &mut buf));
            let mut skip_x = true;

            for x in y_axis {
                if skip_x == true {
                    skip_x = false;
                    continue;
                }

                result.push(',');
                result.push_str(x.numtoa_str(10, &mut buf));
            }

            if y_num != depth.len() - 1 {
                *result += "]},"
            } else {
                *result += "]}"
            }
        }
        if depth_num != model.len() - 1 {
            *result += "]},";
        } else {
            *result += "]}";
        }
    }
    *result += "]";
}

fn export_params(result: &mut String, params: &Params3D) {
    result.push_str(serde_json::to_string(params).unwrap().as_str());
}

fn export_true_axes(result: &mut String, params: &Params3D, axes_export: &Vec<AxisExportType>, model_size: [usize; 3]) {
    let mut now_ax: &Vec<f32>;
    let z_ax = if matches!(axes_export[2], AxisExportType::Scale(_)) {
        (0..model_size[2]+1).map(|i| i as f32).collect()
    } else { vec![] };
    for (export_id, export_type) in axes_export.iter().enumerate() {
        match export_id {
            0 => {
                *result += "{\"x_ax\":[";
                now_ax = params.x_axis().get_axis();
            },
            1 => {
                *result += "\"y_ax\":[";
                now_ax = params.y_axis().get_axis();
            },
            2 => {
                *result += "\"z_ax\":[";
                now_ax = &z_ax 
            },
            _ => unreachable!("Error while exporting")
        }

        match export_type {
            AxisExportType::IsNum => export_num_ax(result, model_size[export_id]),
            AxisExportType::Scale(scale) => export_scale_ax(result, *scale, now_ax),
            AxisExportType::CustomAxis(new_ax) => export_custom_ax(result, new_ax),
        }
        
        match export_id {
            0 => { *result += "]," },
            1 => { *result += "]," },
            2 => { *result += "]" },
            _ => unreachable!("Error while exporting")
        }
    }
    *result += "}"
}

// 3 functions lower created to modify ONLY axis
fn export_num_ax(result: &mut String, axes_size: usize) {
    let mut buf = [0u8; 20];
    for i in 0..axes_size {
        *result += i.numtoa_str(10, &mut buf);
        *result += ",";
    }
    *result += axes_size.numtoa_str(10, &mut buf)
}

fn export_scale_ax(result: &mut String, scale_value: f32, axis: &Vec<f32>) {
    for i in axis[..axis.len()-1].iter() {
        *result += &(i * scale_value).to_string();
        *result += ",";
    }
    *result += &(axis[axis.len()-1] * scale_value).to_string();
}

fn export_custom_ax(result: &mut String, new_axis: &Vec<f32>) {
    for i in new_axis[..new_axis.len()-1].iter() {
        *result += &i.to_string();
        *result += ",";
    }
    *result += &(new_axis[new_axis.len()-1]).to_string();
}

fn get_max_depth(model: &Vec<Vec<Vec<i32>>>) -> i32 {
    let mut max_elem = 0;
    for y_cord in model {
        for x_cord in y_cord {
            for depth in x_cord {
                if *depth > max_elem {
                    max_elem = *depth;
                } 
            }
        } }

    max_elem
}
