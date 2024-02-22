# Grunt
Temporary nothing!
Characters in line hardlimit - 120

## How to use

```
# Import
use grunt::generators::generate_3d;
use grunt::types::generation_params::Params3D;
use grunt::types::*;

# Create paramethers struct first
let mut params = Params3D::new();

# Generate axis or create it from vector
params.set_x_axis(Axis::generate_axis(0.0, 5.0, None));
params.set_y_axis(Axis::generate_axis(0.0, 5.0, None));

# Create layers distribution - (how ideally your model be (now cummulative!!))
params.set_layers_dist(LayersDist::create_from_vec(vec![10, 20, 10]).unwrap());

# Create layers borders (how layers from layers distribution will be modified)
let mut borders = LayersBorder::new();
borders.set_border_deviation(5.0).unwrap();
borders.set_border_max_step(Some(1));
params.set_layers_border(borders);

# Fill model with something
let mut fill = LayersFill::new();
fill.set_values_preset(vec![vec![100], vec![200], vec![300, 320]]).unwrap();
fill.set_is_preset_odreder(true);
params.set_layers_fill(fill);

# Create model and export it to json (you can view exported model using script in misc folder (may be long))
let model = generate_3d(params).expect("crashed");
model.export_model_num("My_best_model", true, true, true);
```
