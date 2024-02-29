use std::fs::File;
use std::io::Write;

use numtoa::NumToA;

use crate::types::models::Model3D;

impl Model3D {
    pub fn export_model_num(self: &Self, name: &str, save_model: bool, save_mask: bool, save_borders: bool) -> Result<(), std::io::Error> {
        let mut result = String::from("");

        result += "{\"model\":";
        if save_model == true {
            add_model_num(&mut result, &self.model, false);
        } else { result += "null"}

        result += ",\"model_mask\":";
        if save_mask  == true {
            add_mask_num(&mut result, &self.model_mask);
        } else { result += "null"}

        result += ",\"borders\":";
        if save_borders == true {
            add_model_num(&mut result, &self.borders, true);
        } else { result += "null"}
        result += "}";

        let mut file = File::create(format!("{name}.json"))?;
        file.write_all(result.as_bytes())?;
        Ok(())
    }
}

fn add_model_num(result: &mut String, model: &Vec<Vec<Vec<i32>>>, is_border: bool) {
    let mut buf = [0u8; 12];
    *result += "[";

    for (depth_num, depth) in model.iter().enumerate() {
        *result += if is_border == true {
            "{\"border_"
        } else {
            "{\"depth_"
        };

        *result += format!("{depth_num}\":[").as_str();
        for (y_num, y_axis) in depth.iter().enumerate() {
            *result += "{\"y_";
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

fn add_mask_num(result: &mut String, model: &Vec<Vec<Vec<u8>>>) {
    let mut buf = [0u8; 12];
    *result += "[";

    for (depth_num, depth) in model.iter().enumerate() {
        *result += "{\"depth_";
        *result += format!("{depth_num}\":[").as_str();
        for (y_num, y_axis) in depth.iter().enumerate() {
            *result += "{\"y_";
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
