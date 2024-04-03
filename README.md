# Grunt
Project to generate 3D raster (image, grid and so on) of surface. Project can create true-like model of ground cover at high resolution, up to 100s millions elements per model (approx. 3Gb RAM required at this res.).

Characters in line hardlimit - 120

## How to install
1) Clone repo
2) Add to your project using local cargo(path to local lib) package

## How to use

```
// Import
use grunt::generators::generate_3d;
use grunt::types::generation_params::Params3D;
use grunt::types::shifts::{Shift3D, ShiftTypes};
use grunt::types::*;

fn main() {
    // Create paramethers struct first
    let mut params = Params3D::new();

    // Generate axis or create it from vector
    params.set_x_axis(Axis::generate_axis(0.0, 5.0, None));
    params.set_y_axis(Axis::generate_axis(0.0, 5.0, None));

    // Create layers distribution - (how ideally your model be (now cummulative!!))
    params.set_layers_dist(LayersDist::create_from_vec(vec![10, 20, 10]).unwrap());

    // Create layers borders (how layers from layers distribution will be modified)
    let mut borders = LayersBorder::new();
    borders.set_border_deviation(5.0).unwrap();
    borders.set_border_max_step(Some(1));
    params.set_layers_border(borders);

    // Fill model with something
    let mut fill = LayersFill::new();
    fill.set_values_preset(vec![vec![100], vec![200], vec![300, 320]]).unwrap();
    fill.set_is_preset_odreder(true);
    params.set_layers_fill(fill);

    // Add shift to your model
    let mut shift = Shift3D::new();
    shift.set_pos_y(20.0);
    shift.set_angle_y(134.0).unwrap();
    shift.set_pos_x(-1.0);
    shift.set_angle_x(135.0).unwrap();
    shift.set_angle_z(80.0).unwrap();
    shift.set_shift_force(10);
    shift.set_shift_type(ShiftTypes::InnerLift);
    shift.set_main_region(1).unwrap();
    params.add_shift(shift);

    // Create model
    let model = generate_3d(params).expect("crashed");

    // Export model (note: axis is always exported)
    let save_state = vec!["params", "borders", "model", "model_mask"];
    let axis_export = vec![AxisExportType::IsNum, AxisExportType::Scale(2.0), AxisExportType::CustomAxis(vec![1.0, 2.0])]; // (x, y, z)
    model.export_model("my_model", &save_state, &axis_export).unwrap();
}
```

Visualize it using ```python ./misc/model_viewer.py```

## Check examples, try it

You can run and view examples using this command (use commands from directory with ```README.md```)

View available examples:
```ls examples/ | sed -e 's/\.rs//'```

Run and view example:
```cargo run --example name_of_example && python ./misc/model_viewer.py```

In python PyVista and NumPy must be installed 

