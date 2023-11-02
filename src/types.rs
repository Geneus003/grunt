#[derive(Debug)]
pub struct Axis {
    start: f32,
    end: f32,
    step: Option<f32>,
    axis: Vec<f32>,
}

#[derive(Debug)]
pub struct DefaultLayersDist {
    layers_num: u32,
    max_layer_size: u32,
    min_layer_size: u32,
    layers_sum: u32,
    layers_dist: Vec<u32>,
}

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
        (0..(((end-start)/step * 1000.0).round() / 1000.0 + 1.0).floor() as i32).map(|num| ((start + num as f32 * step) * 1000.0).round() / 1000.0).collect()
    }

    pub fn get_full_axis(self: &Self) -> (f32, f32, Option<f32>, &Vec<f32>) {
        (self.start, self.end, self.step, &self.axis)
    }

    pub fn get_axis(self: &Self) -> &Vec<f32> {
        &self.axis
    }
}

impl DefaultLayersDist {
    pub fn new() -> DefaultLayersDist {
        DefaultLayersDist {
            layers_num: 3,
            max_layer_size: 100,
            min_layer_size:70,
            layers_sum: 230,
            layers_dist: vec![70, 90, 80],
        }
    }

    pub fn create_from_vec(layers_dist: Vec<u32>) -> Result<DefaultLayersDist, &'static str> {
        if layers_dist.is_empty() {
            return Err("Distibution of layers vec must contain at least one value");
        }

        let (mut min_layer_size, mut max_layer_size, mut layers_sum) = (u32::MAX, 0u32, 0u32);

        for el in &layers_dist {
            if *el < min_layer_size { min_layer_size = *el }
            if *el > max_layer_size { max_layer_size = *el }

            layers_sum = layers_sum.checked_add(*el).ok_or("Problem with calculating sum of layers: u32 overflow")?;
        }

        Ok(DefaultLayersDist {
            layers_num: layers_dist.len() as u32,
            max_layer_size,
            min_layer_size,
            layers_sum,
            layers_dist
        })
    }

    // function params are named by DefaultLayersDist's first letters, e.g. ln - (l)ayers_(n)um
    pub fn generate_layers_dist(ln: Option<u32>, max_ls: Option<u32>, min_ls: Option<u32>, ls: Option<u32>) {
        println!("{:?}, {:?}, {:?}, {:?}", ln, max_ls, min_ls, ls);
        unimplemented!()
    }
}
